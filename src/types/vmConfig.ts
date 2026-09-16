import type { DisplayMode } from "./settings";

import type { QemuBootMode } from "./runtimeStatus";

export type OperatingSystem = "windows" | "linux" | "bsd" | "other";
export type NetworkMode = "disabled" | "user" | "bridged";
export interface VmConfiguration {
  id: string;
  name: string;
  operatingSystem: OperatingSystem;
  cpuCount: number;
  memoryMiB: number;
  diskSizeGiB: number;
  diskPath: string | null;
  isoPath: string;
  networkMode: NetworkMode;
  displayMode: DisplayMode;
  secureBootEnabled: boolean;
  tpmEnabled: boolean;
  bootMode: QemuBootMode;
}

export interface VmDefinition {
  configuration: VmConfiguration;
}
