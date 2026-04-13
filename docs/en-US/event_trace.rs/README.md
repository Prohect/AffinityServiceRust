# event_trace.rs Module

Minimal ETW (Event Tracing for Windows) consumer for real-time process start/stop monitoring.

Uses the Microsoft-Windows-Kernel-Process provider to receive notifications when processes are created or terminated, enabling reactive rule application instead of relying solely on polling.

## Overview

This module provides an ETW-based process monitor that listens for process start and stop events from the Windows kernel. When a new process starts, its PID is sent through a channel so the main loop can immediately apply process-level rules without waiting for the next polling interval. When a process stops, its PID is used to clean up scheduler state and error deduplication entries.

The monitor runs on a background thread and communicates with the main loop via an `mpsc` channel.

## Items

### Statics

| Name | Description |
| --- | --- |
| [ETW_SENDER](ETW_SENDER.md) | Global sender for the ETW callback to dispatch process events through the channel. |
| [ETW_ACTIVE](ETW_ACTIVE.md) | Atomic flag indicating whether the ETW session is currently active. |

### Structs

| Name | Description |
| --- | --- |
| [EtwProcessEvent](EtwProcessEvent.md) | A process start/stop event received from ETW, containing the PID and event type. |
| [EtwProcessMonitor](EtwProcessMonitor.md) | Manages an ETW real-time trace session for process monitoring, including session lifecycle and background processing thread. |

## Architecture

### ETW Session Lifecycle

1. **Start** — `EtwProcessMonitor::start()` creates an ETW real-time session named `"AffinityServiceRust_EtwProcessMonitor"`, enables the `Microsoft-Windows-Kernel-Process` provider with `WINEVENT_KEYWORD_PROCESS`, and spawns a background thread running `ProcessTrace`.
2. **Callback** — The `etw_event_callback` extern function receives each event record from the OS. It extracts the PID from `UserData` and sends an `EtwProcessEvent` through the global `ETW_SENDER` channel.
3. **Consume** — The main loop calls `rx.try_recv()` to drain pending events each iteration. Start events add PIDs to `process_level_pending`; stop events clean up scheduler and error tracking state.
4. **Stop** — `EtwProcessMonitor::stop()` closes the trace, stops the session, joins the background thread, and clears the global sender. Also called automatically via `Drop`.

### Integration with Main Loop

- **Process start**: PID added to `process_level_pending` set. On the next snapshot, process-level rules are applied immediately regardless of grade scheduling.
- **Process stop**: PID removed from `process_level_applied`, `process_level_pending`, `PID_MAP_FAIL_ENTRY_SET`, and scheduler stats via `drop_process_by_pid`.

### Fallback

If ETW initialization fails (e.g., insufficient privileges), the service falls back to polling-only mode with a warning log. All functionality continues to work, just without reactive process detection.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/event_trace.rs` |
| **Windows features** | `Win32_System_Diagnostics_Etw` |
| **Called by** | [`main`](../main.rs/main.md) |
| **Key dependencies** | `windows` crate ETW APIs, `once_cell::sync::Lazy`, `std::sync::mpsc` |

## See also

- [main.rs module overview](../main.rs/README.md) — integrates ETW events into the main loop
- [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) — `drop_process_by_pid` cleans up on process exit
- [PID_MAP_FAIL_ENTRY_SET](../logging.rs/PID_MAP_FAIL_ENTRY_SET.md) — error entries cleaned up on process exit