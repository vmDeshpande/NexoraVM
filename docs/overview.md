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

NexoraVM is in active early development. The desktop shell, settings persistence, VM definition persistence, typed configuration models, and runtime discovery diagnostics exist today. Real VM execution does not.

The current application does not start or stop QEMU, create virtual disks, integrate WHPX execution, manage networking, run AI models, or provide a production installer. Runtime status values are not evidence that a guest is running; newly created definitions default to `stopped`, and Start/Stop remain unavailable placeholders.

## Current Versus Future

| Area | Today | Future direction |
| --- | --- | --- |
| Configuration | Typed settings and VM definitions persist locally as JSON. | Versioned migrations and broader metadata. |
| Runtime | QEMU executable discovery and diagnostics only. | Controlled command construction and process lifecycle. |
| Virtualization | No guest process or hypervisor operation. | QEMU with WHPX where supported. |
| AI | Navigation placeholder only. | Provider-neutral models and approval-aware tools. |
| Safety | Path validation, bounded lookup, no shell execution. | Capability policy, audit events, approvals, and recovery. |

See [current status](current-status.md), [architecture](architecture.md), and the [roadmap](roadmap.md) for details.
