import type { NavigationItemDefinition, PageMeta, NavigationView } from "../types/navigation";

export const navigationItems: readonly NavigationItemDefinition[] = [
  {
    description: "Review runtime health and next setup steps.",
    icon: "dashboard",
    id: "dashboard",
    label: "Dashboard",
    section: "Overview",
    title: "Overview",
  },
  {
    description: "Create and manage guest operating systems.",
    icon: "virtualMachines",
    id: "virtual-machines",
    label: "Virtual Machines",
    section: "Virtualization",
    title: "Virtual Machines",
  },
  {
    description: "Connect a provider-neutral AI workspace.",
    icon: "aiWorkspace",
    id: "ai-workspace",
    label: "AI Workspace",
    section: "AI",
    title: "AI Workspace",
  },
  {
    description: "Manage disks and storage locations.",
    icon: "storage",
    id: "storage",
    label: "Storage",
    section: "Resources",
    title: "Storage",
  },
  {
    description: "Configure application behavior and preferences.",
    icon: "settings",
    id: "settings",
    label: "Settings",
    section: "Application",
    title: "Settings",
  },
];

export function getPageMeta(view: NavigationView): PageMeta {
  const page = navigationItems.find((item) => item.id === view);

  if (!page) {
    return {
      description: "This application area is not available.",
      section: "Application",
      title: "Unavailable",
    };
  }

  return {
    description: page.description,
    section: page.section,
    title: page.title,
  };
}
