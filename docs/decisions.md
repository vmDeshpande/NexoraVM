# Architectural Decisions

This document records the current direction. It complements the historical record in `docs/architecture/decisions/0001-initial-technology-direction.md`.

## Tauri for the Desktop Boundary

**Decision:** Use Tauri v2 for the desktop application.

**Context:** NexoraVM needs a Windows desktop shell with a modern UI and native systems access.

**Rationale:** Tauri keeps the UI productive with web technologies while Rust owns native operations and privileged boundaries.

**Consequences:** The command/event boundary must remain narrow and typed. Windows tooling and WebView2 are development prerequisites.

**Status:** Accepted and implemented for the shell.

## React and TypeScript for the UI

**Decision:** Use React and TypeScript for the frontend.

**Context:** The application has multiple settings, VM, diagnostics, and future AI workflows.

**Rationale:** Component reuse, typed service contracts, and predictable state handling support those workflows without coupling UI code to Rust implementation details.

**Consequences:** Serialized Rust models and TypeScript types must stay synchronized.

**Status:** Accepted and implemented.

## Rust Owns Native Operations

**Decision:** Keep persistence, validation, runtime discovery, and future process control in Rust.

**Context:** Filesystem and process operations are security-sensitive and platform-dependent.

**Rationale:** Rust provides a strong native boundary and makes it possible to centralize validation and process policy.

**Consequences:** Frontend pages communicate through Tauri commands and dedicated services.

**Status:** Accepted and implemented for current native operations.

## Provider-Neutral Runtime Adapter

**Decision:** Expose virtualization operations through a provider-neutral runtime adapter.

**Context:** QEMU is planned first, but the application should not make QEMU command details part of every product layer.

**Rationale:** The adapter allows capability discovery, validation, and future lifecycle behavior to be tested independently from UI code and leaves room for another backend.

**Consequences:** QEMU-specific behavior stays in the adapter; real execution remains a later concern.

**Status:** Accepted and partially implemented.

## Configuration Is Separate from Runtime State

**Decision:** Store VM configuration separately from VM runtime status.

**Context:** A saved definition expresses user intent; it does not prove that a guest process exists.

**Rationale:** This prevents the UI from claiming a VM is running based only on persisted data.

**Consequences:** Definitions contain configuration only. Lifecycle state comes from observed managed-process events and resets to `not-started` after application restart.

**Status:** Accepted and implemented at the model level.

## Defer Process Execution Until Command Construction Is Ready

**Decision:** Do not start QEMU until typed command construction and validation exist.

**Context:** Arbitrary or string-built process arguments create command injection, path, device, and resource risks.

**Rationale:** Deterministic command specifications can be reviewed and unit-tested before a process is launched.

**Consequences:** Start and Stop use only validated command specifications through the in-memory process manager; full guest runtime integration remains deferred.

**Status:** Accepted and partially implemented.

## Avoid Unnecessary Privileges

**Decision:** Do not request administrator privileges or modify host features automatically.

**Context:** WHPX, networking, filesystem access, and process execution can affect the host.

**Rationale:** Least privilege limits blast radius and keeps user control explicit.

**Consequences:** Current WHPX and virtualization diagnostics may report `unknown`, and future setup flows must explain prerequisites rather than silently changing them.

**Status:** Accepted and current.
