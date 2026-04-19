# read_bleack_list function (config.rs)

Reads a blacklist file containing process names to exclude from configuration enforcement. Each line in the file specifies one process name; lines starting with `#` are treated as comments and ignored.

> **Note:** The function name preserves the original spelling `read_bleack_list` as it appears in the source code.

## Syntax

```rust
pub fn read_bleack_list<P: AsRef<Path>>(path: P) -> Result<Vec<String>>
```

## Parameters

`path: P`

File system path to the blacklist file. Accepts any type that implements `AsRef<Path>`, such as `&str`, `String`, or `PathBuf`. The file is expected to be a UTF-8 text file with one process name per line.

## Return value

`Result<Vec<String>>` — On success, returns a `Vec<String>` of lowercased process names. On failure (e.g., file not found, permission denied), returns the underlying `std::io::Error`.

## Remarks

### File format

The blacklist file uses a simple line-oriented format:

```
# This is a comment
svchost.exe
explorer.exe
# Another comment
taskmgr.exe
```

### Parsing rules

1. The file is opened and read line-by-line using a buffered reader.
2. Each line is trimmed of leading and trailing whitespace.
3. The line is converted to lowercase for case-insensitive matching against running process names.
4. Empty lines and lines starting with `#` are discarded.
5. All surviving lines are collected into a `Vec<String>`.

After successful loading, the function logs the count of loaded blacklist items via the `log!` macro (e.g., `"12 blacklist items loaded"`).

### Relationship with hot-reload

This function is called by [hotreload_blacklist](hotreload_blacklist.md) whenever the blacklist file's modification timestamp changes. On read failure during hot-reload, the previous blacklist is retained via `unwrap_or_default()` at the call site.

### Effect on service behavior

Processes whose lowercased names appear in the blacklist are skipped entirely during the apply loop. No process-level or thread-level configuration is applied to blacklisted processes, even if matching rules exist in the config file.

### Error handling

The function propagates `std::io::Error` from `File::open` if the file cannot be opened. Line-reading errors cause the iterator to stop at the first failure (via `map_while(Result::ok)`), but lines read successfully up to that point are still included in the result.

## Requirements

| | |
|---|---|
| **Module** | [`src/config.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/config.rs) |
| **Callers** | [hotreload_blacklist](hotreload_blacklist.md), main initialization in `main.rs` |
| **Callees** | `std::fs::File::open`, `std::io::BufReader`, `log!` macro |
| **Privileges** | File read access to the blacklist path |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [config.rs](README.md) |
| Hot-reload mechanism for blacklist | [hotreload_blacklist](hotreload_blacklist.md) |
| Config file reader (counterpart) | [read_config](read_config.md) |
| CLI arguments specifying blacklist path | [CliArgs](../cli.rs/CliArgs.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*