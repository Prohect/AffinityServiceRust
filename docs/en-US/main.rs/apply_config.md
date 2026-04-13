# apply_config function (main.rs) — SPLIT

> **This function has been split into two separate functions:**
> - [`apply_config_process_level`](apply_config_process_level.md) — Applies process-level settings (one-shot per process): priority, affinity, CPU set, IO priority, memory priority.
> - [`apply_config_thread_level`](apply_config_thread_level.md) — Applies thread-level settings (every polling iteration): prime thread scheduling, ideal processor assignment, cycle time tracking.
>
> This split enables reactive process detection via ETW — process-level settings are applied immediately when a new process is detected, while thread-level settings continue on the regular polling schedule.

## See also

- [apply_config_process_level](apply_config_process_level.md)
- [apply_config_thread_level](apply_config_thread_level.md)
- [main.rs module overview](README.md)