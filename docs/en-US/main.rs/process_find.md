# process_find function (main.rs)

Scans all running processes to discover those not covered by configuration rules or the blacklist, logging them for review.

## Syntax

```rust
fn process_find(
    cli: &CliArgs,
    configs: &HashMap<u32, HashMap<String, ProcessConfig>>,
    blacklist: &[String],
) -> Result<(), windows::core::Error>
```

## Remarks

Only executes when `-find` mode is enabled. Uses the Win32 ToolHelp32 API (`CreateToolhelp32Snapshot`, `Process32FirstW/NextW`) to enumerate processes and checks each against configured rules and the blacklist. Unmanaged processes with default affinity are logged via [`log_process_find`](../logging.rs/log_process_find.md).

Previously inline in `main()`, now extracted as a standalone function.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/main.rs |
| **Source lines** | L207–L241 |
| **Called by** | [`main`](main.md) loop |

## See also

- [main.rs module overview](README.md)