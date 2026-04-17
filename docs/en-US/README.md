# AffinityServiceRust Documentation (en-US)

AffinityServiceRust is a high-performance Windows process management service written in Rust. It continuously monitors running processes and applies CPU affinity, priority, I/O priority, and memory priority rules defined in configuration files. It targets both legacy systems (≤64 logical processors via affinity masks) and modern many-core systems (via CPU Sets across processor groups).

The service operates in a polling loop or ETW-reactive mode, matching each live process against user-defined rules and pushing scheduling policy changes through the Windows kernel API. A prime-thread scheduler dynamically identifies CPU-bound threads each interval and pins them to designated high-performance cores.

## Modules

| Module | Overview | Items |
|--------|----------|-------|
| [apply.rs](apply.rs/README.md) | Process/thread settings application | 16 |
| [cli.rs](cli.rs/README.md) | Command-line argument parsing | 7 |
| [collections.rs](collections.rs/README.md) | Type aliases and capacity constants | 8 |
| [config.rs](config.rs/README.md) | Configuration file parsing and hot-reload | 25 |
| [error_codes.rs](error_codes.rs/README.md) | Win32/NTSTATUS error formatting | 2 |
| [event_trace.rs](event_trace.rs/README.md) | ETW process start/stop monitoring | 4 |
| [logging.rs](logging.rs/README.md) | Logging, error deduplication, find output | 10 |
| [main.rs](main.rs/README.md) | Entry point and main polling loop | 7 |
| [priority.rs](priority.rs/README.md) | Priority enums (process, IO, memory, thread) | 5 |
| [process.rs](process.rs/README.md) | Process snapshot via NtQuerySystemInformation | 4 |
| [scheduler.rs](scheduler.rs/README.md) | Prime thread scheduler and stats tracking | 6 |
| [winapi.rs](winapi.rs/README.md) | Windows API wrappers and handle management | 27 |

## Architecture Overview

The service is structured around a central polling loop in `main.rs`. Each iteration:

1. **Snapshot** — `process.rs` calls `NtQuerySystemInformation` to obtain the current list of processes and threads.
2. **Match** — `config.rs` supplies the parsed rule set; each live process is matched by name against defined rules.
3. **Apply** — `apply.rs` dispatches the matched rule's settings (priority, affinity/CPU sets, I/O priority, memory priority, ideal processors, prime-thread scheduling) to each process and its threads.
4. **Schedule** — `scheduler.rs` tracks per-thread CPU cycle deltas across intervals to identify the busiest ("prime") threads and assign them to configured prime cores.
5. **React** — `event_trace.rs` optionally drives the loop via ETW process-start events instead of a fixed timer, reducing latency for newly launched processes.

## Key Concepts

| Concept | Where |
|---------|-------|
| CPU affinity mask (≤64 cores) | [apply_affinity](apply.rs/apply_affinity.md) |
| CPU Sets (>64 cores, cross-group) | [apply_process_default_cpuset](apply.rs/apply_process_default_cpuset.md) |
| Prime thread scheduling | [apply_prime_threads](apply.rs/apply_prime_threads.md), [scheduler.rs](scheduler.rs/README.md) |
| Ideal processor assignment | [apply_ideal_processors](apply.rs/apply_ideal_processors.md) |
| Hot-reload | [config.rs](config.rs/README.md) |
| Rule grades | [ProcessLevelConfig](config.rs/ProcessLevelConfig.md) |

## See Also

| Resource | Link |
|----------|------|
| Project README | [../../README.md](../../README.md) |
| Docs index | [../README.md](../README.md) |
| zh-CN locale | [../../docs/zh-CN/README.md](../zh-CN/README.md) |

*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*