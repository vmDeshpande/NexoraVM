export type RuntimeType = "qemu";

export type CapabilityState =
  | "available"
  | "unavailable"
  | "not-checked"
  | "unsupported"
  | "error"
  | "unknown";

export interface CapabilityStatus {
  state: CapabilityState;
  detail: string;
}

export interface RuntimeDiagnostic {
  code: string;
  message: string;
}

export interface RuntimeStatus {
  runtimeType: RuntimeType;
  availability: CapabilityState;
  configuredQemuPath: string | null;
  detectedQemuPath: string | null;
  qemuVersion: string | null;
  whpx: CapabilityStatus;
  virtualization: CapabilityStatus;
  diagnostics: RuntimeDiagnostic[];
  lastChecked: string;
}

export type QemuAcceleration = "whpx" | "tcg";
export type QemuDisplayMode = "sdl" | "gtk-fullscreen" | "none";
export type QemuNetworkMode = "none" | "user";

export interface QemuCommandSpec {
  executablePath: string;
  arguments: string[];
  workingDirectory: string | null;
  vmId: string;
  diagnostics: RuntimeDiagnostic[];
  acceleration: QemuAcceleration;
  displayMode: QemuDisplayMode;
  networkMode: QemuNetworkMode;
}

export type QemuProcessState =
  | "not-started"
  | "starting"
  | "running"
  | "stopping"
  | "stopped"
  | "failed"
  | "timed-out"
  | "cancelled";

export type QemuTerminationReason =
  | "graceful"
  | "forced"
  | "unexpected-exit"
  | "failed-to-start"
  | "timed-out"
  | "cancelled";

export interface QemuProcessOutput {
  stdout: string;
  stderr: string;
  truncated: boolean;
}

export interface QemuProcessStatus {
  vmId: string;
  state: QemuProcessState;
  processId: number | null;
  exitCode: number | null;
  terminationReason: QemuTerminationReason | null;
  output: QemuProcessOutput;
}