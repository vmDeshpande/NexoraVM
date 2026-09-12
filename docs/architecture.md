# Architecture

NexoraVM keeps the web interface, native application boundary, persistence, and virtualization provider boundary separate. The goal is to make future runtime and AI work explicit rather than allowing provider-specific process details to spread through the UI.

## High-Level Structure

```text
React + TypeScript UI
        |
        | typed frontend services and Tauri invoke commands
        v
Tauri desktop boundary
        |
        v
Rust application core
  | settings persistence
  | VM definition persistence
  | validation and structured errors
  | runtime adapter boundary
        |
        +--> QEMU discovery and diagnostics (implemented)
        +--> QEMU/WHPX execution (planned)
        +--> AI runtime and policy boundary (planned)
```

## Responsibilities

### React and TypeScript frontend

The frontend owns presentation, forms, loading states, user feedback, and navigation. Tauri calls are kept in service modules such as `settingsService`, `vmService`, and `runtimeService`. Frontend models mirror the serialized Rust contracts.

Current pages include Dashboard, Settings, and Virtual Machines. AI Workspace and Storage remain placeholder areas.

### Tauri desktop layer

Tauri creates the desktop window and registers native commands. It is the narrow bridge between the web UI and Rust. Command registration is centralized in `src-tauri/src/lib.rs`.

### Rust application core

Rust owns validation, persistence, structured command errors, runtime discovery, and the provider-neutral runtime adapter trait. Settings and VM definitions are stored under the operating system's application-data directory rather than in repository files or user-specific hardcoded locations.

### Runtime adapter

`RuntimeAdapter` defines the provider-neutral boundary for availability checks, capability discovery, VM configuration validation, future VM definition creation, and lifecycle operations. `QemuRuntimeAdapter` currently implements discovery and diagnostics. VM creation through the adapter and Start/Stop remain explicit `not_implemented` operations.

### QEMU and future WHPX integration

QEMU is the planned runtime backend. Current code only checks bounded candidate paths and invokes a validated executable with the fixed `--version` argument. WHPX and CPU virtualization states are reported as `unknown` on Windows until reliable detection is implemented. No Windows features are enabled automatically.

The command-construction layer now produces a typed `QemuCommandSpec` with a `PathBuf` executable and ordered argument vector. It is a diagnostic preview boundary, not process management. The builder maps only supported display and network modes, uses TCG when WHPX is not confirmed available, and rejects unsupported bridged networking. No shell command string or arbitrary QEMU flags are accepted.

### Future AI runtime

The AI workspace is intended to sit behind provider-neutral model and capability interfaces. Future AI actions must pass through policy, user approval where appropriate, and observable execution boundaries. AI code is not implemented yet.

## Persistence

- `settings.json` stores application preferences.
- `vm-definitions.json` stores VM configurations plus separate lifecycle status values.
- Missing files recover to defaults or an empty definition list.
- Malformed settings recover to defaults; malformed VM definition JSON recovers to an empty list.
- Loaded VM definitions are validated and duplicate IDs are rejected.

## Data Flow

1. A React page collects typed input.
2. A frontend service invokes a named Tauri command.
3. Rust deserializes the payload into a typed model.
4. Rust validates the model or performs a bounded diagnostic operation.
5. Rust returns typed data or a structured error.
6. The page updates its loading, success, or error state.

For command preview, a VM ID is resolved to a persisted definition, runtime diagnostics are refreshed, and the Rust builder returns structured executable/argument data. No process is launched.

## Error Handling

Commands return structured errors with a code, user-facing message, and optional field. Persistence errors, validation errors, not-found errors, conflicts, and not-implemented runtime operations are distinguished. Internal stack traces are not sent to the UI.

## Security Boundaries

The frontend does not execute host commands directly. Runtime discovery does not use a shell, does not accept arbitrary argument lists, does not scan the entire filesystem, and does not request administrator privileges. Future QEMU process management must preserve these boundaries and add typed command construction before any VM process is started.

The separation between VM configuration and runtime state prevents stored intent from being mistaken for actual execution state. The provider-neutral adapter also leaves room for another runtime without coupling the UI to QEMU command syntax.
