import { invoke } from "@tauri-apps/api/core";
import type { AppCommandError, AppSettings } from "../types/settings";

export interface CommandErrorInfo {
  operation: string;
  code: string | null;
  field: string | null;
  message: string;
}

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

export function getCommandErrorMessage(
  error: unknown,
  operation = "The operation",
) {
  const info = describeCommandError(error, operation);
  if (info.code && info.field) {
    return `${info.operation}: ${info.code} (${info.field}): ${info.message}`;
  }
  if (info.code) {
    return `${info.operation}: ${info.code}: ${info.message}`;
  }
  if (info.field) {
    return `${info.operation}: ${info.field}: ${info.message}`;
  }
  return `${info.operation} failed: ${info.message}`;
}

export function describeCommandError(
  error: unknown,
  operation = "The operation",
): CommandErrorInfo {
  if (isAppCommandError(error)) {
    return {
      operation,
      code: error.code,
      field: error.field,
      message: error.message,
    };
  }

  if (error instanceof Error) {
    return {
      operation,
      code: null,
      field: null,
      message: error.message,
    };
  }

  if (error && typeof error === "object") {
    const candidate = error as Record<string, unknown>;
    const message =
      typeof candidate.message === "string"
        ? candidate.message
        : typeof candidate.error === "string"
          ? candidate.error
          : JSON.stringify(candidate);
    const code = typeof candidate.code === "string" ? candidate.code : null;
    const field = typeof candidate.field === "string" ? candidate.field : null;
    return {
      operation,
      code,
      field,
      message,
    };
  }

  return {
    operation,
    code: null,
    field: null,
    message: "An unexpected error occurred.",
  };
}

function isAppCommandError(error: unknown): error is AppCommandError {
  if (!error || typeof error !== "object") {
    return false;
  }

  const candidate = error as Partial<AppCommandError>;
  return (
    typeof candidate.code === "string" && typeof candidate.message === "string"
  );
}
