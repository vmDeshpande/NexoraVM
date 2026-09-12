import { placeholderRuntimeSnapshot } from "../features/dashboard/placeholderRuntime";
import type { RuntimeSnapshot } from "../types/runtime";
import type { RuntimeStatus } from "../types/runtimeStatus";
import { invoke } from "@tauri-apps/api/core";

export interface RuntimeService {
  getRuntimeSnapshot: () => Promise<RuntimeSnapshot>;
  getRuntimeStatus: () => Promise<RuntimeStatus>;
  refreshRuntimeStatus: () => Promise<RuntimeStatus>;
}

export const runtimeService: RuntimeService = {
  getRuntimeSnapshot: async () => placeholderRuntimeSnapshot,
  getRuntimeStatus: () => invoke<RuntimeStatus>("get_runtime_status"),
  refreshRuntimeStatus: () => invoke<RuntimeStatus>("refresh_runtime_status"),
};
