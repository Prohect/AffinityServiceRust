# read_utf16le_file function (config.rs)

Reads a UTF-16 Little Endian encoded file and returns its contents as a Rust `String`.

## Syntax

```rust
pub fn read_utf16le_file(path: &str) -> Result<String>
```

## Parameters

`path`

The file system path to the UTF-16 LE encoded file to read, as a string slice.

## Return value

Returns `Result<String>` — an `Ok(String)` containing the decoded file contents on success, or an `Err` if the file cannot be read.

The function uses `String::from_utf16_lossy` internally, so any invalid UTF-16 sequences are replaced with the Unicode replacement character (U+FFFD) rather than causing an error.

## Remarks

This function reads the raw bytes of the file, interprets each pair of bytes as a little-endian `u16` code unit, and then converts the resulting UTF-16 sequence into a Rust UTF-8 `String`.

This is primarily used to read Process Lasso configuration files, which are saved in UTF-16 LE encoding. The [convert](convert.md) function depends on this to parse Process Lasso INI-style configs.

The function processes bytes using `chunks_exact(2)`, which means if the file has an odd number of bytes the last byte will be silently discarded.

> **Note:** No BOM (Byte Order Mark) detection or stripping is performed. If the file contains a BOM (`0xFF 0xFE`), it will appear as a leading character in the returned string.

### Example

```rust
// Read a Process Lasso config file
let content = read_utf16le_file("C:\\ProgramData\\ProcessLasso\\ProcessLasso.ini")?;
for line in content.lines() {
    println!("{}", line);
}
```

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Visibility** | `pub` |
| **Called by** | [convert](convert.md) |
| **Dependencies** | `std::fs::read`, `std::io::Result` |

## See also

- [convert](convert.md) — Process Lasso config conversion that uses this function
- [read_config](read_config.md) — reads native AffinityServiceRust config files (UTF-8)
- [read_list](read_list.md) — reads simple list files (UTF-8)