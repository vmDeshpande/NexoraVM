# Development Roadmap

## Phase 0 — Foundation

- [x] Create repository
- [x] Define product direction
- [x] Define architecture boundaries
- [x] Define initial security principles
- [ ] Choose desktop framework
- [ ] Choose first virtualization backend
- [ ] Choose implementation language split

## Phase 1 — Desktop Shell

- [ ] Create desktop application
- [ ] Add application navigation
- [ ] Add persistent settings
- [ ] Add structured logging
- [ ] Add error reporting surface

## Phase 2 — VM Runtime

- [ ] Define hypervisor adapter interface
- [ ] Detect host capabilities
- [ ] Create VM configuration model
- [ ] Create/start/stop/pause/delete VM
- [ ] Manage VM disks and metadata
- [ ] Expose VM state to the desktop UI

## Phase 3 — Guest Interaction

- [ ] Display guest screen
- [ ] Forward keyboard and mouse input
- [ ] Define guest-agent protocol
- [ ] Build initial guest agent
- [ ] Add controlled command execution
- [ ] Add guest health and capability reporting

## Phase 4 — AI Runtime

- [ ] Define model-provider interface
- [ ] Add local-model provider
- [ ] Add cloud-model provider
- [ ] Define observation schema
- [ ] Define action schema
- [ ] Implement action validation
- [ ] Implement execution and verification loop

## Phase 5 — Security and Recovery

- [ ] Capability-based policy engine
- [ ] Approval workflow for sensitive actions
- [ ] Network controls
- [ ] Host filesystem and clipboard controls
- [ ] Snapshot and restore workflow
- [ ] Immutable audit events
- [ ] Threat model and security review

## Phase 6 — Packaging

- [ ] Windows installer
- [ ] Hardware acceleration checks
- [ ] First-run setup
- [ ] Crash-safe state recovery
- [ ] Documentation and examples
- [ ] Reproducible release process
