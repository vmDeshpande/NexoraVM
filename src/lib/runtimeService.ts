import { placeholderRuntimeSnapshot } from "../features/dashboard/placeholderRuntime";
import type { RuntimeSnapshot } from "../types/runtime";

export interface RuntimeService {
  getRuntimeSnapshot: () => Promise<RuntimeSnapshot>;
}

export const runtimeService: RuntimeService = {
  getRuntimeSnapshot: async () => placeholderRuntimeSnapshot,
};
