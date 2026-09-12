# Development Guide

## Required Tools

- Node.js and npm.
- Rust stable with the Windows MSVC toolchain.
- Visual Studio or Visual Studio Build Tools with the Desktop development with C++ workload.
- Tauri v2 Windows prerequisites and WebView2 Runtime.

A Visual Studio Developer PowerShell may be required when the Rust MSVC linker or Windows SDK is not available in the current terminal environment. The recommended terminal on Windows is PowerShell, preferably Developer PowerShell when building native code.

## Install Dependencies

From the repository root:

```powershell
npm install
```

Cargo dependencies are resolved by Cargo during Rust commands.

## Run and Build

```powershell
npm run dev
npm run build
npm run tauri dev
npm run tauri build
```

`npm run dev` starts the Vite frontend. `npm run tauri dev` starts the desktop app and its frontend development server. `npm run tauri build` prepares a packaged Tauri build, but the project does not yet promise a production-ready installer.

## Rust Checks

```powershell
cargo fmt --manifest-path .\src-tauri\Cargo.toml -- --check
cargo check --manifest-path .\src-tauri\Cargo.toml
cargo test --manifest-path .\src-tauri\Cargo.toml
```

Run `git diff --check` before submitting changes.

## Working Practices

1. Inspect the current implementation and working tree before editing.
2. Keep changes focused and preserve user changes.
3. Keep frontend `invoke` calls inside service modules.
4. Keep Rust models and TypeScript models synchronized.
5. Add focused tests for validation, persistence recovery, serialization, and security-sensitive behavior.
6. Update the relevant documentation when behavior or limitations change.
7. Do not commit generated output, `node_modules`, Rust `target`, secrets, or machine-specific paths.

## Adding a Tauri Command

1. Identify the owning Rust module.
2. Define typed request and response models.
3. Validate input before side effects.
4. Return `CommandError` with a stable code and user-facing message.
5. Register the command in `src-tauri/src/lib.rs`.
6. Add a typed wrapper in the relevant frontend service.
7. Add focused Rust tests and update documentation.

Do not put direct `invoke` calls in React components.

## Adding or Modifying Typed Models

Update the Rust serde model and the matching TypeScript type together. Preserve `camelCase` field serialization and explicit enums. Decide whether persisted configuration and runtime state should be separate before changing a model.

## Documentation Changes

Use the README for project orientation and links. Use the focused documents for architecture, current status, configuration, runtime behavior, security, contribution workflow, and roadmap detail. Do not document planned behavior as implemented behavior.

## Debugging Common Issues

- **MSVC linker errors:** open Visual Studio Developer PowerShell and retry the Cargo command.
- **WebView2 or Tauri startup errors:** verify Tauri Windows prerequisites and WebView2.
- **Frontend type errors:** run `npm run build` to execute TypeScript checking and Vite bundling.
- **Rust compile errors:** run `cargo check --manifest-path .\src-tauri\Cargo.toml` before starting the desktop app.
- **Runtime diagnostics report unavailable:** verify the configured path points to a regular QEMU executable; discovery does not scan the whole filesystem.

## Commits and Pull Requests

Use focused branches and commits with clear imperative messages. A pull request should explain the behavior change, list validation commands, describe limitations, and call out security implications. Keep unrelated refactors out of the change. Reviewers should be able to trace each new command, model, and persistence effect to its tests and documentation.
