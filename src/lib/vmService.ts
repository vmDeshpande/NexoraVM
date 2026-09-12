import { invoke } from "@tauri-apps/api/core";
import type { VmConfiguration, VmDefinition } from "../types/vmConfig";

export interface VmService {
  listVmDefinitions: () => Promise<VmDefinition[]>;
  getVmDefinition: (vmId: string) => Promise<VmDefinition>;
  createVmDefinition: (configuration: VmConfiguration) => Promise<VmDefinition>;
  updateVmDefinition: (configuration: VmConfiguration) => Promise<VmDefinition>;
  deleteVmDefinition: (vmId: string) => Promise<void>;
  startVm: (vmId: string) => Promise<void>;
  stopVm: (vmId: string) => Promise<void>;
}

export const vmService: VmService = {
  listVmDefinitions: () => invoke<VmDefinition[]>("list_vm_definitions"),
  getVmDefinition: (vmId) => invoke<VmDefinition>("get_vm_definition", { vmId }),
  createVmDefinition: (configuration) =>
    invoke<VmDefinition>("create_vm_definition", { configuration }),
  updateVmDefinition: (configuration) =>
    invoke<VmDefinition>("update_vm_definition", { configuration }),
  deleteVmDefinition: (vmId) => invoke<void>("delete_vm_definition", { vmId }),
  startVm: (vmId) => invoke<void>("start_vm", { vmId }),
  stopVm: (vmId) => invoke<void>("stop_vm", { vmId }),
};