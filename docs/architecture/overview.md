# NexoraVM Architecture Overview

## Core Boundary

NexoraVM is divided into five major boundaries:

1. **Desktop application** — user interface, lifecycle, settings, and visual VM interaction.
2. **Application core** — orchestration, state, events, policies, and persistence.
3. **Virtualization runtime** — adapter over a mature hypervisor backend.
4. **AI runtime** — provider abstraction, planning, tool execution, verification, and recovery.
5. **Guest integration** — an agent running inside the guest OS for controlled communication.

## Initial Runtime Flow

```text
User request
    ↓
AI runtime interprets task
    ↓
Policy engine checks requested capabilities
    ↓
Guest observation is collected
    ↓
AI selects an allowed action
    ↓
Action is executed through guest integration
    ↓
Result is observed and verified
    ↓
Event is recorded and user is shown the result
```

## Security Boundary

The AI should not directly receive unrestricted host APIs. It should operate through a narrow capability interface. Each capability must define:

- What it can access.
- Whether user approval is required.
- Whether it is reversible.
- What gets logged.
- What happens on failure.

The first implementation should default to:

- No host filesystem mounts.
- Restricted or disabled guest networking.
- No host secret exposure.
- Explicit clipboard and file-transfer controls.
- Resource limits.
- Snapshot support before high-impact operations.

## Backend Strategy

The virtualization layer will be an abstraction. The first backend will be selected after validating Windows host support, guest display integration, hardware acceleration, licensing, and automation APIs. NexoraVM will not implement a hypervisor from scratch in the initial project phase.
