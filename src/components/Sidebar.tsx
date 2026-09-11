import { NavigationItem } from "./NavigationItem";
import type { NavigationView } from "../types/navigation";
import { navigationItems } from "../lib/navigation";

interface SidebarProps {
  activeView: NavigationView;
  onNavigate: (view: NavigationView) => void;
}

export function Sidebar({ activeView, onNavigate }: SidebarProps) {
  return (
    <aside className="sidebar">
      <div className="brand" aria-label="NexoraVM">
        <div className="brand__mark" aria-hidden="true">
          N
        </div>
        <div className="brand__copy">
          <strong>NexoraVM</strong>
          <span>Virtualization workspace</span>
        </div>
      </div>

      <nav className="sidebar__navigation" aria-label="Primary navigation">
        {navigationItems.map((item) => (
          <NavigationItem
            key={item.id}
            active={activeView === item.id}
            icon={item.icon}
            label={item.label}
            onNavigate={onNavigate}
            view={item.id}
          />
        ))}
      </nav>

      <div className="sidebar__footer">
        <span className="status-dot status-dot--neutral" aria-hidden="true" />
        <span>Local shell</span>
        <span className="sidebar__version">v0.1.0</span>
      </div>
    </aside>
  );
}
