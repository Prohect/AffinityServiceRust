# purge_fail_map function (logging.rs) — REMOVED

> **This function has been removed.** Process exit cleanup is now handled reactively via ETW events in the main loop. When a process stop event is received from [`EtwProcessMonitor`](../event_trace.rs/EtwProcessMonitor.md), the main loop directly removes the PID's entries from [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) and calls [`drop_process_by_pid`](../scheduler.rs/PrimeThreadScheduler.md) on the scheduler.

## See also

- [event_trace.rs module overview](../event_trace.rs/README.md) — reactive process monitoring
- [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md)
- [logging.rs module overview](README.md)