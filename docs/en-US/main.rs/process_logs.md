# process_logs function (main.rs)

Processes find-mode log files to discover new unmanaged processes. Reads all `.find.log` files from a logs directory, extracts unique process names, filters out processes already present in the configuration or blacklist, and uses [Everything search](https://www.voidtools.com/) (`es.exe`) to locate executable paths on disk. Results are written to a text file for manual review.

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

`configs: &ConfigResult`

The parsed [ConfigResult](../config.rs/ConfigResult.md) containing all process-level and thread-level configuration maps. Used to determine which processes are already managed.

`blacklist: &[String]`

A list of lowercase process names that should be excluded from the results, in addition to those already in `configs`.

`logs_path: Option<&str>`

Path to the directory containing `.find.log` files. Defaults to `"logs"` when `None`.

`output_file: Option<&str>`

Path to the output text file where results are written. Defaults to `"new_processes_results.txt"` when `None`.

## Return value

This function does not return a value. Output is written to the file specified by `output_file`.

## Remarks

The function implements a multi-stage pipeline:

1. **Log scanning** — Iterates all files in `logs_path` whose names end with `.find.log`. For each file, it parses lines looking for the pattern `"find <process_name>"` and extracts process names ending in `.exe`. All names are lowercased and collected into a `HashSet` for deduplication.

2. **Filtering** — Removes any process name that appears in any grade of `configs.process_level_configs` or `configs.thread_level_configs`, as well as any name present in the `blacklist`.

3. **Executable location** — For each remaining process name, invokes the Everything command-line interface (`es.exe`) with `-utf8-bom -r ^<escaped_name>$` to perform a regex search for the exact filename. The `.` characters in the process name are escaped to `\.` for correct regex matching.

4. **Encoding handling** — Detects the console output code page via `GetConsoleOutputCP` and selects the appropriate encoding (GBK for code page 936, otherwise `windows-<codepage>`). The `es.exe` output is decoded from this encoding. A UTF-8 BOM prefix (`0xEF 0xBB 0xBF`) is stripped if present.

5. **Output formatting** — Each process is written as a block:
   - `Process: <name>` header
   - `Found:` section with indented paths, or `Not found, result empty` / `Not found, es failed`
   - `---` separator

### Side effects

- Sets the global `use_console` flag to `true` (forces console output mode).
- Spawns external `es.exe` processes. If Everything search is not installed or `es.exe` is not on `PATH`, all lookups will report failure.

### Encoding edge cases

The function handles the mismatch between `es.exe` output encoding and Rust's UTF-8 strings by using the `encoding_rs` crate. On Chinese-locale systems (code page 936), GBK decoding is used; on other locales, the appropriate Windows code page encoding is selected.

## Requirements

| | |
|---|---|
| **Module** | `src/main.rs` |
| **Callers** | [main](main.md) (when `cli.process_logs_mode` is `true`) |
| **Callees** | `std::fs::read_dir`, `std::fs::read_to_string`, `std::fs::write`, `std::process::Command` (`es.exe`), `GetConsoleOutputCP`, `encoding_rs::Encoding::for_label_no_replacement` |
| **External tools** | [Everything CLI (`es.exe`)](https://www.voidtools.com/support/everything/command_line_interface/) |
| **Privileges** | None beyond file system read/write access |

## See Also

| Topic | Link |
|-------|------|
| Find-mode runtime discovery | [process_find](process_find.md) |
| Configuration types | [ConfigResult](../config.rs/ConfigResult.md) |
| Entry point and mode dispatch | [main](main.md) |
| Logging infrastructure | [logging.rs](../logging.rs/README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*