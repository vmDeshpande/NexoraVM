import { invoke } from "@tauri-apps/api/core";
import type { AppCommandError, AppSettings } from "../types/settings";

export interface SettingsService {
  getAppSettings: () => Promise<AppSettings>;
  saveAppSettings: (settings: AppSettings) => Promise<AppSettings>;
  resetAppSettings: () => Promise<AppSettings>;
}

export const settingsService: SettingsService = {
  getAppSettings: () => invoke<AppSettings>("get_app_settings"),
  saveAppSettings: (settings) =>
    invoke<AppSettings>("save_app_settings", { settings }),
  resetAppSettings: () => invoke<AppSettings>("reset_app_settings"),
};

export function getCommandErrorMessage(error: unknown) {
  if (isAppCommandError(error)) {
    return error.message;
  }

  if (error instanceof Error) {
    return error.message;
  }

  return "The settings operation could not be completed.";
}

function isAppCommandError(error: unknown): error is AppCommandError {
  if (!error || typeof error !== "object") {
    return false;
  }

  const candidate = error as Partial<AppCommandError>;
  return typeof candidate.message === "string";
}
