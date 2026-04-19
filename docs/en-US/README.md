# AffinityServiceRust Documentation (en-US)

AffinityServiceRust is a Windows service written in Rust that manages process CPU affinity, priority, IO priority, memory priority, and thread scheduling via an INI-like configuration file. It supports hot-reloading of configuration, ETW-based reactive process detection, prime thread scheduling with hysteresis, and tools for Process Lasso config conversion.

## Modules

| Module | Description |
|--------|-------------|
| [main.rs](main.rs/README.md) | Application entry point and main loop orchestrator |
| [cli.rs](cli.rs/README.md) | Command-line argument parsing and help output |
| [config.rs](config.rs/README.md) | Configuration file parsing, validation, hot-reload |
| [apply.rs](apply.rs/README.md) | Core enforcement engine — applies all settings to processes |
| [scheduler.rs](scheduler.rs/README.md) | Prime thread scheduler with hysteresis-based selection |
| [process.rs](process.rs/README.md) | Process snapshot via NtQuerySystemInformation |
| [winapi.rs](winapi.rs/README.md) | Windows API wrappers (handles, CPU sets, privileges) |
| [event_trace.rs](event_trace.rs/README.md) | ETW consumer for real-time process start/stop monitoring |
| [logging.rs](logging.rs/README.md) | Logging infrastructure with file rotation and error dedup |
| [priority.rs](priority.rs/README.md) | Type-safe enums for Windows priority levels |
| [collections.rs](collections.rs/README.md) | Performance-oriented collection type aliases and constants |
| [error_codes.rs](error_codes.rs/README.md) | Human-readable Win32/NTSTATUS error lookup |

## Architecture Overview

The service follows a loop-based enforcement architecture:

1. **Config parsing** — `config.rs` reads and validates the INI-like configuration file, producing a set of process rules.
2. **Main loop** — `main.rs` orchestrates the polling cycle and coordinates all subsystems.
3. **Process snapshot** — `process.rs` takes a system-wide process snapshot via `NtQuerySystemInformation`.
4. **Rule matching** — Each running process is matched against the loaded configuration rules.
5. **Apply enforcement** — `apply.rs` applies CPU affinity, priority, IO priority, memory priority, and thread scheduling settings to matched processes.
6. **Sleep / ETW wait** — The loop sleeps or waits for ETW process-start events from `event_trace.rs` before repeating.

Configuration hot-reload is supported: the service detects config file changes and re-parses without restart. The prime thread scheduler in `scheduler.rs` uses hysteresis to avoid excessive thread migration between cores.

## Locales

| Locale | Link |
|--------|------|
| en-US | (this page) |
| zh-CN | [zh-CN](../zh-CN/README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*