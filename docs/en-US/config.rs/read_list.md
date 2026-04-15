# read_list function (config.rs)

Reads a text file and returns its non-empty, non-comment lines as a vector of lowercase strings. This utility function is used to load simple line-oriented list files such as the blacklist file used in find mode.

## Syntax

```AffinityServiceRust/src/config.rs#L877-886
pub fn read_list<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map_while(Result::ok)
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
        .collect())
}
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `path` | `P: AsRef<Path>` | A file system path to the text file to read. Accepts any type that can be converted to a `Path` reference, including `&str`, `String`, and `PathBuf`. |

## Return value

Type: `std::io::Result<Vec<String>>`

On success, returns `Ok` containing a vector of strings where each element is a trimmed, lowercased, non-empty line from the file that does not begin with `#`. On failure, returns the `std::io::Error` from `File::open` (e.g., file not found, permission denied).

### Examples

Given a file with the following content:

```/dev/null/example_blacklist.txt#L1-6
# Processes to ignore during find mode
svchost.exe
System
  Explorer.EXE

# End of list
```

The returned vector would be:

```/dev/null/example_result.txt#L1-3
["svchost.exe", "system", "explorer.exe"]
```

## Remarks

### Processing pipeline

1. The file is opened with `File::open` and wrapped in a `BufReader` for line-buffered reading.
2. Lines are read lazily via the `lines()` iterator. The `map_while(Result::ok)` combinator stops reading at the first I/O error encountered after a successful open (partial results are not returned in that case — the iterator simply terminates early and returns what was collected so far).
3. Each line is trimmed of leading and trailing whitespace and converted to lowercase for case-insensitive matching at runtime.
4. Empty lines (after trimming) and lines that start with `#` (comments) are filtered out.
5. The remaining lines are collected into a `Vec<String>`.

### Case normalization

All entries are lowercased to enable case-insensitive comparisons with Windows process names, which are inherently case-insensitive.

### Comment syntax

Lines beginning with `#` (after trimming) are treated as comments and excluded from the result. Inline comments (e.g., `svchost.exe # system process`) are **not** supported — the entire line would be included as-is (lowercased and trimmed) because it does not start with `#`.

### Error propagation

Only the `File::open` call can produce an error that propagates to the caller via the `?` operator. Subsequent line-reading errors are silently absorbed by `map_while(Result::ok)`, which terminates iteration at the first failed line read rather than propagating the error.

### Encoding

The function reads the file as UTF-8 (the default for `BufReader::lines`). For files encoded in other formats (e.g., UTF-16 LE), use [`read_utf16le_file`](read_utf16le_file.md) instead.

### Usage

`read_list` is primarily used by:
- The main loop to load the blacklist file (`-blacklist <file>`) for find mode.
- [`hotreload_blacklist`](hotreload_blacklist.md) to reload the blacklist when it changes on disk.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Visibility | `pub` |
| Callers | `main.rs` (blacklist loading), [`hotreload_blacklist`](hotreload_blacklist.md) |
| Callees | `File::open`, `BufReader::new`, `BufRead::lines` |
| API | `std::io::Result`, `std::fs::File`, `std::io::BufReader`, `std::path::Path` |
| Privileges | File system read access to the specified path |

## See Also

| Resource | Link |
|----------|------|
| read_utf16le_file | [read_utf16le_file](read_utf16le_file.md) |
| read_config | [read_config](read_config.md) |
| hotreload_blacklist | [hotreload_blacklist](hotreload_blacklist.md) |
| config module overview | [README](README.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*