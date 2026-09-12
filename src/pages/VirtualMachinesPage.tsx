import { useEffect, useState } from "react";
import { Button } from "../components/Button";
import { EmptyState } from "../components/EmptyState";
import { getCommandErrorMessage } from "../lib/settingsService";
import { vmService } from "../lib/vmService";
import type {
  NetworkMode,
  OperatingSystem,
  VmConfiguration,
  VmDefinition,
} from "../types/vmConfig";
import type { DisplayMode } from "../types/settings";

const defaultConfiguration: VmConfiguration = {
  id: "",
  name: "",
  operatingSystem: "linux",
  cpuCount: 2,
  memoryMiB: 4096,
  diskSizeGiB: 64,
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

  const loadDefinitions = async () => {
    try {
      const loadedDefinitions = await vmService.listVmDefinitions();
      setDefinitions(loadedDefinitions);
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
      }
      setFormOpen(false);
      await loadDefinitions();
    } catch (saveError) {
      setError(getCommandErrorMessage(saveError));
    } finally {
      setSaving(false);
    }
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
    } catch (deleteError) {
      setError(getCommandErrorMessage(deleteError));
    } finally {
      setDeletingId(null);
    }
  };

  const controlsDisabled = loading || saving || deletingId !== null;

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
          {definitions.map((definition) => (
            <VmCard
              key={definition.configuration.id}
              definition={definition}
              deleting={deletingId === definition.configuration.id}
              disabled={controlsDisabled}
              onDelete={() => void deleteDefinition(definition)}
              onEdit={() => openEditForm(definition)}
            />
          ))}
        </div>
      )}
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
  onDelete: () => void;
  onEdit: () => void;
}

function VmCard({ definition, deleting, disabled, onDelete, onEdit }: VmCardProps) {
  const { configuration } = definition;
  return (
    <article className="vm-card">
      <div className="vm-card__header">
        <div>
          <p className="section-kicker">{formatLabel(configuration.operatingSystem)}</p>
          <h2>{configuration.name}</h2>
        </div>
        <span className={`vm-status vm-status--${definition.status}`}>
          {formatLabel(definition.status)}
        </span>
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
          <dt>Network</dt>
          <dd>{formatLabel(configuration.networkMode)}</dd>
        </div>
      </dl>
      <p className="vm-card__note">
        {configuration.isoPath ? `ISO: ${configuration.isoPath}` : "No ISO selected"}
      </p>
      <div className="vm-card__actions">
        <Button disabled variant="ghost" title="Runtime integration is not implemented yet.">
          Start
        </Button>
        <Button disabled variant="ghost" title="Runtime integration is not implemented yet.">
          Stop
        </Button>
        <span className="vm-card__action-spacer" />
        <Button disabled={disabled} onClick={onEdit}>
          Edit
        </Button>
        <Button disabled={disabled || deleting} onClick={onDelete}>
          {deleting ? "Deleting" : "Delete"}
        </Button>
      </div>
    </article>
  );
}

function formatLabel(value: string) {
  return value
    .split(/(?=[A-Z])|-/)
    .join(" ")
    .replace(/^\w/, (character) => character.toUpperCase());
}