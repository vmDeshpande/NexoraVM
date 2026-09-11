import { Header } from "../components/Header";
import { PageContainer } from "../components/PageContainer";
import { Sidebar } from "../components/Sidebar";
import type { PageMeta } from "../types/navigation";
import type { NavigationView } from "../types/navigation";
import type { RuntimeStatus } from "../types/runtime";

interface AppShellProps {
  activeView: NavigationView;
  children: React.ReactNode;
  onNavigate: (view: NavigationView) => void;
  page: PageMeta;
  runtime: RuntimeStatus;
}

export function AppShell({
  activeView,
  children,
  onNavigate,
  page,
  runtime,
}: AppShellProps) {
  return (
    <div className="app-shell">
      <Sidebar activeView={activeView} onNavigate={onNavigate} />
      <div className="app-main">
        <Header page={page} runtime={runtime} />
        <div className="app-content">
          <PageContainer>{children}</PageContainer>
        </div>
      </div>
    </div>
  );
}
