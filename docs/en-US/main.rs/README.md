# main module (AffinityServiceRust)

The `main` module is the entry point and top-level orchestrator for the AffinityServiceRust service. It declares all submodules, parses command-line arguments, loads configuration, manages the main polling loop, and coordinates the application of process-level and thread-level scheduling policies to running Windows processes. The module integrates ETW (Event Tracing for Windows) for reactive process detection, supports hot-reloading of configuration and blacklist files, and provides ancillary modes such as process discovery (`-find`), log analysis (`-processLogs`), config validation, and config conversion.

## Functions

| Function | Description |
|----------|-------------|
| [apply_process_level](apply_process_level.md) | Applies one-shot process-level settings (priority, affinity, CPU set, IO priority, memory priority) to a single process. |
| [apply_thread_level](apply_thread_level.md) | Applies per-iteration thread-level settings (prime thread scheduling, ideal processor assignment, cycle tracking) to a single process. |
| [apply_config](apply_config.md) | Orchestrates both process-level and thread-level configuration application for a matched process, merging results and logging. |
| [log_apply_results](log_apply_results.md) | Formats and emits log output for the changes and errors produced by a single `apply_config` invocation. |
| [process_logs](process_logs.md) | Scans `.find.log` files to discover previously-unknown processes, resolves their executable paths via Everything search (`es.exe`), and writes results to a file. |
| [process_find](process_find.md) | Enumerates running processes with the Toolhelp API and logs any that are not yet covered by configuration or blacklist. |
| [main](main.md) | Program entry point. Parses CLI arguments, loads configuration, acquires privileges, starts the ETW monitor, and runs the main polling/apply loop. |

## Structs / Enums

This module does not define any public structs or enums. All data types used in this module are imported from sibling modules such as `config`, `apply`, `scheduler`, and `cli`.

## See Also

| Related module | Link |
|----------------|------|
| scheduler | [scheduler module](../scheduler.rs/README.md) |
| priority | [priority module](../priority.rs/README.md) |
| apply | [apply module](../apply.rs/README.md) |
| config | [config module](../config.rs/README.md) |
| cli | [cli module](../cli.rs/README.md) |
| logging | [logging module](../logging.rs/README.md) |
| event_trace | [event_trace module](../event_trace.rs/README.md) |
| winapi | [winapi module](../winapi.rs/README.md) |
| process | [process module](../process.rs/README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
