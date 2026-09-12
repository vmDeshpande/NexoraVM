export type DisplayMode = "windowed" | "fullscreen" | "headless";

export interface AppSettings {
  qemuExecutablePath: string | null;
  defaultVmStoragePath: string | null;
  defaultIsoPath: string | null;
  defaultMemoryMiB: number;
  defaultCpuCount: number;
  preferredDisplayMode: DisplayMode;
  startMinimized: boolean;
  checkForUpdates: boolean;
}

export interface AppCommandError {
  code: string;
  message: string;
  field: string | null;
}
