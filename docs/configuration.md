# Configuration

NexoraVM stores local configuration under the operating system's application-data directory selected through Tauri's path API. The repository does not hardcode user-specific absolute paths.

## Application Settings

The Settings page supports these fields:

| Setting | Default | Notes |
| --- | --- | --- |
| QEMU executable path | unset | Optional user-configured path used first during discovery. |
| Default VM storage path | unset | Stored preference only; storage provisioning is not implemented. |
| Default ISO path | unset | Stored preference only; no ISO validation against the filesystem occurs. |
| Default memory | `4096` MiB | Valid range is `512` to `262144` MiB. |
| Default CPU count | `2` | Valid range is `1` to `128`. |
| Preferred display mode | `windowed` | Supported values are `windowed`, `fullscreen`, and `headless`. |
| Start minimized | `false` | Preference is persisted; startup behavior is not otherwise expanded here. |
| Check for updates | `true` | Preference is persisted; update service integration is not implemented. |

Settings are stored in `settings.json`. Save validates values and paths. Reset writes the sensible defaults. Missing settings return defaults. Malformed or invalid stored settings recover to defaults rather than crashing the application.

## VM Definitions

VM definitions are stored separately in `vm-definitions.json` and contain typed configuration only. They do not persist process handles or runtime status. Live process state is held only by the Rust process manager.

Configuration includes name, operating system, CPU count, memory, disk size, disk path, ISO path, network mode, display mode, Secure Boot, and TPM. Validation rejects empty or oversized names, invalid IDs, unsupported resource ranges, syntactically invalid paths, and invalid disk sizes. ISO path existence and type are validated at launch time, not at definition save time. Disk path existence and type are validated at launch time and at disk creation time; they are not verified when a VM definition is saved. A disk size of `64` GiB is used by default when disk creation is requested, but existing disk images are never overwritten automatically. Those behaviors require later runtime validation and security review.

## Error Behavior

Commands return structured errors with a code, message, and optional field. Missing definitions return `not_found`; duplicate persisted IDs return `conflict`; persistence failures return `storage_error`; invalid input returns `validation_error`.

Malformed VM-definition JSON recovers to an empty list. Loaded definitions are then validated, and duplicate IDs are rejected. These recovery rules are intended to keep local corruption from causing a panic while still exposing an actionable result to the caller.

## What Configuration Does Not Do

Configuration does not start QEMU, create disks, enable WHPX, change networking, grant administrator privileges, run AI models, or verify that configured files exist. ISO path existence and type are validated at launch time, not at definition save time. Those behaviors require later, separately reviewed runtime and security work.
