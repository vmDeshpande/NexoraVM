# Runtime Diagnostics

The runtime adapter is the provider-neutral boundary between NexoraVM and a future virtualization backend. The current QEMU adapter performs discovery and diagnostics only. It does not create, start, stop, or manage a VM process.

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

## QEMU Command Preview

Milestone 5 adds `QemuCommandSpec`, which keeps the executable path separate from an ordered `arguments` list. A preview is built from a validated persisted VM configuration and detected runtime status through `build_qemu_command_spec`. It includes the VM ID, acceleration choice, display mode, network mode, diagnostics, and optional working directory.

The generator emits deterministic arguments for VM name, `q35`, CPU count, memory, acceleration, display, network, optional disk, and optional ISO. Disk paths are represented in a single QEMU drive argument and are rejected when they contain option-separator commas. Bridged networking is rejected as unsupported; unknown or unavailable WHPX selects the explicit `tcg` fallback. The preview is never executed and does not create or modify disk files.

## WHPX and CPU Virtualization

On Windows, WHPX and CPU virtualization are currently reported as `unknown` because reliable detection is not implemented. On non-Windows builds, Windows-specific diagnostics report `unsupported`. NexoraVM does not enable, disable, or modify Windows features and does not request administrator privileges.

## Before Real VM Execution

The project must first add a typed QEMU command specification with deterministic argument generation, path and resource validation, and security tests. Only after that boundary is reviewed should controlled process management be introduced. Real execution will also require truthful state synchronization, cancellation, cleanup, logging policy, and host capability handling.
