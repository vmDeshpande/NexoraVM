import type { RuntimeSnapshot } from "../../types/runtime";

export const placeholderRuntimeSnapshot: RuntimeSnapshot = {
  activity: [],
  runtime: {
    detail: "The application shell is ready. Backend services are not configured yet.",
    label: "Not configured",
    state: "not-configured",
  },
  virtualMachines: {
    count: 0,
    detail: "No virtual machines have been created yet.",
    state: "not-configured",
  },
  ai: {
    detail: "No AI provider or model has been connected.",
    model: "Not connected",
    provider: "Not configured",
    state: "not-configured",
  },
  storage: {
    detail: "No storage location has been configured.",
    state: "not-configured",
  },
};
