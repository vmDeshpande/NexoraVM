# ADR 0001 — Initial Technology Direction

## Status

Accepted for the initial implementation.

## Context

NexoraVM is intended to become a single desktop application that manages real guest operating systems, provides an AI control layer, and keeps virtualization and AI providers behind explicit abstractions.

The repository is currently a documentation-only Phase 0 foundation. The implementation needs a concrete direction without locking the project to a single hypervisor or AI provider.

## Decision

### Desktop application

Use **Tauri 2 + Rust + a web UI** for the desktop shell.

Rust will own the privileged/native application core. The UI will remain a presentation layer that communicates with the Rust core through narrow commands/events.

### Core implementation language

Use **Rust as the primary systems language** for the application core, virtualization integration, security/policy layer, eventing, persistence, and guest communication.

### Virtualization strategy

Define a **NexoraVM virtualization adapter** and make the first backend **QEMU**, using **Windows Hypervisor Platform (WHPX)** acceleration on supported Windows hosts.

The adapter must prevent QEMU-specific types and command construction from leaking into the rest of the application.

### Host target for the first implementation

Target **64-bit Windows 11 first**. The architecture should leave room for future host backends/platforms, but cross-platform support is not a Phase 1 requirement.

### AI runtime

Define a provider-neutral AI interface from the beginning. The first implementation should support a local model provider before adding cloud providers, while keeping model selection and credentials outside the VM orchestration layer.

### Security model

Treat guest control as a capability system. The AI may request capabilities, but policy decides whether each capability is allowed, requires approval, is reversible, and is logged.

## Consequences

### Positive

- Rust gives the core direct access to Windows and virtualization APIs while providing strong memory and concurrency safety.
- Tauri keeps the desktop UI lightweight while allowing a modern web UI.
- QEMU is a mature virtualization stack and can use WHPX for hardware acceleration on Windows.
- The virtualization adapter preserves the ability to add another backend later.
- The AI provider abstraction prevents the product from becoming coupled to a single model vendor.

### Trade-offs

- QEMU integration requires careful process lifecycle, argument construction, device configuration, and display/input integration.
- Tauri introduces a Rust/web boundary that must be kept narrow and well-typed.
- Windows-first development means some future portability work will be deferred.

## Initial implementation order

1. Create the Tauri/Rust workspace and desktop shell.
2. Define shared domain models and application events.
3. Define the virtualization adapter trait and a safe QEMU backend boundary.
4. Add host capability detection for Windows virtualization prerequisites.
5. Implement VM configuration and lifecycle management against the backend.
6. Add the first desktop VM dashboard.

## Revisit triggers

Reconsider this decision if:

- QEMU/WHPX cannot provide the required guest display/input behavior reliably.
- A different backend materially reduces complexity while preserving the required isolation model.
- Linux or macOS becomes an immediate first-class host target rather than a later platform.
- The Tauri/Rust boundary becomes a significant bottleneck for product development.
