export type RuntimeState = "starting" | "ready" | "not-configured" | "error";

export interface RuntimeStatus {
  detail: string;
  label: string;
  state: RuntimeState;
}

export type VirtualMachineStatus =
  | "not-configured"
  | "stopped"
  | "starting"
  | "running"
  | "paused"
  | "error";

export interface VirtualMachineStatusSummary {
  count: number;
  detail: string;
  state: VirtualMachineStatus;
}

export type AiProviderStatus = "not-configured" | "connecting" | "ready" | "error";

export interface AiStatusSummary {
  detail: string;
  model: string;
  provider: string;
  state: AiProviderStatus;
}

export type StorageStatus = "not-configured" | "available" | "low-space" | "error";

export interface StorageStatusSummary {
  detail: string;
  state: StorageStatus;
}

export type ActivityEventKind =
  | "runtime"
  | "virtual-machine"
  | "ai"
  | "storage"
  | "settings";

export interface ActivityEventEntry {
  description: string;
  id: string;
  kind: ActivityEventKind;
  label: string;
  timestamp: string;
}

export interface RuntimeSnapshot {
  activity: readonly ActivityEventEntry[];
  ai: AiStatusSummary;
  runtime: RuntimeStatus;
  storage: StorageStatusSummary;
  virtualMachines: VirtualMachineStatusSummary;
}
