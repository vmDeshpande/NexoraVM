# NexoraVM

> Virtual machines, operated by intelligence.

NexoraVM is an AI-native desktop virtualization platform designed to run real guest operating systems inside isolated virtual machines and let users interact with them through an integrated AI control layer.

The long-term goal is a single desktop application where users can:

- Create and manage virtual machines.
- Select and configure guest operating systems.
- Choose between supported local and cloud AI models.
- Give an AI controlled, auditable access to the guest environment.
- Observe, execute, verify, and recover from tasks performed inside the VM.
- Control isolation, networking, storage, resources, file transfer, and snapshots from one place.

## Project Status

**Phase 0 — Foundation**

The repository currently contains the architectural baseline and development documentation. No virtualization backend or AI-control runtime has been implemented yet.

## Design Principles

### 1. Desktop-first
NexoraVM is a complete desktop application, not an AI plugin layered on top of another virtualization product.

### 2. Real virtualization
The first implementation will use a mature virtualization backend rather than attempting to build a production hypervisor from scratch.

### 3. Model-agnostic AI
The architecture should support multiple local and cloud AI providers without coupling the platform to one model.

### 4. Explicit user control
AI capabilities inside a VM should be visible, configurable, auditable, and revocable.

### 5. Isolation by default
Host access, shared folders, networking, clipboard, and file transfer should default to restrictive settings and require explicit configuration.

### 6. Recoverable automation
AI actions should be observable and reversible through snapshots, checkpoints, action logs, and controlled recovery paths.

## Initial Architecture

```text
┌────────────────────────────────────────────────────────────┐
│                        NexoraVM Desktop                    │
├────────────────────────────────────────────────────────────┤
│ UI / Dashboard                                             │
│  ├── VM Manager                                             │
│  ├── VM Display                                             │
│  ├── AI Workspace                                           │
│  └── Settings / Security                                    │
├────────────────────────────────────────────────────────────┤
│ Application Core                                            │
│  ├── VM Lifecycle Manager                                   │
│  ├── AI Runtime                                             │
│  ├── Guest Communication                                    │
│  ├── Policy / Permissions                                   │
│  ├── Storage / Network Management                           │
│  └── Audit / Events                                         │
├────────────────────────────────────────────────────────────┤
│ Virtualization Abstraction                                  │
│  └── Hypervisor Backend                                     │
├────────────────────────────────────────────────────────────┤
│ Guest Environment                                           │
│  └── Nexora Guest Agent                                     │
└────────────────────────────────────────────────────────────┘
```

## Repository Layout

```text
NexoraVM/
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── SECURITY.md
├── docs/
│   ├── architecture/
│   ├── development/
│   └── security/
├── apps/
│   └── desktop/
├── crates/
│   ├── core/
│   ├── vm-runtime/
│   ├── guest-agent/
│   ├── ai-runtime/
│   └── security/
├── guest/
├── scripts/
└── tests/
```

Directories will be introduced as implementation begins; the structure is intentionally separated by responsibility so the desktop layer, VM runtime, AI runtime, guest integration, and security model can evolve independently.

## Non-Goals

At the beginning of development, NexoraVM will **not** attempt to:

- Implement a hypervisor from scratch.
- Support every host operating system simultaneously.
- Support every guest operating system simultaneously.
- Grant an AI unrestricted host-machine access.
- Claim that virtualization provides absolute security against every possible escape or host compromise.

## Development Roadmap

1. Repository and architecture foundation
2. Desktop shell and application lifecycle
3. VM backend abstraction
4. Create/start/stop/delete VM lifecycle
5. Guest display and input integration
6. Guest-agent communication
7. AI runtime and model-provider abstraction
8. AI observation/action/verification loop
9. Security policy and isolation controls
10. Snapshots, recovery, and audit logs
11. Packaging and Windows `.exe` distribution

See `docs/architecture/overview.md` for the current architecture and `docs/development/roadmap.md` for the development sequence.

## License

License terms will be finalized before the first public implementation release.
