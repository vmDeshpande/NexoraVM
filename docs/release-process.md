# Release Process

NexoraVM does not yet have an established production release process. This document records the intended checklist so future releases are repeatable and reviewable.

## Pre-Release Checks

- Confirm the intended version in `package.json`, `src-tauri/Cargo.toml`, and `src-tauri/tauri.conf.json` is consistent.
- Review the feature status and known limitations.
- Review open release-blocking issues.
- Confirm no secrets, user-specific paths, generated output, `node_modules`, or Rust `target` files are included.

## Build and Test Validation

```powershell
npm run build
cargo fmt --manifest-path .\src-tauri\Cargo.toml -- --check
cargo check --manifest-path .\src-tauri\Cargo.toml
cargo test --manifest-path .\src-tauri\Cargo.toml
git diff --check
```

A future release should also verify a Windows Tauri build in a clean environment and exercise the packaged application on the supported Windows target.

## Documentation and Changelog

Update [CHANGELOG.md](../CHANGELOG.md), README status tables, current status, configuration/runtime limitations, and release notes together. Do not describe planned QEMU, WHPX, AI, storage, or networking functionality as released until it is verified.

## Installer and Security Review

Installer verification, signing, update integrity, dependency review, permissions review, and security review are not established yet. Before broad distribution, validate bundle contents, Windows installation/uninstallation, upgrades, rollback behavior, logging, and least-privilege behavior.

## Versioning and Release Notes

No versioning policy has been selected. Before the first public release, choose a versioning convention and document compatibility expectations for persisted settings and VM-definition files. Release notes should summarize user-visible changes, known limitations, validation performed, and migration considerations.

## Rollback

A future release process should define how to withdraw a broken build, publish a known-good version, preserve or migrate local configuration safely, and communicate security or data-loss risks. No automated rollback mechanism exists today.
