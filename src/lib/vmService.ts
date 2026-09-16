import { invoke } from "@tauri-apps/api/core";
import type { VmConfiguration, VmDefinition } from "../types/vmConfig";
import type {
  QemuBootMode,
  QemuCommandSpec,
  QemuProcessStatus,
  VmDiskStatus,
  VmStartRequest,
} from "../types/runtimeStatus";

export interface VmService {
  listVmDefinitions: () => Promise<VmDefinition[]>;
  getVmDefinition: (vmId: string) => Promise<VmDefinition>;
  createVmDefinition: (configuration: VmConfiguration) => Promise<VmDefinition>;
  updateVmDefinition: (configuration: VmConfiguration) => Promise<VmDefinition>;
  deleteVmDefinition: (vmId: string) => Promise<void>;
  createVmDisk: (vmId: string) => Promise<VmDiskStatus>;
  getVmDiskStatus: (vmId: string) => Promise<VmDiskStatus>;
  startVm: (request: VmStartRequest) => Promise<QemuProcessStatus>;
  stopVm: (vmId: string) => Promise<QemuProcessStatus>;
  getVmProcessStatus: (vmId: string) => Promise<QemuProcessStatus>;
  refreshAllVmProcessStatuses: () => Promise<QemuProcessStatus[]>;
  buildQemuCommandSpec: (request: {
    vmId: string;
    bootMode: QemuBootMode;
  }) => Promise<QemuCommandSpec>;
}

export const vmService: VmService = {
  listVmDefinitions: () => invoke<VmDefinition[]>("list_vm_definitions"),
  getVmDefinition: (vmId) => invoke<VmDefinition>("get_vm_definition", { vmId }),
  createVmDefinition: (configuration) =>
    invoke<VmDefinition>("create_vm_definition", { configuration }),
  updateVmDefinition: (configuration) =>
    invoke<VmDefinition>("update_vm_definition", { configuration }),
  deleteVmDefinition: (vmId) => invoke<void>("delete_vm_definition", { vmId }),
  createVmDisk: (vmId) => invoke<VmDiskStatus>("create_vm_disk", { vmId }),
  getVmDiskStatus: (vmId) => invoke<VmDiskStatus>("get_vm_disk_status", { vmId }),
  startVm: (request) => invoke<QemuProcessStatus>("start_vm", request),
  stopVm: (vmId) => invoke<QemuProcessStatus>("stop_vm", { vmId }),
  getVmProcessStatus: (vmId) =>
    invoke<QemuProcessStatus>("get_vm_process_status", { vmId }),
  refreshAllVmProcessStatuses: () =>
    invoke<QemuProcessStatus[]>("refresh_all_vm_runtime_status"),
  buildQemuCommandSpec: (request) => invoke<QemuCommandSpec>("build_qemu_command_spec", request),
};