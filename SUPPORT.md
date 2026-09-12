# Support

NexoraVM is an early-development project. Start with the documentation before opening an issue:

1. [README](README.md)
2. [Documentation index](docs/README.md)
3. [Current status](docs/current-status.md)
4. [Development guide](docs/development.md)
5. [Runtime diagnostics](docs/runtime.md)
6. [Security policy](SECURITY.md)

Search existing GitHub issues and discussions before opening a new request. Use the issue templates when they match your report.

## Bug Reports

Include:

- What you were trying to do.
- Reproduction steps.
- Expected and actual behavior.
- Operating system and relevant tool versions.
- The command that failed and its output.
- A minimal, redacted log excerpt.
- Screenshots only when they clarify a UI problem.

Remove secrets, credentials, tokens, personal information, and sensitive filesystem paths before sharing logs. Do not publicly report security-sensitive vulnerabilities; follow [SECURITY.md](SECURITY.md).

## Unsupported Requests

The project does not currently provide support for real QEMU startup/shutdown, WHPX execution, virtual disk creation, complete networking, AI inference, snapshots, or production packaging because those features are not implemented yet. Feature proposals are welcome through the issue template, but no response time or implementation date is promised.
