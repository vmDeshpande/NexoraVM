# Current Status

NexoraVM is an early-development desktop application. This document describes the implementation that exists in the repository; planned behavior is listed separately as deferred work.

## Completed Milestones

1. Desktop application shell with Tauri v2, React, TypeScript, Vite, Rust, navigation, layout, and responsive styling.
2. Persistent application settings and typed VM/runtime configuration foundation.
3. Persistent VM definitions and VM management flow.
4. Runtime discovery and QEMU capability diagnostics.
5. Safe typed QEMU command-spec construction and preview.
6. Controlled QEMU process management.
7. First real QEMU launch path with ISO path validation: persisted VM definitions can launch a validated QEMU command when QEMU is detected and the ISO path exists.

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

The Start/Stop commands launch or stop only a validated QEMU command specification through the in-memory process manager. They do not modify persisted VM configuration or claim a guest is running when the process is unavailable.

The command-spec command returns a deterministic executable path and ordered argument list for diagnostics only. It does not launch QEMU, create disks, or change host state. Start/Stop use that same specification through controlled direct process APIs.

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
- No virtual disk creation or disk management.
- No complete networking configuration.
- No guest display, input forwarding, or guest agent.
- No AI model or inference integration.
- No production installer or release process guarantee.
- Start requests fail clearly when QEMU is unavailable or the configured ISO path does not exist.

## Intentionally Deferred

Controlled QEMU process management is implemented with bounded output, duplicate-start protection, stop fallback, and live status. The next runtime milestone must add complete VM runtime integration, guest display/state synchronization, and deeper host capability handling.
