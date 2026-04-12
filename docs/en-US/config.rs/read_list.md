# read_list function (config.rs)

Reads a simple line-based list file (e.g., a blacklist) and returns a filtered vector of lowercase strings.

## Syntax

```rust
pub fn read_list<P: AsRef<Path>>(path: P) -> Result<Vec<String>>
```

## Parameters

`path`

The file system path to the list file. Accepts any type that implements `AsRef<Path>`, such as `&str`, `String`, or `PathBuf`.

## Return value

Returns `Result<Vec<String>>`. On success, yields a `Vec<String>` containing the non-empty, non-comment lines from the file, each trimmed and lowercased. Returns an I/O error if the file cannot be opened.

## Remarks

This function reads a plain-text file line by line using a buffered reader. Each line is trimmed of leading and trailing whitespace and converted to lowercase. Lines that are empty or begin with `#` (comments) are excluded from the result.

The primary use case is loading blacklist files that specify process names to exclude from configuration. The file format is straightforward — one entry per line:

```
# Processes to ignore
explorer.exe
taskmgr.exe

# System processes
svchost.exe
```

Because all entries are lowercased, matching against the returned list is case-insensitive by convention. Callers can compare lowercased process names directly against the list contents.

Unlike [read_config](read_config.md), this function does not interpret any special syntax such as aliases, constants, groups, or colon-delimited rule fields. It is a minimal utility for simple line-oriented data.

Errors from individual line reads are silently skipped via `map_while(Result::ok)`, so partial reads of a corrupted file will return whatever lines were successfully read before the first I/O error on a line.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Lines** | L842–L851 |
| **Visibility** | `pub` |
| **Dependencies** | `std::fs::File`, `std::io::{BufRead, BufReader, Result}`, `std::path::Path` |
| **Called by** | `main::main` (to load blacklist file) |

## See also

- [read_config](read_config.md) — full configuration file parser
- [read_utf16le_file](read_utf16le_file.md) — reads UTF-16 LE encoded files