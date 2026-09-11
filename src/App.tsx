import { useEffect, useState } from "react";
import { AppShell } from "./layouts/AppShell";
import { DashboardPage } from "./pages/DashboardPage";
import { PlaceholderPage } from "./pages/PlaceholderPage";
import { getPageMeta } from "./lib/navigation";
import { useRuntimeSnapshot } from "./hooks/useRuntimeSnapshot";
import { placeholderRuntimeSnapshot } from "./features/dashboard/placeholderRuntime";
import type { NavigationView } from "./types/navigation";
import type { RuntimeSnapshot } from "./types/runtime";
import "./App.css";

const placeholderDescriptions: Record<NavigationView, string> = {
  dashboard: "Review runtime health and next setup steps.",
  "virtual-machines": "Virtual machine creation and lifecycle controls are coming soon.",
  "ai-workspace": "Provider-neutral AI configuration and inference are coming soon.",
  storage: "Storage locations and disk management are coming soon.",
  settings: "Application preferences and runtime settings are coming soon.",
};

function getRuntimeSnapshot(state: {
  error: string | null;
  snapshot: RuntimeSnapshot | null;
}) {
  if (state.snapshot) {
    return state.snapshot;
  }

  return {
    ...placeholderRuntimeSnapshot,
    runtime: {
      ...placeholderRuntimeSnapshot.runtime,
      detail: state.error ?? placeholderRuntimeSnapshot.runtime.detail,
      label: state.error ? "Unavailable" : placeholderRuntimeSnapshot.runtime.label,
      state: state.error ? "error" : placeholderRuntimeSnapshot.runtime.state,
    },
  } satisfies RuntimeSnapshot;
}

function App() {
  const [activeView, setActiveView] = useState<NavigationView>("dashboard");
  const runtimeState = useRuntimeSnapshot();
  const snapshot = getRuntimeSnapshot(runtimeState);
  const page = getPageMeta(activeView);

  useEffect(() => {
    document.title = `NexoraVM — ${page.title}`;
  }, [page.title]);

  return (
    <AppShell
      activeView={activeView}
      onNavigate={setActiveView}
      page={page}
      runtime={snapshot.runtime}
    >
      {activeView === "dashboard" ? (
        <DashboardPage onNavigate={setActiveView} snapshot={snapshot} />
      ) : (
        <PlaceholderPage
          description={placeholderDescriptions[activeView]}
          onNavigate={setActiveView}
          returnView="dashboard"
          title={`${page.title} is coming soon`}
        />
      )}
    </AppShell>
  );
}

export default App;
