# process_logs function (main.rs)

Processes `.find.log` files to discover the full executable paths of previously found process names, using `es.exe` (Everything search) as the lookup engine.

## Syntax

```rust
pub fn process_logs(
    configs: &HashMap<u32, HashMap<String, ProcessConfig>>,
    blacklist: &[String],
    logs_path: Option<&str>,
    output_file: Option<&str>,
)
```

## Parameters

`configs`

Reference to the parsed configuration map (grade → process name → [`ProcessConfig`](../config.rs/ProcessConfig.md)). Used to determine which process names are relevant — only names that appear in the configuration are looked up.

`blacklist`

A slice of process names that should be excluded from the lookup. Blacklisted process names are skipped even if they appear in the find log.

`logs_path`

Optional path to the directory containing `.find.log` files. When `None`, the function uses the default log directory (adjacent to the executable).

`output_file`

Optional path to an output file where the results will be written. When `None`, results are written to stdout.

## Return value

This function does not return a value. Results are written to the specified output file or to stdout.

## Remarks

The `process_logs` function implements a post-processing workflow for the process discovery data collected during normal operation or find mode. The workflow is:

1. **Read find logs** — the function reads `.find.log` files from the specified (or default) directory. These files contain process names that were discovered during previous runs, written by [`log_process_find`](../logging.rs/log_process_find.md).

2. **Filter** — process names that appear in the `blacklist` are removed. Only names that match entries in the `configs` map are retained, ensuring that the lookup is scoped to processes the user actually cares about.

3. **Lookup via es.exe** — for each remaining process name, the function invokes `es.exe` (the command-line interface to [Everything](https://www.voidtools.com/)), a fast file search utility for Windows. The query searches for executable files matching the process name, returning their full file system paths.

4. **Output** — the results (process name → full path mappings) are written to the output file or stdout in a human-readable format. This allows the user to see where each discovered process's executable resides on disk.

### Prerequisites

- **Everything** must be installed and its `es.exe` command-line tool must be available in the system PATH or at a known location. If `es.exe` is not found, the function will fail to resolve any paths.
- The Everything service must be running and have an up-to-date index for the lookup to return results.

### Use case

This function is primarily intended for users who want to understand which executables correspond to the process names in their configuration. By running the application in find mode first (which populates the `.find.log`), and then running with `--process-logs`, the user gets a complete mapping of configured process names to their on-disk locations.

### Interaction with find mode

The typical workflow is:

1. Run with `--find` flag to populate the `.find.log` with discovered process names via [`log_process_find`](../logging.rs/log_process_find.md).
2. Run with `--process-logs` to look up full paths for those process names.
3. Review the output to verify that the correct executables are being targeted by the configuration.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/main.rs |
| **Source lines** | L106–L194 |
| **Called by** | [`main`](main.md) when `process_logs_mode` is `true` |
| **Reads** | `.find.log` files produced by [`log_process_find`](../logging.rs/log_process_find.md) |
| **External dependency** | `es.exe` ([Everything](https://www.voidtools.com/) command-line search) |

## See also

- [main function](main.md)
- [CliArgs](../cli.rs/CliArgs.md) (`process_logs_mode`, `out_file_name`)
- [log_process_find](../logging.rs/log_process_find.md)
- [FIND_LOG_FILE](../logging.rs/FIND_LOG_FILE.md)
- [main.rs module overview](README.md)