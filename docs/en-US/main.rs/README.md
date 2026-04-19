# main module (AffinityServiceRust)

The `main` module is the application entry point and top-level orchestrator of AffinityServiceRust. It parses CLI arguments, reads configuration, requests administrative privileges, and runs the main polling loop that takes process snapshots, matches them against configuration rules, and delegates to the [apply](../apply.rs/README.md) module for enforcement. It also manages ETW-based reactive sleep, hot-reloading of configuration and blacklist files, and a find-mode that discovers unmanaged processes.

## Functions

| Name | Description |
|------|-------------|
| [apply_process_level](apply_process_level.md) | Opens a process handle and applies all process-level settings (priority, affinity, CPU set, IO priority, memory priority). |
| [apply_thread_level](apply_thread_level.md) | Applies all thread-level settings (prime thread scheduling, ideal processor assignment, cycle time tracking). |
| [apply_config](apply_config.md) | Combined entry point that applies both process-level and thread-level configs for a matched process. |
| [log_apply_results](log_apply_results.md) | Formats and logs an [ApplyConfigResult](../apply.rs/ApplyConfigResult.md) with aligned multi-line output. |
| [process_logs](process_logs.md) | Post-processes find-mode log files to discover new unmanaged processes and locate their executables. |
| [process_find](process_find.md) | Takes a process snapshot and logs any process with default (full) CPU affinity not in config or blacklist. |
| [main](main.md) | Application entry point. Handles CLI modes, privilege elevation, ETW monitor, main loop, and graceful shutdown. |

## See Also

| Topic | Link |
|-------|------|
| Apply engine | [apply.rs](../apply.rs/README.md) |
| CLI argument parsing | [CliArgs](../cli.rs/CliArgs.md) |
| Configuration types | [ConfigResult](../config.rs/ConfigResult.md), [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md), [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md) |
| Prime thread scheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| Process snapshot | [ProcessSnapshot](../process.rs/ProcessSnapshot.md), [ProcessEntry](../process.rs/ProcessEntry.md) |
| ETW monitor | [EtwProcessMonitor](../event_trace.rs/EtwProcessMonitor.md) |
| Priority enums | [ProcessPriority](../priority.rs/ProcessPriority.md), [IOPriority](../priority.rs/IOPriority.md), [MemoryPriority](../priority.rs/MemoryPriority.md), [ThreadPriority](../priority.rs/ThreadPriority.md) |
| Win32 helpers | [winapi.rs](../winapi.rs/README.md) |
| Logging | [logging.rs](../logging.rs/README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*