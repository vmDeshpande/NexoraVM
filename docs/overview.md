# NexoraVM Overview

NexoraVM is a Windows-first desktop virtualization and AI workspace built with Tauri, React, TypeScript, and Rust. It aims to give users one controlled workspace for defining virtual machines, inspecting host/runtime readiness, and eventually operating guest systems through explicit, auditable AI-assisted workflows.

## The Problem

Virtual machine setup, runtime diagnostics, storage choices, and automation are often scattered across command-line tools and provider-specific interfaces. NexoraVM is intended to bring those concerns together behind a clear desktop experience and narrow native interfaces.

The project is deliberately building the configuration and safety boundaries before it performs real virtualization. That order keeps future QEMU/WHPX integration observable, testable, and resistant to arbitrary command execution.

## Intended Users

- Desktop users who want a focused VM workspace.
- Developers building and testing guest environments on Windows.
- Contributors interested in safe desktop virtualization tooling.
- Future users who want controlled AI assistance around VM workflows.

## Main Product Areas

- **Dashboard:** workspace summary, persisted VM totals, and runtime diagnostics.
- **Settings:** application preferences and an optional configured QEMU path.
- **Virtual Machines:** persistent VM definitions with typed configuration, validation, and CRUD management.
- **Runtime diagnostics:** bounded QEMU discovery, fixed version probing, and capability status reporting.
- **AI workspace:** reserved for future provider and permission-aware automation work.
- **Storage and networking:** planned areas for future VM infrastructure.

## Current Maturity

NexoraVM is in active early development. The desktop shell, settings persistence, VM definition persistence, typed configuration models, runtime discovery diagnostics, QEMU command construction, and controlled QEMU process management exist today. Complete guest runtime integration, display, networking, and AI workflows do not.

The current application can construct validated QEMU command specifications, start and stop QEMU processes through those specifications, manage their lifecycle and output, create persistent virtual disks, and validate boot prerequisites (ISO paths, disk existence, boot mode rules). It does not yet integrate WHPX execution, configure VM networking, run AI models, or provide a production installer. Runtime status values reflect managed process state, not guest operation; newly created definitions default to `stopped`, and Start/Stop are gated by validated disk and ISO prerequisites.

## Current Versus Future

| Area | Today | Future direction |
| --- | --- | --- |
| Configuration | Typed settings and VM definitions persist locally as JSON. | Versioned migrations and broader metadata. |
| Runtime | QEMU executable discovery and diagnostics only. | Controlled command construction and process lifecycle. |
| Virtualization | No guest process or hypervisor operation. | QEMU with WHPX where supported. |
| AI | Navigation placeholder only. | Provider-neutral models and approval-aware tools. |
| Safety | Path validation, bounded lookup, no shell execution. | Capability policy, audit events, approvals, and recovery. |

See [current status](current-status.md), [architecture](architecture.md), and the [roadmap](roadmap.md) for details.
