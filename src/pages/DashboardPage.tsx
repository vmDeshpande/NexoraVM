import { useEffect, useState } from "react";
import { Button } from "../components/Button";
import { EmptyState } from "../components/EmptyState";
import { StatCard } from "../components/StatCard";
import { StatusBadge } from "../components/StatusBadge";
import { RuntimeDiagnostics } from "../components/RuntimeDiagnostics";
import type { NavigationView } from "../types/navigation";
import type { RuntimeSnapshot } from "../types/runtime";
import { vmService } from "../lib/vmService";
import type { VmDefinition } from "../types/vmConfig";

interface DashboardPageProps {
  onNavigate: (view: NavigationView) => void;
  snapshot: RuntimeSnapshot;
}

export function DashboardPage({ onNavigate, snapshot }: DashboardPageProps) {
  const hasActivity = snapshot.activity.length > 0;
  const [definitions, setDefinitions] = useState<VmDefinition[]>([]);
  const [vmLoadError, setVmLoadError] = useState<string | null>(null);

  useEffect(() => {
    let isCurrent = true;
    vmService
      .listVmDefinitions()
      .then((loadedDefinitions) => {
        if (isCurrent) {
          setDefinitions(loadedDefinitions);
          setVmLoadError(null);
        }
      })
      .catch(() => {
        if (isCurrent) {
          setVmLoadError("VM definitions are unavailable.");
        }
      });

    return () => {
      isCurrent = false;
    };
  }, []);

  const vmCount = definitions.length;
  const stoppedCount = definitions.filter((definition) => definition.status === "stopped").length;
  const runningCount = definitions.filter((definition) => definition.status === "running").length;

  return (
    <div className="dashboard">
      <div className="dashboard__intro">
        <div>
          <p className="section-kicker">Workspace status</p>
          <h2>Overview</h2>
          <p>
            NexoraVM is ready for the first setup steps. Configure a runtime,
            add a virtual machine definition, or connect an AI provider.
          </p>
        </div>
        <StatusBadge runtime={snapshot.runtime} />
      </div>

      <div className="stat-grid">
        <StatCard
          detail={snapshot.runtime.detail}
          label="Runtime status"
          tone={snapshot.runtime.state === "not-configured" ? "neutral" : "info"}
          value={snapshot.runtime.label}
        />
        <StatCard
          detail={
            vmLoadError ??
            `${stoppedCount} stopped · ${runningCount} running`
          }
          label="Virtual machines"
          tone={vmCount > 0 ? "success" : "neutral"}
          value={vmCount.toString()}
        />
        <StatCard
          detail={snapshot.ai.detail}
          label="AI provider / model"
          tone="neutral"
          value={
            snapshot.ai.provider === "Not configured"
              ? "Not configured"
              : snapshot.ai.model
          }
        />
        <StatCard
          detail={snapshot.storage.detail}
          label="Storage"
          tone="neutral"
          value="Not configured"
        />
      </div>

      <RuntimeDiagnostics />

      <div className="dashboard__lower-grid">
        <section className="panel dashboard__activity">
          <div className="panel__header">
            <div>
              <p className="section-kicker">Activity</p>
              <h2>Recent activity</h2>
            </div>
            <span className="panel__meta">No new events</span>
          </div>
          {hasActivity ? (
            <ol className="activity-list">
              {snapshot.activity.map((event) => (
                <li key={event.id} className="activity-list__item">
                  <span className="activity-list__marker" aria-hidden="true" />
                  <div>
                    <strong>{event.label}</strong>
                    <p>{event.description}</p>
                  </div>
                  <time dateTime={event.timestamp}>{event.timestamp}</time>
                </li>
              ))}
            </ol>
          ) : (
            <EmptyState
              description="Events from VM lifecycle, AI configuration, and storage operations will appear here."
              title="No recent activity"
            />
          )}
        </section>

        <section className="panel dashboard__actions">
          <div className="panel__header">
            <div>
              <p className="section-kicker">Get started</p>
              <h2>Set up your workspace</h2>
            </div>
          </div>
          <div className="action-list">
            <Button variant="primary" onClick={() => onNavigate("virtual-machines")}>
              Create Virtual Machine
            </Button>
            <Button onClick={() => onNavigate("ai-workspace")}>
              Configure AI
            </Button>
            <Button onClick={() => onNavigate("settings")}>
              Open Settings
            </Button>
          </div>
          <p className="panel__note">
            VM definitions are persisted locally. Runtime execution and AI inference are planned for upcoming milestones.
          </p>
        </section>
      </div>
    </div>
  );
}
