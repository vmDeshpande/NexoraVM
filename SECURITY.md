# Security Policy

NexoraVM handles virtualization configuration and controlled host processes. These are security-sensitive operations. QEMU launch is limited to validated command specifications; complete VM runtime integration remains deferred.

## Current Security Boundaries

- No arbitrary shell execution.
- No unrestricted command strings or user-provided process argument lists.
- Runtime discovery uses fixed executable names, bounded lookup locations, and the fixed `--version` argument.
- QEMU command previews use a typed executable path and ordered arguments; they do not expose shell strings or arbitrary flags.
- Paths and VM resources are validated before persistence.
- ISO paths are validated as existing regular files before QEMU launch; directories and missing paths are rejected with structured errors.
- No administrator privileges are requested.
- No Windows features, firewall rules, or networking are modified.
- VM configuration is separate from observed runtime state.
- Sensitive filesystem paths are not intentionally logged as diagnostics.
- AI inference and AI tool permissions are not implemented.

## Threat Model

Relevant risks include command injection, unsafe path composition, malformed persisted data, excessive host access, untrusted guest behavior, privilege escalation, dependency compromise, and future AI actions that exceed user intent.

QEMU process management now uses direct process APIs, separate executable/argument fields, bounded output buffers, duplicate-start protection, stop timeouts, forced termination fallback, and in-memory handles. It still requires deeper process isolation, cancellation policy, cleanup review, and truthful guest-state integration before it is considered complete runtime support. Future AI tools must use explicit capabilities, policy checks, user approval for destructive actions, auditability, and recovery behavior.

The manager never scans for or adopts unrelated processes. Live ownership ends when the application exits; restart does not attempt unsafe process rediscovery or termination.

The current command-construction layer is intentionally preview-only. It validates configuration, rejects shell-like VM names and unsafe paths, rejects unsupported bridged networking, and never starts QEMU or creates disks.

## Reporting a Vulnerability

A formal private security reporting channel has not yet been configured. Do not disclose sensitive vulnerability details in a public issue. Until a dedicated channel exists, contact repository maintainers through an available private GitHub mechanism if one is available, and include only the minimum information needed to reproduce the issue.

For non-sensitive hardening ideas, use a regular issue after searching existing discussions. Never include secrets, credentials, private paths, or personal data in issues or pull requests.

## Dependency and Supply-Chain Concerns

Dependency changes should be focused and reviewed. Future release work should add dependency auditing, signed artifacts, reproducible builds, and update-integrity checks before broad distribution.
