# Security Policy

NexoraVM handles virtualization configuration and will eventually manage host processes. These are security-sensitive operations. The project is intentionally deferring real QEMU execution until command construction, validation, process policy, and testing are ready.

## Current Security Boundaries

- No arbitrary shell execution.
- No unrestricted command strings or user-provided process argument lists.
- Runtime discovery uses fixed executable names, bounded lookup locations, and the fixed `--version` argument.
- QEMU command previews use a typed executable path and ordered arguments; they do not expose shell strings or arbitrary flags.
- Paths and VM resources are validated before persistence.
- No administrator privileges are requested.
- No Windows features, firewall rules, or networking are modified.
- VM configuration is separate from observed runtime state.
- Sensitive filesystem paths are not intentionally logged as diagnostics.
- AI inference and AI tool permissions are not implemented.

## Threat Model

Relevant risks include command injection, unsafe path composition, malformed persisted data, excessive host access, untrusted guest behavior, privilege escalation, dependency compromise, and future AI actions that exceed user intent.

Future QEMU process management must address typed command construction, process isolation, least privilege, output and resource limits, timeouts, cancellation, cleanup, and truthful runtime-state updates. Future AI tools must use explicit capabilities, policy checks, user approval for destructive actions, auditability, and recovery behavior.

The current command-construction layer is intentionally preview-only. It validates configuration, rejects shell-like VM names and unsafe paths, rejects unsupported bridged networking, and never starts QEMU or creates disks.

## Reporting a Vulnerability

A formal private security reporting channel has not yet been configured. Do not disclose sensitive vulnerability details in a public issue. Until a dedicated channel exists, contact repository maintainers through an available private GitHub mechanism if one is available, and include only the minimum information needed to reproduce the issue.

For non-sensitive hardening ideas, use a regular issue after searching existing discussions. Never include secrets, credentials, private paths, or personal data in issues or pull requests.

## Dependency and Supply-Chain Concerns

Dependency changes should be focused and reviewed. Future release work should add dependency auditing, signed artifacts, reproducible builds, and update-integrity checks before broad distribution.
