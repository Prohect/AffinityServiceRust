# read_list function (config.rs)

Reads a simple line-per-entry list file and returns a vector of trimmed, lowercased, non-empty, non-comment strings. This function is used to load the blacklist file, which contains process names that the service should skip during rule application.

## Syntax

```rust
pub fn read_list<P: AsRef<Path>>(path: P) -> Result<Vec<String>>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `path` | `P: AsRef<Path>` | File system path to the list file. Accepts any type that implements `AsRef<Path>`, including `&str`, `String`, and `PathBuf`. The file is opened with `File::open` and read line-by-line via a buffered reader. |

## Return value

Returns `Result<Vec<String>>`:

- **`Ok(Vec<String>)`** — A vector of lowercased, trimmed process names extracted from the file. The vector preserves the order of entries as they appear in the file, with blank lines and comment lines removed.
- **`Err(io::Error)`** — Propagated from `File::open` if the file cannot be opened (e.g., file not found, permission denied).

## Remarks

### Parsing rules

For each line in the file:

1. The line is trimmed of leading and trailing whitespace.
2. The trimmed line is converted to lowercase via `to_lowercase()`.
3. The line is **skipped** if:
   - It is empty after trimming.
   - It starts with `#` (treated as a comment).
4. All surviving lines are collected into the returned vector.

### File format

The list file is a plain-text file with one entry per line. Comments and blank lines are supported for readability:

```text
# Processes to skip
system
svchost.exe
csrss.exe

# Development tools
devenv.exe
code.exe
```

### Case normalization

All entries are lowercased to ensure case-insensitive matching when the service compares running process names against the blacklist. Windows process names are case-insensitive, so this normalization is consistent with the rest of the configuration system.

### Encoding

The file is read using Rust's default UTF-8 file I/O. Lines that contain invalid UTF-8 are silently dropped by the `map_while(Result::ok)` iterator adapter — no error is raised for individual malformed lines.

### Difference from read_config

Unlike [read_config](read_config.md), which handles a complex multi-format configuration language, `read_list` performs simple line-by-line extraction with no special syntax for aliases, constants, groups, or rule fields. Each line is either a process name or a comment.

### Blacklist usage

The primary consumer of `read_list` is the service main loop, which loads the blacklist at startup and via [hotreload_blacklist](hotreload_blacklist.md). Processes whose lowercased names appear in the blacklist vector are skipped during the apply phase, regardless of any matching rules in the configuration.

### Error propagation

The `?` operator on `File::open` means that file-not-found or permission errors are propagated to the caller as `io::Error`. The caller is responsible for handling this — for example, [hotreload_blacklist](hotreload_blacklist.md) uses `unwrap_or_default()` to fall back to an empty blacklist on error.

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Visibility** | `pub` |
| **Callers** | [main](../main.rs/main.md), [hotreload_blacklist](hotreload_blacklist.md) |
| **Callees** | `std::fs::File::open`, `std::io::BufReader`, `std::io::BufRead::lines` |
| **API** | Standard library file I/O only — no Windows API calls |
| **Privileges** | Read access to the list file path |

## See Also

| Topic | Link |
|-------|------|
| Main config file reader | [read_config](read_config.md) |
| Blacklist hot-reload | [hotreload_blacklist](hotreload_blacklist.md) |
| UTF-16 LE file reader | [read_utf16le_file](read_utf16le_file.md) |
| Config module overview | [README](README.md) |