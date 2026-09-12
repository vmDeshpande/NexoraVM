# Roadmap

This is the canonical NexoraVM roadmap. A checked item means the repository contains the corresponding implementation, not that the broader product vision is complete.

## Phase 0 — Project Foundation — Completed

**Objective:** establish the repository and technical direction.

**Deliverables:** repository setup, Tauri shell foundation, React/TypeScript frontend, Rust backend direction, architecture and security principles.

**Dependencies:** none.

**Security considerations:** define narrow native boundaries and avoid unrestricted host access.

**Completion criteria:** repository builds as a Tauri application and the initial architecture is documented.

## Phase 1 — Desktop Shell — Completed

**Objective:** provide a usable desktop workspace.

**Deliverables:** navigation, Dashboard, Settings page, reusable layout/components, responsive shell, placeholder application areas.

**Dependencies:** Phase 0.

**Security considerations:** keep the UI separate from privileged native operations.

**Completion criteria:** the desktop shell loads and provides stable navigation.

## Phase 2 — Configuration Foundation — Completed

**Objective:** establish typed configuration and persistence boundaries.

**Deliverables:** application settings persistence, typed VM configuration, structured errors, provider-neutral runtime adapter boundary.

**Dependencies:** Phase 1.

**Security considerations:** validate resource values and paths; recover malformed local data safely.

**Completion criteria:** settings commands and typed configuration compile, validate, persist, and test successfully.

## Phase 3 — VM Management — Completed

**Objective:** manage VM definitions without executing a runtime.

**Deliverables:** persistent VM definitions, create/edit/delete flow, validation, UUID-based IDs, separate VM status model, empty/loading/error states.

**Dependencies:** Phase 2.

**Security considerations:** do not treat configuration as execution; require confirmation for deletion.

**Completion criteria:** definitions can be persisted and managed through typed commands and the Virtual Machines page.

## Phase 4 — Runtime Diagnostics — Completed

**Objective:** safely report whether a usable QEMU executable can be found.

**Deliverables:** configured-path, bounded standard-location, and PATH lookup; regular-file validation; fixed-argument version detection with timeout; runtime diagnostics in Dashboard and Settings; typed WHPX/virtualization states.

**Dependencies:** Phases 1–3.

**Security considerations:** no shell execution, arbitrary command strings, whole-filesystem scanning, elevated privileges, or VM startup.

**Completion criteria:** missing and invalid QEMU paths produce structured diagnostics, and the Rust test suite covers the discovery boundary.

## Phase 5 — Safe QEMU Command Construction — Completed

**Objective:** produce deterministic, validated QEMU command specifications without starting a process.

**Deliverables:** typed command specification, deterministic argument generation, optional disk and ISO mapping, display and network mapping, TCG/WHPX acceleration selection, security validation, VM-page preview, and unit tests. The specification is not executed.

**Dependencies:** VM configuration, runtime diagnostics, storage design.

**Security considerations:** prohibit arbitrary arguments, shell interpolation, unsafe path composition, and unsupported device mappings.

**Completion criteria:** a command specification can be reviewed and tested without spawning QEMU. This milestone is complete.

## Phase 6 — Controlled QEMU Process Management — Completed

**Objective:** manage a validated QEMU process through a narrow lifecycle abstraction.

**Deliverables:** process lifecycle, start/stop handling, exit-code handling, logs, timeouts, cancellation, and no-shell execution.

**Dependencies:** Phase 5 and security review.

**Security considerations:** least privilege, bounded resources, output handling, cancellation, and safe cleanup.

**Completion criteria:** process behavior is covered by focused tests, accepts only validated command specifications, uses direct process APIs, bounds output, and cannot receive arbitrary UI command input. This milestone is complete.

## Phase 7 — VM Runtime Integration — Current next milestone

**Objective:** connect persisted definitions to real runtime state.

**Deliverables:** startup, shutdown, state synchronization, console/display integration, and error recovery.

**Dependencies:** Phases 5–6 and WHPX/host capability work.

**Security considerations:** state must reflect observed process/runtime events, not optimistic UI updates.

**Completion criteria:** supported VMs can be started and stopped with truthful state reporting.

## Phase 8 — Storage and Networking — Planned

**Objective:** manage guest disks, ISO assets, and network configuration.

**Deliverables:** virtual disk management, ISO library, network configuration, and resource validation.

**Dependencies:** VM runtime integration and security policy.

**Security considerations:** path confinement, resource limits, explicit network policy, and no silent host sharing.

**Completion criteria:** storage and networking workflows are validated and observable.

## Phase 9 — AI Workspace — Planned

**Objective:** add provider-neutral AI configuration and model interaction.

**Deliverables:** model-provider abstraction, local/cloud provider support, model selection, AI workspace UI, and secure tool permissions.

**Dependencies:** stable application/runtime boundaries and security policy.

**Security considerations:** credentials isolation, least privilege, user visibility, and provider-specific trust boundaries.

**Completion criteria:** AI configuration works without coupling model providers to VM orchestration.

## Phase 10 — AI-to-VM Workflows — Planned

**Objective:** allow controlled natural-language workflows around VMs.

**Deliverables:** configuration assistance, safe action planning, approval for sensitive actions, auditable execution, rollback, and failure handling.

**Dependencies:** Phases 7–9 and capability policy.

**Security considerations:** no direct unrestricted host API access; destructive actions require approval and recovery planning.

**Completion criteria:** representative workflows are reviewable, bounded, and auditable.

## Phase 11 — Release Hardening — Planned

**Objective:** prepare for responsible distribution.

**Deliverables:** installer, updates, accessibility, performance work, security review, CI/CD, documentation, and release process.

**Dependencies:** product functionality and security review.

**Security considerations:** signing, update integrity, dependency review, crash handling, and reproducible builds.

**Completion criteria:** release criteria are documented and verified for the supported Windows target.
