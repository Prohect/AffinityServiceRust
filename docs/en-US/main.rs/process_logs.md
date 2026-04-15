# process_logs function (main.rs)

Scans `.find.log` files produced by the `-find` mode to discover processes that are not yet covered by any configuration grade or blacklist entry. For each unknown process, the function attempts to locate the executable on disk using [Everything search](https://www.voidtools.com/) (`es.exe`) and writes the aggregated results to a text file for manual review.

## Syntax

```rust
fn process_logs(
    configs: &ConfigResult,
    blacklist: &[String],
    logs_path: Option<&str>,
    output_file: Option<&str>,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `configs` | `&ConfigResult` | The fully-parsed configuration result. Used to determine which process names are already covered by any process-level or thread-level config grade. |
| `blacklist` | `&[String]` | Slice of lowercase process names that should be excluded from the results (known-uninteresting processes). |
| `logs_path` | `Option<&str>` | Directory path to scan for `.find.log` files. Defaults to `"logs"` when `None`. |
| `output_file` | `Option<&str>` | File path to write the discovery results to. Defaults to `"new_processes_results.txt"` when `None`. |

## Return value

This function does not return a value.

## Remarks

### Log parsing

The function reads every file in `logs_path` whose name ends with `.find.log`. For each line it searches for the substring `"find "` and extracts the token immediately following it (up to the next space). Only tokens ending with `.exe` are collected. All names are lowercased before comparison.

### Filtering

A discovered process name is **excluded** if any of the following are true:

- It appears as a key in any grade of `configs.process_level_configs`.
- It appears as a key in any grade of `configs.thread_level_configs`.
- It is present in the `blacklist` slice.

### Executable path resolution

For each remaining (new) process, the function shells out to the `es` command-line tool (Everything search) with the arguments `-utf8-bom -r ^<escaped_name>$`. The process name's dots are escaped for the regex. The command's stdout is decoded using the encoding that matches the current console output code page (`GetConsoleOutputCP`); code page 936 is mapped to `"gbk"`, and all others are mapped to `"windows-<cp>"`. A UTF-8 BOM prefix (`0xEF 0xBB 0xBF`) is stripped if present before decoding.

### Output format

Results are written as plain text. Each process entry has the form:

```
Process: <name>
Found:
  <path1>
  <path2>
---
```

If `es.exe` returns no output, `"Not found, result empty"` is printed instead. If `es.exe` fails entirely, `"Not found, es failed"` is printed.

### Side effects

- Sets the global console-logging flag (`*get_use_console!() = true`) so that any `log!` calls during execution are written to stdout.
- Writes the output file to disk via `std::fs::write`. Errors are logged but do not cause a panic.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `main.rs` |
| Callers | [main](main.md) (when `cli.process_logs_mode` is `true`) |
| Callees | `std::fs::read_dir`, `std::fs::read_to_string`, `std::fs::write`, `std::process::Command` (`es`), `GetConsoleOutputCP`, `encoding_rs::Encoding::for_label_no_replacement` |
| External tools | `es.exe` (Everything command-line interface) must be on `PATH` |
| Privileges | None beyond normal file-system access; does not require administrator privileges. |

## See Also

| Reference | Link |
|-----------|------|
| process_find | [process_find](process_find.md) |
| main | [main](main.md) |
| main module overview | [README](README.md) |
| config module | [config](../config.rs/README.md) |

---
> Commit SHA: `b0df9da35213b050501fab02c3020ad4dbd6c4e0`
