export type NavigationView =
  | "dashboard"
  | "virtual-machines"
  | "ai-workspace"
  | "storage"
  | "settings";

export type NavigationIcon =
  | "dashboard"
  | "virtualMachines"
  | "aiWorkspace"
  | "storage"
  | "settings";

export interface NavigationItemDefinition {
  description: string;
  icon: NavigationIcon;
  id: NavigationView;
  label: string;
  section: string;
  title: string;
}

export interface PageMeta {
  description: string;
  section: string;
  title: string;
}
