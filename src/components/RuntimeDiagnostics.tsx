import { useEffect, useState } from "react";
import { Button } from "./Button";
import { getCommandErrorMessage } from "../lib/settingsService";
import { runtimeService } from "../lib/runtimeService";
import type { CapabilityState, RuntimeStatus } from "../types/runtimeStatus";

interface RuntimeDiagnosticsProps {
  compact?: boolean;
}

export function RuntimeDiagnostics({ compact = false }: RuntimeDiagnosticsProps) {
  const [status, setStatus] = useState<RuntimeStatus | null>(null);
  const [loading, setLoading] = useState(true);
  const [refreshing, setRefreshing] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const refresh = async () => {
    setRefreshing(true);
    setError(null);
    try {
      setStatus(await runtimeService.refreshRuntimeStatus());
    } catch (refreshError) {
      setError(getCommandErrorMessage(refreshError));
    } finally {
      setLoading(false);
      setRefreshing(false);
    }
  };

  useEffect(() => {
    let isCurrent = true;
    runtimeService
      .getRuntimeStatus()
      .then((loadedStatus) => {
        if (isCurrent) {
          setStatus(loadedStatus);
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

  return (
    <section className={`panel runtime-diagnostics ${compact ? "runtime-diagnostics--compact" : ""}`}>
      <div className="panel__header">
        <div>
          <p className="section-kicker">Runtime diagnostics</p>
          <h2>QEMU and Windows capabilities</h2>
        </div>
        <Button disabled={loading || refreshing} onClick={() => void refresh()}>
          {refreshing ? "Checking" : "Refresh"}
        </Button>
      </div>

      {loading ? (
        <p className="panel__note">Checking configured runtime and host capabilities.</p>
      ) : error ? (
        <div className="settings-message settings-message--error" role="status">
          {error}
        </div>
      ) : status ? (
        <>
          <div className="runtime-summary">
            <StatusRow label="QEMU availability" status={status.availability} />
            <StatusRow label="WHPX" status={status.whpx.state} detail={status.whpx.detail} />
            <StatusRow
              label="CPU virtualization"
              status={status.virtualization.state}
              detail={status.virtualization.detail}
            />
          </div>
          <dl className="runtime-details">
            <div>
              <dt>Configured path</dt>
              <dd>{status.configuredQemuPath ?? "Not configured"}</dd>
            </div>
            <div>
              <dt>Detected path</dt>
              <dd>{status.detectedQemuPath ?? "Not detected"}</dd>
            </div>
            <div>
              <dt>QEMU version</dt>
              <dd>{status.qemuVersion ?? "Unavailable"}</dd>
            </div>
            <div>
              <dt>Last checked</dt>
              <dd>{status.lastChecked}</dd>
            </div>
          </dl>
          {status.diagnostics.length > 0 ? (
            <ul className="runtime-diagnostics__list">
              {status.diagnostics.map((diagnostic) => (
                <li key={`${diagnostic.code}-${diagnostic.message}`}>{diagnostic.message}</li>
              ))}
            </ul>
          ) : null}
        </>
      ) : null}
    </section>
  );
}

interface StatusRowProps {
  detail?: string;
  label: string;
  status: CapabilityState;
}

function StatusRow({ detail, label, status }: StatusRowProps) {
  return (
    <div className="runtime-summary__item" title={detail}>
      <span>{label}</span>
      <strong className={`runtime-state runtime-state--${status}`}>{formatState(status)}</strong>
    </div>
  );
}

function formatState(state: CapabilityState) {
  return state.replace(/-/g, " ").replace(/^\w/, (character: string) => character.toUpperCase());
}