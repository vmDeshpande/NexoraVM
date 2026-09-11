# NexoraVM

> Virtual machines, operated by intelligence.

NexoraVM is a Windows-first desktop application in early development. The long-term product goal is a unified virtualization workspace where users can create virtual machines, select guest operating systems, configure AI model providers, and eventually connect an AI workspace to the selected VM through explicit, auditable controls.

The current repository contains the first production-oriented desktop shell milestone. It does not yet implement VM creation, QEMU integration, Windows Hypervisor Platform integration, AI inference, storage management, or real runtime control.

## Current Status

**Phase 1 - Desktop Shell**

Implemented:

- Tauri 2 desktop application at the repository root.
- React + TypeScript frontend shell.
- Typed in-memory navigation for Dashboard, Virtual Machines, AI Workspace, Storage, and Settings.
- Reusable layout and UI components.
- Runtime snapshot service abstraction for future Tauri commands.
- Useful empty-state dashboard with runtime, VM, AI provider, storage, and activity sections.
- Dark-first desktop interface with responsive behavior for smaller windows.

Not implemented yet:

- VM creation, lifecycle management, guest display, or input forwarding.
- QEMU or WHPX integration.
- AI model execution, local model loading, or cloud model connections.
- Persistent settings, storage provisioning, credentials, telemetry, or analytics.

## Technology Stack

- Windows 11 x64 first release target.
- Tauri v2 for the desktop runtime.
- Rust for the native application boundary.
- React for the frontend.
- TypeScript for strict frontend types.
- Vite for development and frontend builds.
- Rust MSVC toolchain for Windows builds.

QEMU is planned for a later VM backend milestone and will remain behind a dedicated virtualization adapter.

## Prerequisites

- Node.js and npm.
- Rust stable with the MSVC toolchain.
- Tauri v2 prerequisites for Windows development.
- Microsoft Visual Studio Build Tools or Visual Studio with the C++ desktop workload.
- WebView2 Runtime, normally already present on Windows 11.

## Installation

Install frontend dependencies from the repository root:

```powershell
npm install
```

The project has already been initialized at the repository root. Do not create a nested app directory for normal development.

## Development Commands

Run the Vite frontend:

```powershell
npm run dev
```

Build the frontend:

```powershell
npm run build
```

Run the Tauri desktop app during development:

```powershell
npm run tauri dev
```

Build the packaged Tauri app:

```powershell
npm run tauri build
```

Tauri builds may take longer than frontend builds because they compile the Rust application and prepare desktop bundles.

## Project Structure

```text
NexoraVM/
|-- docs/
|   |-- architecture/
|   `-- development/
|-- public/
|-- src/
|   |-- components/
|   |-- features/
|   |-- hooks/
|   |-- layouts/
|   |-- lib/
|   |-- pages/
|   |-- types/
|   `-- App.tsx
|-- src-tauri/
|   |-- capabilities/
|   |-- icons/
|   |-- src/
|   |-- Cargo.toml
|   `-- tauri.conf.json
|-- package.json
|-- tsconfig.json
`-- vite.config.ts
```

## Architecture Direction

NexoraVM is divided into five major boundaries:

1. Desktop application: UI, settings, and visual VM interaction.
2. Application core: orchestration, state, policies, events, and persistence.
3. Virtualization runtime: adapter over a mature hypervisor backend.
4. AI runtime: provider abstraction, planning, tool execution, verification, and recovery.
5. Guest integration: a controlled guest agent for communication with the VM.

The current code only implements the desktop shell and frontend service boundary. Native commands, VM runtime operations, and AI runtime operations are intentionally deferred.

## Design Principles

- Desktop-first: NexoraVM is intended to be a complete desktop executable.
- Real virtualization: the project will use a mature backend instead of building a hypervisor from scratch.
- Provider-neutral AI: local and cloud model providers should sit behind a stable abstraction.
- Explicit user control: sensitive AI and VM actions should be visible, configurable, auditable, and revocable.
- Isolation by default: host access, networking, clipboard, shared folders, and file transfer should require deliberate configuration.
- Recoverable automation: future AI actions should be observable and reversible through logs, snapshots, checkpoints, and recovery flows.

## Planned Next Steps

1. Add persistent settings and structured application events.
2. Define shared domain models for VM configuration and runtime state.
3. Add host capability detection for Windows virtualization prerequisites.
4. Define the virtualization adapter trait before adding QEMU-specific code.
5. Implement VM creation and lifecycle management behind the adapter.
6. Add AI provider configuration and model selection without coupling it to VM orchestration.

## License

License terms will be finalized before the first public implementation release.
