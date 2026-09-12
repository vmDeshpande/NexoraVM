# Contributing to NexoraVM

Thank you for helping improve NexoraVM. The project is in active early development, so contributions should preserve clear boundaries, honest feature status, and a conservative security posture.

## Finding or Proposing Work

Read the [documentation index](docs/README.md), [current status](docs/current-status.md), and [roadmap](docs/roadmap.md) first. Search existing issues before opening a new one. For a new idea, describe the user problem, proposed behavior, alternatives, scope, and security implications.

## Branches and Changes

Create a focused branch from the current main branch. Use a descriptive name such as `docs/runtime-guide`, `fix/settings-recovery`, or `feat/qemu-command-spec`. Keep each change coherent and avoid unrelated refactors, generated files, secrets, and machine-specific paths.

Preserve these project conventions:

- Keep Tauri `invoke` calls inside typed frontend service modules.
- Keep Rust validation and native operations behind the Rust/Tauri boundary.
- Keep VM configuration separate from VM runtime state.
- Use explicit enums and structured errors where practical.
- Never claim that a runtime is available or running without evidence.
- Do not add shell execution, unrestricted process arguments, or unnecessary administrator privileges.

## Frontend and Rust Development

Frontend changes should use the existing React components, TypeScript models, and visual language. Rust changes should keep persistence, validation, and runtime-provider boundaries explicit. Update matching Rust and TypeScript models together when serialized contracts change.

Detailed setup and development commands are in [docs/development.md](docs/development.md).

## Testing

Run the full validation set before opening a pull request:

```powershell
npm run build
cargo fmt --manifest-path .\src-tauri\Cargo.toml -- --check
cargo check --manifest-path .\src-tauri\Cargo.toml
cargo test --manifest-path .\src-tauri\Cargo.toml
git diff --check
```

Add focused tests for validation, persistence recovery, serialization, command behavior, and security-sensitive logic. If native UI interaction cannot be performed, say so explicitly rather than implying it was tested.

## Documentation

Update the relevant documentation whenever commands, models, persistence formats, security behavior, or limitations change. Keep the README concise and use the detailed documents in `docs/` for technical material.

## Commits and Pull Requests

Use clear imperative commit messages. Pull requests should include:

- A concise summary and related issue.
- The change type and implementation details.
- Exact validation commands and results.
- Documentation updates.
- Security considerations and known limitations.
- Screenshots or recordings for meaningful UI changes, when available.

Reviewers will check behavior, compatibility, error handling, tests, documentation accuracy, and security boundaries. See the [pull request template](.github/PULL_REQUEST_TEMPLATE.md).
