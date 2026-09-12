# NexoraVM

> A Windows-first desktop virtualization and AI workspace built with Tauri, React, TypeScript, and Rust.

NexoraVM is an early-stage desktop application for defining virtual machines, diagnosing local runtime readiness, and eventually operating guest systems through explicit, auditable workflows. The long-term vision is a focused workspace where virtualization, storage, networking, and AI-assisted guest interaction share clear boundaries and user-visible controls.

## Current Status

NexoraVM is in active early development. The desktop shell, persistent application settings, typed VM configuration, persistent VM definitions, VM management UI, and safe QEMU discovery diagnostics are implemented. Real VM execution is not implemented yet.

The current application does not start or stop QEMU, create virtual disks, integrate WHPX execution, manage guest networking, run AI models, or provide a production-ready installer. See [current status](docs/current-status.md) for the exact implementation boundary.

## Feature Status

| Feature | Status | Notes |
| --- | --- | --- |
| Desktop shell | Implemented | Tauri v2 window, React UI, navigation, responsive layout. |
| Dashboard | Implemented | Workspace summary, persisted VM totals, and runtime diagnostics. |
| Persistent settings | Implemented | JSON settings in the OS application-data directory with validation and reset. |
| VM configuration | Implemented | Typed Rust and TypeScript models with resource and path validation. |
| VM create/edit/delete | Implemented | Persistent definitions, UUID-based IDs, typed form, and delete confirmation. |
| Runtime discovery | Implemented | Bounded configured-path, standard-location, and PATH lookup. |
| QEMU detection | Partial | Regular-file validation and fixed `--version` probing; no VM launch. |
| QEMU execution | Not implemented | Controlled command construction must precede process execution. |
| WHPX integration | Planned | Current Windows capability fields report `unknown`; no WHPX execution exists. |
| VM lifecycle | Not implemented | Start/Stop commands are explicit placeholders. |
| Storage management | Planned | Storage preferences exist; disk creation and management do not. |
| AI workspace | Planned | Navigation placeholder only. |
| AI model integration | Not implemented | No local or cloud inference provider exists. |
| Snapshots | Planned | No snapshot or restore workflow exists. |
| Networking | Planned | VM network mode is modeled; host/network configuration is not implemented. |

## Screenshots

No verified screenshots are included yet. Screenshots will be added after a repeatable capture workflow is established for the desktop application.

## Architecture

The frontend is a React and TypeScript application. Tauri provides the desktop window and a narrow command bridge. Rust owns persistence, validation, structured errors, runtime discovery, and the provider-neutral runtime adapter boundary. The current QEMU adapter performs bounded executable discovery and version diagnostics only. Future QEMU/WHPX execution and AI runtime work will remain behind explicit interfaces rather than leaking process or provider details into the UI.

See [architecture](docs/architecture.md) for data flow, persistence, error handling, and security boundaries.

## Technology Stack

- **Tauri v2:** Windows desktop shell and typed bridge to native commands.
- **React:** component-based frontend and page state management.
- **TypeScript:** typed UI models and service contracts.
- **Vite:** frontend development server and production bundling.
- **Rust:** native application core, validation, persistence, diagnostics, and future process control.
- **QEMU/WHPX:** planned virtualization backend; neither is used for VM execution yet.

## Installation and Development

### Prerequisites

- Node.js and npm.
- Rust stable with the Windows MSVC toolchain.
- Visual Studio or Visual Studio Build Tools with the Desktop development with C++ workload.
- Tauri v2 Windows prerequisites.
- WebView2 Runtime, normally present on Windows 11.

On Windows, Visual Studio Developer PowerShell may be required for the Rust MSVC linker and Windows SDK.

### Install dependencies

```powershell
npm install
```

### Build and run

```powershell
npm run dev
npm run build
npm run tauri dev
npm run tauri build
```

`npm run tauri build` prepares a packaged build, but NexoraVM does not yet guarantee a production-ready installer or release process.

### Rust validation

```powershell
cargo fmt --manifest-path .\src-tauri\Cargo.toml -- --check
cargo check --manifest-path .\src-tauri\Cargo.toml
cargo test --manifest-path .\src-tauri\Cargo.toml
git diff --check
```

The complete contributor workflow is documented in [development](docs/development.md).

## Project Structure

```text
NexoraVM/
|-- docs/                  Documentation and roadmap
|-- public/                Static frontend assets
|-- src/
|   |-- components/        Reusable UI, including runtime diagnostics
|   |-- features/          Feature-specific frontend data
|   |-- hooks/              React hooks
|   |-- layouts/            Desktop shell layout
|   |-- lib/                Typed frontend services and navigation
|   |-- pages/              Dashboard, Settings, VM management, placeholders
|   |-- types/              TypeScript domain models
|   `-- App.tsx             Frontend route selection
|-- src-tauri/
|   |-- src/                Rust commands, models, persistence, adapters
|   |-- capabilities/       Tauri permissions
|   |-- Cargo.toml          Rust dependencies and package metadata
|   `-- tauri.conf.json     Tauri application configuration
|-- package.json            Frontend scripts and dependencies
`-- vite.config.ts         Vite configuration
```

## Configuration and Persistence

Application settings are stored as `settings.json` in the operating system's application-data directory selected through Tauri. VM definitions are stored separately as `vm-definitions.json` in the same conceptual location. The repository does not contain user-specific absolute paths.

Settings include optional QEMU, VM storage, and ISO paths; default memory and CPU values; display mode; startup preference; and update preference. VM definitions include typed guest, resource, storage, display, network, Secure Boot, and TPM fields. Persistence behavior and validation are described in [configuration](docs/configuration.md).

## Current Limitations

- No actual QEMU process execution.
- No real VM startup, shutdown, pause, or lifecycle synchronization.
- No WHPX execution integration or reliable Windows virtualization detection.
- No virtual disk creation or storage management.
- No complete network management.
- No guest display, input forwarding, or guest agent.
- No AI inference or model-provider integration.
- No snapshot and restore workflow.
- No production installer or release guarantee yet.

Runtime discovery does not claim that QEMU or WHPX is usable unless the configured or boundedly discovered executable passes the fixed version check. Windows WHPX and CPU virtualization fields currently report `unknown`.

## Roadmap

The canonical phased roadmap is [docs/roadmap.md](docs/roadmap.md). It covers safe QEMU command construction, controlled process management, runtime integration, storage and networking, AI workflows, and release hardening.

## Contributing

Contributions should be focused, typed, tested, documented, and security-conscious. Use a branch, preserve existing boundaries, keep Tauri calls in frontend services, add tests for persistence and validation changes, and explain limitations in pull requests. Do not add arbitrary shell execution, unrestricted process arguments, unnecessary administrator privileges, secrets, generated output, or unrelated refactors.

Read [CONTRIBUTING.md](CONTRIBUTING.md) and [docs/development.md](docs/development.md) before opening a pull request.

## Security

Virtualization and process execution are security-sensitive. Arbitrary shell execution, unrestricted command arguments, unsafe path composition, and unnecessary administrator privileges are not acceptable design shortcuts. Future QEMU process management and AI tools must use typed boundaries, least privilege, explicit approvals where needed, bounded resources, and observable error handling.

See [SECURITY.md](SECURITY.md) for the current threat model and deferred security work. A dedicated vulnerability reporting process has not yet been defined.

For troubleshooting and issue-reporting guidance, see [SUPPORT.md](SUPPORT.md).

## License

Licensing is not yet specified. A license should be added before broad distribution or accepting contributions under public reuse terms.

## Disclaimer

NexoraVM is under active development and is not production-ready. Features, APIs, persistence formats, and security boundaries may change before a supported release.

## Documentation Index

- [Documentation index](docs/README.md)
- [Overview](docs/overview.md)
- [Architecture](docs/architecture.md)
- [Current status](docs/current-status.md)
- [Roadmap](docs/roadmap.md)
- [Development guide](docs/development.md)
- [Configuration](docs/configuration.md)
- [Runtime diagnostics](docs/runtime.md)
- [Security](SECURITY.md)
- [Contributing](CONTRIBUTING.md)
- [Architectural decisions](docs/decisions.md)
