import { useEffect, useState } from "react";
import { Button } from "../components/Button";
import { EmptyState } from "../components/EmptyState";
import { getCommandErrorMessage } from "../lib/settingsService";
import { vmService } from "../lib/vmService";
import type { NetworkMode, OperatingSystem, VmConfiguration, VmDefinition } from "../types/vmConfig";
import type { DisplayMode } from "../types/settings";
import type {
  QemuBootMode,
  QemuCommandSpec,
  QemuProcessState,
  QemuProcessStatus,
  VmDiskStatus,
} from "../types/runtimeStatus";

const defaultConfiguration: VmConfiguration = {
  id: "",
  name: "",
  operatingSystem: "linux",
  cpuCount: 2,
  memoryMiB: 4096,
  diskSizeGiB: 64,
  diskPath: null,
  isoPath: "",
  networkMode: "user",
  displayMode: "windowed",
  secureBootEnabled: false,
  tpmEnabled: false,
};

const operatingSystems: readonly OperatingSystem[] = [
  "windows",
  "linux",
  "bsd",
  "other",
];
const networkModes: readonly NetworkMode[] = ["disabled", "user", "bridged"];
const displayModes: readonly DisplayMode[] = [
  "windowed",
  "fullscreen",
  "headless",
];

export function VirtualMachinesPage() {
  const [definitions, setDefinitions] = useState<VmDefinition[]>([]);
  const [configuration, setConfiguration] = useState(defaultConfiguration);
  const [editingId, setEditingId] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [deletingId, setDeletingId] = useState<string | null>(null);
  const [formOpen, setFormOpen] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [feedback, setFeedback] = useState<string | null>(null);
  const [preview, setPreview] = useState<QemuCommandSpec | null>(null);
  const [previewingId, setPreviewingId] = useState<string | null>(null);
  const [processStatuses, setProcessStatuses] = useState<
    Record<string, QemuProcessStatus>
  >({});
  const [processActionId, setProcessActionId] = useState<string | null>(null);
  const [diskStatuses, setDiskStatuses] = useState<Record<string, VmDiskStatus>>({});
  const [bootModes, setBootModes] = useState<Record<string, QemuBootMode>>({});

  const loadDefinitions = async () => {
    try {
      const loadedDefinitions = await vmService.listVmDefinitions();
      setDefinitions(loadedDefinitions);
      const statuses = await vmService.refreshAllVmProcessStatuses();
      setProcessStatuses(
        Object.fromEntries(statuses.map((status) => [status.vmId, status])),
      );
      setError(null);
    } catch (loadError) {
      setError(getCommandErrorMessage(loadError));
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void loadDefinitions();
  }, []);

  const loadDiskStatuses = async () => {
    try {
      const definitions = await vmService.listVmDefinitions();
      const statuses = await Promise.all(
        definitions.map(async (definition) => ({
          vmId: definition.configuration.id,
          status: await vmService.getVmDiskStatus(definition.configuration.id),
        })),
      );
      const next: Record<string, VmDiskStatus> = {};
      for (const { vmId, status } of statuses) {
        next[vmId] = status;
      }
      setDiskStatuses(next);
    } catch {
      // The list load or disk-status failure is surfaced elsewhere.
    }
  };

  const loadDiskStatusesForCurrentView = async () => {
    await loadDiskStatuses();
  };

  useEffect(() => {
    void loadDiskStatusesForCurrentView();
  }, [definitions]);

  const openCreateForm = () => {
    setConfiguration({ ...defaultConfiguration });
    setEditingId(null);
    setFormOpen(true);
    setFeedback(null);
    setError(null);
  };

  const openEditForm = (definition: VmDefinition) => {
    setConfiguration({ ...definition.configuration });
    setEditingId(definition.configuration.id);
    setFormOpen(true);
    setFeedback(null);
    setError(null);
  };

  const saveDefinition = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setSaving(true);
    setFeedback(null);
    setError(null);

    try {
      if (editingId) {
        await vmService.updateVmDefinition(configuration);
        setFeedback("Virtual machine updated.");
      } else {
        await vmService.createVmDefinition(configuration);
        setFeedback("Virtual machine created.");
        setBootModes((current) => ({
          ...current,
          [configuration.id]: "normal",
        }));
        if (configuration.diskPath) {
          try {
            await vmService.createVmDisk(configuration.id);
          } catch {
            // Disk creation failures are surfaced when reviewing the VM card.
          }
        }
      }
      setFormOpen(false);
      await loadDefinitions();
      await loadDiskStatuses();
    } catch (saveError) {
      setError(getCommandErrorMessage(saveError));
    } finally {
      setSaving(false);
    }
  };

  const createDisk = async (vmId: string) => {
    setProcessActionId(vmId);
    setError(null);
    try {
      const status = await vmService.createVmDisk(vmId);
      setDiskStatuses((current) => ({ ...current, [vmId]: status }));
      if (status.state === "creation-failed") {
        setError(status.message ?? "Disk creation failed.");
      } else {
        setFeedback("Disk created.");
      }
    } catch (createError) {
      setError(getCommandErrorMessage(createError));
    } finally {
      setProcessActionId(null);
    }
  };

  const startInBootMode = async (vmId: string, bootMode: QemuBootMode) => {
    setProcessActionId(vmId);
    setError(null);
    try {
      const status = await vmService.startVm({ vmId, bootMode });
      setProcessStatuses((current) => ({ ...current, [vmId]: status }));
      if (bootMode === "install") {
        setFeedback(`Installing from ISO: ${status.state}`);
      }
    } catch (startError) {
      setError(getCommandErrorMessage(startError));
      await refreshProcessStatus(vmId);
    } finally {
      setProcessActionId(null);
    }
  };

  const previewCommand = async (vmId: string, bootMode: QemuBootMode) => {
    setPreviewingId(vmId);
    setPreview(null);
    setError(null);
    try {
      setPreview(await vmService.buildQemuCommandSpec({ vmId, bootMode }));
    } catch (previewError) {
      setError(getCommandErrorMessage(previewError));
    } finally {
      setPreviewingId(null);
    }
  };

  const setBootMode = (vmId: string, mode: QemuBootMode) => {
    setBootModes((current) => ({ ...current, [vmId]: mode }));
  };

  const deleteDefinition = async (definition: VmDefinition) => {
    const confirmed = window.confirm(
      `Delete the virtual machine definition “${definition.configuration.name}”?`,
    );
    if (!confirmed) {
      return;
    }

    setDeletingId(definition.configuration.id);
    setError(null);
    setFeedback(null);
    try {
      await vmService.deleteVmDefinition(definition.configuration.id);
      setFeedback("Virtual machine deleted.");
      await loadDefinitions();
      setDiskStatuses((current) => {
        const next = { ...current };
        delete next[definition.configuration.id];
        return next;
      });
    } catch (deleteError) {
      setError(getCommandErrorMessage(deleteError));
    } finally {
      setDeletingId(null);
    }
  };

  const startProcess = async (vmId: string) => {
    setProcessActionId(vmId);
    setError(null);
    try {
      const status = await vmService.startVm({
        vmId,
        bootMode: bootModes[vmId] ?? "normal",
      });
      setProcessStatuses((current) => ({ ...current, [vmId]: status }));
    } catch (startError) {
      setError(getCommandErrorMessage(startError));
      await refreshProcessStatus(vmId);
    } finally {
      setProcessActionId(null);
    }
  };

  const stopProcess = async (vmId: string) => {
    setProcessActionId(vmId);
    setError(null);
    try {
      const status = await vmService.stopVm(vmId);
      setProcessStatuses((current) => ({ ...current, [vmId]: status }));
    } catch (stopError) {
      setError(getCommandErrorMessage(stopError));
      await refreshProcessStatus(vmId);
    } finally {
      setProcessActionId(null);
    }
  };

  const refreshProcessStatus = async (vmId: string) => {
    try {
      const status = await vmService.getVmProcessStatus(vmId);
      setProcessStatuses((current) => ({ ...current, [vmId]: status }));
    } catch {
      // The original command error remains the user-facing diagnostic.
    }
  };

  const controlsDisabled =
    loading || saving || deletingId !== null || processActionId !== null;

  return (
    <div className="virtual-machines-page">
      <section className="panel vm-page-header">
        <div>
          <p className="section-kicker">Virtualization</p>
          <h2>Virtual machines</h2>
          <p className="panel__note">
            Saved definitions are ready for a future QEMU or WHPX runtime.
          </p>
        </div>
        <Button disabled={controlsDisabled} onClick={openCreateForm} variant="primary">
          Create Virtual Machine
        </Button>
      </section>

      {(error || feedback) && (
        <div
          className={`settings-message ${
            error ? "settings-message--error" : "settings-message--success"
          }`}
          role="status"
        >
          {error ?? feedback}
        </div>
      )}

      {formOpen ? (
        <VmDefinitionForm
          configuration={configuration}
          disabled={saving}
          editing={editingId !== null}
          onCancel={() => setFormOpen(false)}
          onChange={setConfiguration}
          onSubmit={saveDefinition}
        />
      ) : null}

      {loading ? (
        <section className="panel">
          <p className="section-kicker">Definitions</p>
          <h2>Loading virtual machines</h2>
          <p className="panel__note">Reading saved VM definitions.</p>
        </section>
      ) : definitions.length === 0 && !formOpen ? (
        <EmptyState
          actionDisabled={controlsDisabled}
          actionLabel="Create Virtual Machine"
          description="Create a definition to prepare a guest configuration. Runtime execution is not available yet."
          onAction={openCreateForm}
          title="No virtual machines yet"
        />
      ) : (
        <div className="vm-list">
          {definitions.map((definition) => {
            const bootMode = bootModes[definition.configuration.id] ?? "normal";
            return (
              <VmCard
                key={definition.configuration.id}
                definition={definition}
                deleting={deletingId === definition.configuration.id}
                disabled={controlsDisabled}
                diskStatus={diskStatuses[definition.configuration.id]}
                bootMode={bootMode}
                onDelete={() => void deleteDefinition(definition)}
                onEdit={() => openEditForm(definition)}
                onPreview={() => void previewCommand(definition.configuration.id, bootMode)}
                previewing={previewingId === definition.configuration.id}
                processStatus={
                  processStatuses[definition.configuration.id] ?? {
                    vmId: definition.configuration.id,
                    state: "stopped",
                    processId: null,
                    exitCode: null,
                    terminationReason: null,
                    output: { stdout: "", stderr: "", truncated: false },
                  }
                }
                onInstall={() => {
                  setBootMode(definition.configuration.id, "install");
                  void startInBootMode(definition.configuration.id, "install");
                }}
                onStart={() => void startProcess(definition.configuration.id)}
                onStop={() => void stopProcess(definition.configuration.id)}
                onCreateDisk={() => void createDisk(definition.configuration.id)}
                processAction={processActionId === definition.configuration.id}
              />
            );
          })}
        </div>
      )}
      {preview ? <CommandPreview spec={preview} /> : null}
    </div>
  );
}

interface VmDefinitionFormProps {
  configuration: VmConfiguration;
  disabled: boolean;
  editing: boolean;
  onCancel: () => void;
  onChange: (configuration: VmConfiguration) => void;
  onSubmit: (event: React.FormEvent<HTMLFormElement>) => void;
}

function VmDefinitionForm({
  configuration,
  disabled,
  editing,
  onCancel,
  onChange,
  onSubmit,
}: VmDefinitionFormProps) {
  const update = <Key extends keyof VmConfiguration>(
    field: Key,
    value: VmConfiguration[Key],
  ) => onChange({ ...configuration, [field]: value });

  return (
    <form className="panel vm-form" onSubmit={onSubmit}>
      <div className="panel__header">
        <div>
          <p className="section-kicker">Definition</p>
          <h2>{editing ? "Edit virtual machine" : "Create virtual machine"}</h2>
        </div>
      </div>
      <div className="settings-grid">
        <label className="field field--wide">
          <span>VM name</span>
          <input
            autoFocus
            disabled={disabled}
            maxLength={100}
            onChange={(event) => update("name", event.target.value)}
            required
            value={configuration.name}
          />
        </label>
        <SelectField
          disabled={disabled}
          label="Operating system"
          onChange={(value) => update("operatingSystem", value as OperatingSystem)}
          options={operatingSystems}
          value={configuration.operatingSystem}
        />
        <NumberField
          disabled={disabled}
          label="CPU count"
          max={128}
          min={1}
          onChange={(value) => update("cpuCount", value)}
          value={configuration.cpuCount}
        />
        <NumberField
          disabled={disabled}
          label="Memory (MiB)"
          max={262144}
          min={512}
          onChange={(value) => update("memoryMiB", value)}
          value={configuration.memoryMiB}
        />
        <NumberField
          disabled={disabled}
          label="Disk size (GiB)"
          max={1048576}
          min={1}
          onChange={(value) => update("diskSizeGiB", value)}
          value={configuration.diskSizeGiB}
        />
        <label className="field field--wide">
          <span>Virtual disk path (optional)</span>
          <input
            disabled={disabled}
            onChange={(event) =>
              update("diskPath", event.target.value.trim() || null)
            }
            placeholder="Optional; no disk is created"
            value={configuration.diskPath ?? ""}
          />
        </label>
        <label className="field field--wide">
          <span>ISO path (optional)</span>
          <input
            disabled={disabled}
            onChange={(event) => update("isoPath", event.target.value)}
            placeholder="Optional"
            value={configuration.isoPath}
          />
        </label>
        <SelectField
          disabled={disabled}
          label="Network mode"
          onChange={(value) => update("networkMode", value as NetworkMode)}
          options={networkModes}
          value={configuration.networkMode}
        />
        <SelectField
          disabled={disabled}
          label="Display mode"
          onChange={(value) => update("displayMode", value as DisplayMode)}
          options={displayModes}
          value={configuration.displayMode}
        />
      </div>
      <div className="settings-toggles">
        <CheckboxField
          checked={configuration.secureBootEnabled}
          disabled={disabled}
          label="Secure Boot"
          onChange={(checked) => update("secureBootEnabled", checked)}
        />
        <CheckboxField
          checked={configuration.tpmEnabled}
          disabled={disabled}
          label="TPM"
          onChange={(checked) => update("tpmEnabled", checked)}
        />
      </div>
      <div className="settings-actions">
        <Button disabled={disabled} type="submit" variant="primary">
          {disabled ? "Saving" : editing ? "Save Changes" : "Create VM"}
        </Button>
        <Button disabled={disabled} onClick={onCancel}>
          Cancel
        </Button>
      </div>
    </form>
  );
}

interface SelectFieldProps {
  disabled: boolean;
  label: string;
  onChange: (value: string) => void;
  options: readonly string[];
  value: string;
}

function SelectField({ disabled, label, onChange, options, value }: SelectFieldProps) {
  return (
    <label className="field">
      <span>{label}</span>
      <select
        disabled={disabled}
        onChange={(event) => onChange(event.target.value)}
        value={value}
      >
        {options.map((option) => (
          <option key={option} value={option}>
            {formatLabel(option)}
          </option>
        ))}
      </select>
    </label>
  );
}

interface NumberFieldProps {
  disabled: boolean;
  label: string;
  max: number;
  min: number;
  onChange: (value: number) => void;
  value: number;
}

function NumberField({ disabled, label, max, min, onChange, value }: NumberFieldProps) {
  return (
    <label className="field">
      <span>{label}</span>
      <input
        disabled={disabled}
        max={max}
        min={min}
        onChange={(event) => {
          const nextValue = event.target.valueAsNumber;
          onChange(Number.isNaN(nextValue) ? min : nextValue);
        }}
        required
        type="number"
        value={value}
      />
    </label>
  );
}

interface CheckboxFieldProps {
  checked: boolean;
  disabled: boolean;
  label: string;
  onChange: (checked: boolean) => void;
}

function CheckboxField({ checked, disabled, label, onChange }: CheckboxFieldProps) {
  return (
    <label className="toggle-field">
      <input
        checked={checked}
        disabled={disabled}
        onChange={(event) => onChange(event.target.checked)}
        type="checkbox"
      />
      <span>{label}</span>
    </label>
  );
}

interface VmCardProps {
  definition: VmDefinition;
  deleting: boolean;
  disabled: boolean;
  diskStatus?: VmDiskStatus | undefined;
  bootMode?: QemuBootMode;
  onDelete: () => void;
  onEdit: () => void;
  onPreview: () => void;
  previewing: boolean;
  onInstall: () => void;
  onStart: () => void;
  onStop: () => void;
  onCreateDisk: () => void;
  processAction: boolean;
  processStatus: QemuProcessStatus;
}

function VmCard({
  definition,
  deleting,
  disabled,
  diskStatus,
  bootMode = "normal",
  onDelete,
  onEdit,
  onPreview,
  previewing,
  onInstall,
  onStart,
  onStop,
  onCreateDisk,
  processAction,
  processStatus,
}: VmCardProps) {
  const { configuration } = definition;
  const diskLabel = diskStatus ? formatDiskState(diskStatus) : "Disk status unavailable";
  const bootLabel = bootMode === "install" ? "Installation" : "Normal boot";
  return (
    <article className="vm-card">
      <div className="vm-card__header">
        <div>
          <p className="section-kicker">{formatLabel(configuration.operatingSystem)}</p>
          <h2>{configuration.name}</h2>
        </div>
        <div className="vm-card__meta">
          <span className={`vm-status vm-status--${processStatus.state}`}>
            {formatLabel(processStatus.state)}
          </span>
          <span className="vm-card__boot">{bootLabel}</span>
        </div>
      </div>
      <dl className="vm-card__details">
        <div>
          <dt>CPU</dt>
          <dd>{configuration.cpuCount} cores</dd>
        </div>
        <div>
          <dt>Memory</dt>
          <dd>{configuration.memoryMiB.toLocaleString()} MiB</dd>
        </div>
        <div>
          <dt>Disk</dt>
          <dd>{configuration.diskSizeGiB.toLocaleString()} GiB</dd>
        </div>
        <div>
          <dt>Disk status</dt>
          <dd>{diskLabel}</dd>
        </div>
        <div>
          <dt>Network</dt>
          <dd>{formatLabel(configuration.networkMode)}</dd>
        </div>
      </dl>
      <p className="vm-card__note">
        {configuration.isoPath ? `ISO: ${configuration.isoPath}` : "No ISO selected"}
      </p>
      {diskStatus?.path ? (
        <p className="vm-card__note">Disk: {diskStatus.path}</p>
      ) : null}
      {processStatus.exitCode !== null || processStatus.terminationReason ? (
        <p className="vm-card__note">
          {processStatus.exitCode !== null
            ? `Exit code: ${processStatus.exitCode}`
            : "No exit code"}
          {processStatus.terminationReason
            ? ` · ${formatLabel(processStatus.terminationReason)}`
            : ""}
        </p>
      ) : null}
      <div className="vm-card__actions">
        <Button
          disabled={disabled || processAction || !canStart(processStatus.state)}
          onClick={onStart}
          variant="ghost"
        >
          {processAction && processStatus.state === "starting" ? "Starting" : "Start"}
        </Button>
        <Button
          disabled={
            disabled || processAction || processStatus.state !== "running"
          }
          onClick={onInstall}
          variant="ghost"
        >
          Install
        </Button>
        <Button
          disabled={disabled || processAction || !canStop(processStatus.state)}
          onClick={onStop}
          variant="ghost"
        >
          {processAction && processStatus.state === "stopping" ? "Stopping" : "Stop"}
        </Button>
        <Button disabled={disabled} onClick={onCreateDisk} variant="ghost">
          {diskStatus?.state === "ready" ? "Disk ready" : "Create disk"}
        </Button>
        <span className="vm-card__action-spacer" />
        <Button disabled={disabled} onClick={onEdit}>
          Edit
        </Button>
        <Button disabled={disabled || previewing} onClick={onPreview}>
          {previewing ? "Checking" : "Preview QEMU configuration"}
        </Button>
        <Button disabled={disabled || deleting} onClick={onDelete}>
          {deleting ? "Deleting" : "Delete"}
        </Button>
      </div>
    </article>
  );
}

function formatDiskState(status: VmDiskStatus) {
  return formatLabel(status.state);
}

function formatLabel(value: string) {
  return value
    .split(/(?=[A-Z])|-/)
    .join(" ")
    .replace(/^\w/, (character) => character.toUpperCase());
}

interface CommandPreviewProps {
  spec: QemuCommandSpec;
}

function CommandPreview({ spec }: CommandPreviewProps) {
  return (
    <section className="panel command-preview">
      <div className="panel__header">
        <div>
          <p className="section-kicker">Diagnostic preview</p>
          <h2>QEMU command specification</h2>
        </div>
        <span className="panel__meta">Not executed</span>
      </div>
      <dl className="runtime-details">
        <div>
          <dt>VM ID</dt>
          <dd>{spec.vmId}</dd>
        </div>
        <div>
          <dt>Boot mode</dt>
          <dd>{formatLabel(spec.bootMode)}</dd>
        </div>
        <div>
          <dt>Acceleration</dt>
          <dd>{formatLabel(spec.acceleration)}</dd>
        </div>
        <div>
          <dt>Display</dt>
          <dd>{formatLabel(spec.displayMode)}</dd>
        </div>
        <div>
          <dt>Network</dt>
          <dd>{formatLabel(spec.networkMode)}</dd>
        </div>
      </dl>
      <p className="panel__note">
        Arguments are shown as separate values. This preview does not launch QEMU or change the host.
      </p>
      <pre className="command-preview__arguments">{JSON.stringify(spec.arguments, null, 2)}</pre>
      {spec.diagnostics.length > 0 ? (
        <ul className="runtime-diagnostics__list">
          {spec.diagnostics.map((diagnostic) => (
            <li key={`${diagnostic.code}-${diagnostic.message}`}>{diagnostic.message}</li>
          ))}
        </ul>
      ) : null}
    </section>
  );
}

function canStart(state: QemuProcessState) {
  return state === "stopped" || state === "not-started" || state === "failed";
}

function canStop(state: QemuProcessState) {
  return state === "starting" || state === "running";
}