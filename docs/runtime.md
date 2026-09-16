# Runtime Diagnostics

The runtime adapter is the provider-neutral boundary between NexoraVM and a future virtualization backend. The QEMU adapter performs discovery, command construction, and controlled process management. It does not create disks, modify networking, enable WHPX, or implement complete guest runtime integration.

## Discovery Order

1. Check the configured QEMU executable path from application settings.
2. Check bounded standard locations under the Windows Program Files variables for `qemu-system-x86_64.exe` or the platform-neutral executable name.
3. Check PATH entries for those fixed executable names.

NexoraVM does not scan the entire filesystem. A candidate must be a regular file before it is considered.

## Version Detection

For a validated candidate, the adapter starts only that fixed executable with the fixed `--version` argument. Standard input is closed, output is captured, and the operation is limited by a two-second timeout. A non-zero exit, unreadable output, timeout, or unrecognized version line produces a structured diagnostic. No shell is used and no UI-provided argument list is accepted.

## Runtime Status

The typed status includes:

- Runtime type, currently QEMU.
- Availability state: available, unavailable, error, or another explicit capability state.
- Configured and detected paths.
- QEMU version text when successfully detected.
- WHPX and CPU virtualization capability statuses.
- Diagnostic codes and messages.
- Last-check timestamp.

Dashboard and Settings expose the same status through `get_runtime_status` and `refresh_runtime_status`, using the dedicated frontend runtime service.

VM runtime status is available through `get_vm_process_status` and `refresh_all_vm_runtime_status`. These commands inspect only processes owned by the in-memory process manager. They do not scan for or adopt unrelated QEMU processes.

## QEMU Command Preview

Milestone 5 adds `QemuCommandSpec`, which keeps the executable path separate from an ordered `arguments` list. A preview is built from a validated persisted VM configuration and detected runtime status through `build_qemu_command_spec`. It includes the VM ID, acceleration choice, display mode, network mode, diagnostics, and optional working directory.

The generator emits deterministic arguments for VM name, `q35`, CPU count, memory, acceleration, display, network, optional disk, and optional ISO. Disk paths are represented in a single QEMU drive argument and are rejected when they contain option-separator commas. Bridged networking is rejected as unsupported; unknown or unavailable WHPX selects the explicit `tcg` fallback. The preview is never executed and does not create or modify disk files.

## Persistent Disk

`DiskManager` creates and reports persistent disk images through `create_vm_disk` and `get_vm_disk_status`. The backend validates a VM definition before disk creation, uses a conservative `qcow2` default format, and accepts a default size of 64 GiB that callers can configure through VM configuration. Disk creation invokes `qemu-img create` directly, without shell execution, without arbitrary frontend arguments, and without overwriting an existing disk image. If the disk path already exists and is a regular file, `create_vm_disk` reports the disk as ready instead of overwriting it. Missing, directory, unsafe, or invalid paths return structured validation errors.

## QEMU Boot Modes

The QEMU command builder now takes an explicit boot mode. Install mode attaches the persistent disk and the ISO as removable media, then sets the boot order to start from removable media. Normal mode attaches only the persistent disk and sets the boot order to start from the disk. Normal boot requires an existing persistent disk; install boot requires an existing persistent disk and a non-empty ISO path. The boot mode is included in `QemuCommandSpec` and surfaced in the VM preview. Preview commands never execute QEMU or create disks.

## Controlled Process Management

`QemuProcessManager` accepts only a validated `QemuCommandSpec`. It passes the executable path and every argument separately to Rust's process API, optionally applies the typed working directory, closes standard input, and captures stdout/stderr in bounded 64 KiB buffers. It never invokes a shell and is not exposed as a generic process-execution command.

When `start_vm` is called, the backend resolves the persisted VM definition, validates the ISO path before launch, refreshes runtime diagnostics, builds the typed QEMU command specification, and launches QEMU through the controlled process manager. Live process handles are kept only in Tauri-managed memory. Start reports a process as running only after the child is observed alive. Immediate exits become failures, duplicate starts are rejected, and stop uses a bounded graceful-stop attempt followed by forced termination when needed.

A monitor thread checks managed children at a bounded interval, records exit codes and output, and removes the live child handle after exit. It uses short mutex scopes and never adopts a process after application restart.

Actual QEMU process launch is now enabled when runtime discovery succeeds and the ISO path exists, but this is not complete VM execution. No disk is created, no WHPX feature is enabled, and no guest display or full lifecycle synchronization is provided yet.

## WHPX and CPU Virtualization

On Windows, WHPX and CPU virtualization are currently reported as `unknown` because reliable detection is not implemented. On non-Windows builds, Windows-specific diagnostics report `unsupported`. NexoraVM does not enable, disable, or modify Windows features and does not request administrator privileges.

## Before Real VM Execution

Controlled process launch now exists, but complete VM runtime integration still requires truthful guest-state synchronization, cancellation policy, cleanup review, logging policy, console/display handling, and deeper host capability handling. On application restart, previously managed processes are reported as `not-started`; NexoraVM does not rediscover or adopt unrelated processes. The next milestone must build those pieces without weakening the typed command and least-privilege boundaries.
