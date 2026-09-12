# Current Status

NexoraVM is an early-development desktop application. This document describes the implementation that exists in the repository; planned behavior is listed separately as deferred work.

## Completed Milestones

1. Desktop application shell with Tauri v2, React, TypeScript, Vite, Rust, navigation, layout, and responsive styling.
2. Persistent application settings and typed VM/runtime configuration foundation.
3. Persistent VM definitions and VM management flow.
4. Runtime discovery and QEMU capability diagnostics.

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

The Start/Stop commands intentionally return `not_implemented` and do not launch a VM.

## Settings and VM Definitions

Settings persist as JSON in the OS application-data directory. Supported settings include QEMU executable path, default VM storage and ISO paths, memory, CPU count, display mode, start-minimized preference, and update checks.

VM definitions persist separately as JSON. They contain typed configuration for name, operating system, CPU, memory, disk size, ISO path, network mode, display mode, Secure Boot, and TPM. VM runtime status is separate from configuration and defaults to `stopped`.

## Runtime Discovery

Runtime diagnostics check the configured QEMU path first, then bounded standard installation locations and PATH entries. Candidates must be regular files. A detected executable is queried only with the fixed `--version` argument, without shell execution, arbitrary arguments, or VM startup.

The result includes QEMU availability, configured and detected paths, version text when available, diagnostics, and WHPX/CPU virtualization capability states. Windows capability states are currently `unknown`; non-Windows builds report the Windows-specific diagnostics as `unsupported`.

## Tests and Validation

The Rust suite covers settings recovery, VM validation, VM serialization, definition storage recovery, duplicate and missing IDs, status defaults, runtime status serialization, invalid paths, version parsing, missing executables, and placeholder lifecycle behavior.

Repository validation commands are documented in [development](development.md). Build output, `node_modules`, and Rust `target` directories are not documentation deliverables.

## Known Limitations

- No QEMU VM process execution.
- No real VM startup, shutdown, pause, or lifecycle synchronization.
- No WHPX execution integration or reliable Windows virtualization detection.
- No virtual disk creation or disk management.
- No complete networking configuration.
- No guest display, input forwarding, or guest agent.
- No AI model or inference integration.
- No production installer or release process guarantee.

## Intentionally Deferred

Controlled QEMU command construction must be designed and tested before process management. Later work must also address host capability detection, storage, networking, runtime state events, guest interaction, AI permissions, security review, and release hardening.
