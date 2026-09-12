import { useEffect, useState } from "react";
import { Button } from "../components/Button";
import {
  getCommandErrorMessage,
  settingsService,
} from "../lib/settingsService";
import type { AppSettings, DisplayMode } from "../types/settings";

const displayModeOptions: readonly DisplayMode[] = [
  "windowed",
  "fullscreen",
  "headless",
];

const emptySettings: AppSettings = {
  qemuExecutablePath: null,
  defaultVmStoragePath: null,
  defaultIsoPath: null,
  defaultMemoryMiB: 4096,
  defaultCpuCount: 2,
  preferredDisplayMode: "windowed",
  startMinimized: false,
  checkForUpdates: true,
};

export function SettingsPage() {
  const [settings, setSettings] = useState<AppSettings>(emptySettings);
  const [error, setError] = useState<string | null>(null);
  const [feedback, setFeedback] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    let isCurrent = true;

    settingsService
      .getAppSettings()
      .then((loadedSettings) => {
        if (isCurrent) {
          setSettings(loadedSettings);
          setError(null);
        }
      })
      .catch((loadError: unknown) => {
        if (isCurrent) {
          setError(getCommandErrorMessage(loadError));
        }
      })
      .finally(() => {
        if (isCurrent) {
          setLoading(false);
        }
      });

    return () => {
      isCurrent = false;
    };
  }, []);

  const updateField = <Key extends keyof AppSettings>(
    field: Key,
    value: AppSettings[Key],
  ) => {
    setSettings((currentSettings) => ({
      ...currentSettings,
      [field]: value,
    }));
    setFeedback(null);
    setError(null);
  };

  const saveSettings = async () => {
    setSaving(true);
    setError(null);
    setFeedback(null);

    try {
      const savedSettings = await settingsService.saveAppSettings(normalize(settings));
      setSettings(savedSettings);
      setFeedback("Settings saved.");
    } catch (saveError) {
      setError(getCommandErrorMessage(saveError));
    } finally {
      setSaving(false);
    }
  };

  const resetSettings = async () => {
    setSaving(true);
    setError(null);
    setFeedback(null);

    try {
      const defaultSettings = await settingsService.resetAppSettings();
      setSettings(defaultSettings);
      setFeedback("Settings reset to defaults.");
    } catch (resetError) {
      setError(getCommandErrorMessage(resetError));
    } finally {
      setSaving(false);
    }
  };

  const disabled = loading || saving;

  if (loading) {
    return (
      <div className="settings-page">
        <section className="panel">
          <p className="section-kicker">Preferences</p>
          <h2>Loading settings</h2>
          <p className="panel__note">Reading saved application preferences.</p>
        </section>
      </div>
    );
  }

  return (
    <form
      className="settings-page"
      onSubmit={(event) => {
        event.preventDefault();
        void saveSettings();
      }}
    >
      <section className="panel settings-panel">
        <div className="panel__header">
          <div>
            <p className="section-kicker">Runtime paths</p>
            <h2>Application settings</h2>
          </div>
        </div>

        <div className="settings-grid">
          <TextField
            disabled={disabled}
            label="QEMU executable path"
            onChange={(value) => updateField("qemuExecutablePath", value)}
            placeholder="Optional"
            value={settings.qemuExecutablePath ?? ""}
          />
          <TextField
            disabled={disabled}
            label="Default VM storage path"
            onChange={(value) => updateField("defaultVmStoragePath", value)}
            placeholder="Optional"
            value={settings.defaultVmStoragePath ?? ""}
          />
          <TextField
            disabled={disabled}
            label="Default ISO path"
            onChange={(value) => updateField("defaultIsoPath", value)}
            placeholder="Optional"
            value={settings.defaultIsoPath ?? ""}
          />
          <NumberField
            disabled={disabled}
            label="Default memory (MiB)"
            max={262144}
            min={512}
            onChange={(value) => updateField("defaultMemoryMiB", value)}
            value={settings.defaultMemoryMiB}
          />
          <NumberField
            disabled={disabled}
            label="Default CPU count"
            max={128}
            min={1}
            onChange={(value) => updateField("defaultCpuCount", value)}
            value={settings.defaultCpuCount}
          />
          <label className="field">
            <span>Preferred display mode</span>
            <select
              disabled={disabled}
              onChange={(event) =>
                updateField(
                  "preferredDisplayMode",
                  event.target.value as DisplayMode,
                )
              }
              value={settings.preferredDisplayMode}
            >
              {displayModeOptions.map((mode) => (
                <option key={mode} value={mode}>
                  {formatDisplayMode(mode)}
                </option>
              ))}
            </select>
          </label>
        </div>
      </section>

      <section className="panel settings-panel">
        <div className="panel__header">
          <div>
            <p className="section-kicker">Behavior</p>
            <h2>Startup and maintenance</h2>
          </div>
        </div>

        <div className="settings-toggles">
          <CheckboxField
            checked={settings.startMinimized}
            disabled={disabled}
            label="Start minimized"
            onChange={(checked) => updateField("startMinimized", checked)}
          />
          <CheckboxField
            checked={settings.checkForUpdates}
            disabled={disabled}
            label="Check for updates"
            onChange={(checked) => updateField("checkForUpdates", checked)}
          />
        </div>
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

      <div className="settings-actions">
        <Button disabled={disabled} type="submit" variant="primary">
          {saving ? "Saving" : "Save"}
        </Button>
        <Button disabled={disabled} onClick={() => void resetSettings()}>
          Reset
        </Button>
      </div>
    </form>
  );
}

interface TextFieldProps {
  disabled: boolean;
  label: string;
  onChange: (value: string) => void;
  placeholder?: string;
  value: string;
}

function TextField({
  disabled,
  label,
  onChange,
  placeholder,
  value,
}: TextFieldProps) {
  return (
    <label className="field">
      <span>{label}</span>
      <input
        disabled={disabled}
        onChange={(event) => onChange(event.target.value)}
        placeholder={placeholder}
        type="text"
        value={value}
      />
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

function NumberField({
  disabled,
  label,
  max,
  min,
  onChange,
  value,
}: NumberFieldProps) {
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

function CheckboxField({
  checked,
  disabled,
  label,
  onChange,
}: CheckboxFieldProps) {
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

function normalize(settings: AppSettings): AppSettings {
  return {
    ...settings,
    qemuExecutablePath: normalizePath(settings.qemuExecutablePath),
    defaultVmStoragePath: normalizePath(settings.defaultVmStoragePath),
    defaultIsoPath: normalizePath(settings.defaultIsoPath),
  };
}

function normalizePath(value: string | null) {
  const normalizedValue = value?.trim() ?? "";
  return normalizedValue.length > 0 ? normalizedValue : null;
}

function formatDisplayMode(mode: DisplayMode) {
  return mode
    .split(/(?=[A-Z])/)
    .join(" ")
    .replace(/^\w/, (character) => character.toUpperCase());
}
