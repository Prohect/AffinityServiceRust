# main module (AffinityServiceRust)

The `main` module is the entry point and top-level orchestrator for AffinityServiceRust. It wires together CLI parsing, configuration loading, process snapshot enumeration, and the core apply loop that enforces process-level and thread-level settings on managed processes. The module also provides utility entry points for log processing and unmanaged-process discovery.

The main polling loop takes a periodic process snapshot via `NtQuerySystemInformation`, matches running processes against the loaded configuration rules, and delegates to the `apply` module for the actual Win32/NT API calls. Process-level settings (priority, affinity, CPU set, IO priority, memory priority) are applied once per process lifetime. Thread-level settings (prime thread scheduling, ideal processor assignment) are re-evaluated every polling iteration. An optional ETW-based real-time process monitor supplements the polling approach for faster reaction to new process starts.

## Functions

| Function | Description |
|----------|-------------|
| [apply_config_process_level](apply_config_process_level.md) | Applies one-shot process-level settings: priority class, CPU affinity, CPU set, IO priority, and memory priority. |
| [apply_config_thread_level](apply_config_thread_level.md) | Applies per-iteration thread-level settings: prime thread scheduling, ideal processor assignment, and cycle-time tracking. |
| [process_logs](process_logs.md) | Processes `.find.log` files from `-find` mode to discover new unmanaged processes and resolve their executable paths. |
| [process_find](process_find.md) | Enumerates running processes via `CreateToolhelp32Snapshot` and logs those with default (unmanaged) affinity. |
| [main](main.md) | Program entry point. Parses CLI arguments, loads configuration, manages the polling loop, ETW integration, hot-reload, and grade-based scheduling. |

## See Also

| Topic | Link |
|-------|------|
| CLI argument parsing | [cli.rs](../cli.rs/README.md) |
| Configuration file parsing | [config.rs](../config.rs/README.md) |
| Apply functions (process & thread level) | [apply.rs](../apply.rs/README.md) |
| Prime thread scheduler | [scheduler.rs](../scheduler.rs/README.md) |
| ETW process monitor | [event_trace.rs](../event_trace.rs/README.md) |
| Process snapshot | [process.rs](../process.rs/README.md) |
| Win32/NT API wrappers | [winapi.rs](../winapi.rs/README.md) |
| Logging infrastructure | [logging.rs](../logging.rs/README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd