# Current Status

NexoraVM is an early-development desktop application. This document describes the implementation that exists in the repository; planned behavior is listed separately as deferred work.

## Completed Milestones

1. Desktop application shell with Tauri v2, React, TypeScript, Vite, Rust, navigation, layout, and responsive styling.
2. Persistent application settings and typed VM/runtime configuration foundation.
3. Persistent VM definitions and VM management flow.
4. Runtime discovery and QEMU capability diagnostics.
5. Safe typed QEMU command-spec construction and preview.
6. Controlled QEMU process management.
7. VM runtime-state synchronization
8. Persistent VM disk and normal boot flow: VMs can use a persistent `qcow2` disk, disk images can be created through validated `qemu-img` invocation, and the command builder supports explicit install and normal boot modes.
9. Lifecycle hardening and boot validation: Install mode requires a valid ISO and existing persistent disk; normal mode omits the ISO and requires the persistent disk. Duplicate starts are rejected, missing disk or ISO paths return structured errors, failed launches do not leave stale running state, stops clean up managed process handles, and application restarts do not claim old QEMU processes are running.

## Implemented Frontend

- Dashboard with workspace summary, persisted VM totals, and runtime diagnostics.
- Settings page with loading, editing, saving, resetting, validation feedback, and runtime diagnostics.
- Virtual Machines page with loading and empty states, create/edit forms, typed fields, validation feedback, deletion confirmation, and persisted cards.
- Placeholder navigation for AI Workspace and Storage.

Manual native-window interaction has not been represented as completed verification in this repository documentation.

## Implemented Tauri Commands

- `get_app_settings`
- `save_app_settings`
- `reset_app_settings`
- `list_vm_definitions`
- `get_vm_definition`
- `create_vm_definition`
- `update_vm_definition`
- `delete_vm_definition`
- `start_vm`
- `stop_vm`
- `get_runtime_status`
- `refresh_runtime_status`
- `build_qemu_command_spec`
- `get_vm_process_status`
- `refresh_all_vm_runtime_status`
- `create_vm_disk`
- `get_vm_disk_status`

The Start/Stop commands launch or stop only a validated QEMU command specification through the in-memory process manager. They do not modify persisted VM configuration or claim a guest is running when the process is unavailable.

`create_vm_disk` creates a `qcow2` image only for a validated VM definition, rejects existing disk images, and does not use a shell. `get_vm_disk_status` reports whether the persistent disk exists without managing processes.

## Settings and VM Definitions

Settings persist as JSON in the OS application-data directory. Supported settings include QEMU executable path, default VM storage and ISO paths, memory, CPU count, display mode, start-minimized preference, and update checks.

VM definitions persist separately as JSON and contain typed configuration only. They do not persist process handles or runtime status. Live process state is returned by the Rust process manager and starts as `not-started` after application restart.

## Runtime Discovery

Runtime diagnostics check the configured QEMU path first, then bounded standard installation locations and PATH entries. Candidates must be regular files. A detected executable is queried only with the fixed `--version` argument, without shell execution, arbitrary arguments, or VM startup.

The result includes QEMU availability, configured and detected paths, version text when available, diagnostics, and WHPX/CPU virtualization capability states. Windows capability states are currently `unknown`; non-Windows builds report the Windows-specific diagnostics as `unsupported`.

The Virtual Machines page can preview the structured QEMU configuration for a persisted definition. Supported preview mappings include CPU, memory, optional disk and ISO paths, windowed/fullscreen/headless display, disabled/user networking, and TCG fallback acceleration. Bridged networking is currently unsupported.

## Tests and Validation

The Rust suite covers settings recovery, VM validation, VM serialization, definition storage recovery, duplicate and missing IDs, status defaults, runtime status serialization, invalid paths, version parsing, missing executables, command-spec generation, process validation, duplicate starts/stops, failed launches, immediate exits, graceful stop, forced-stop fallback, output limits, monitoring, and process cleanup.

Repository validation commands are documented in [development](development.md). Build output, `node_modules`, and Rust `target` directories are not documentation deliverables.

## Known Limitations

- No complete VM runtime integration, guest display, or lifecycle synchronization.
- QEMU process management is limited to validated command specifications and in-memory status.
- No WHPX execution integration or reliable Windows virtualization detection.
- No virtual disk resizing, snapshots, or disk management beyond initial creation.
- No complete networking configuration.
- No guest display, input forwarding, or guest agent.
- No AI model or inference integration.
- No production installer or release process guarantee.
- Start requests fail clearly when QEMU is unavailable, the configured ISO path does not exist, the persistent disk is missing, or a VM is already starting or running.

## Intentionally Deferred

Controlled QEMU process management is implemented with bounded output, duplicate-start protection, stop fallback, and live status. The next runtime milestone must add complete VM runtime integration, guest display/state synchronization, and deeper host capability handling.
