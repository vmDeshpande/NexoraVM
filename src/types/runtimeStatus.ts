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