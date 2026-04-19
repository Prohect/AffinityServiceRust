# read_utf16le_file function (config.rs)

Reads a file encoded in UTF-16 Little Endian and returns its contents as a standard Rust UTF-8 `String`. Used by the [convert](convert.md) function to import Process Lasso configuration files, which are typically saved in UTF-16LE encoding.

## Syntax

```rust
pub fn read_utf16le_file(path: &str) -> Result<String>
```

## Parameters

`path: &str`

The filesystem path to the UTF-16LE encoded file to read. The path is passed directly to `std::fs::read` for raw byte loading.

## Return value

`Result<String>` — On success, returns `Ok(String)` containing the file's content decoded from UTF-16LE into UTF-8. On failure, returns an `Err` propagated from the underlying `std::fs::read` call (e.g., file not found, permission denied).

## Remarks

### Decoding algorithm

1. The entire file is read into memory as a raw byte vector via `std::fs::read`.
2. The bytes are chunked into pairs of two (`chunks_exact(2)`) and each pair is interpreted as a little-endian `u16` code unit using `u16::from_le_bytes([c[0], c[1]])`.
3. The resulting `Vec<u16>` is decoded into a Rust `String` via `String::from_utf16_lossy`, which replaces any invalid UTF-16 surrogate sequences with the Unicode replacement character (U+FFFD) rather than returning an error.

### BOM handling

This function does **not** strip a UTF-16LE Byte Order Mark (BOM, `U+FEFF`). If the source file begins with a BOM, the first character of the returned string will be `\u{FEFF}`. In practice, this does not affect the [convert](convert.md) function because the BOM character is whitespace-like and is ignored during line-based parsing.

### Odd byte count

If the file has an odd number of bytes, the final byte is silently dropped by `chunks_exact(2)`. No warning or error is produced for this edge case.

### Lossy conversion

The use of `from_utf16_lossy` means this function never fails due to encoding issues — only I/O errors can cause a failure. Any malformed UTF-16 sequences are replaced with `�` (U+FFFD), which may produce unexpected output but will not panic or return an error.

### Comparison with standard file reading

For files encoded in UTF-8 (the default for AffinityServiceRust config files), [read_config](read_config.md) uses `std::io::BufReader` with line-by-line reading. `read_utf16le_file` is only needed for Process Lasso imports, which use Windows-native UTF-16LE encoding.

## Requirements

| | |
|---|---|
| **Module** | `src/config.rs` |
| **Callers** | [convert](convert.md) |
| **Callees** | `std::fs::read`, `u16::from_le_bytes`, `String::from_utf16_lossy` |
| **Privileges** | File system read access to the specified path |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [config.rs](README.md) |
| Process Lasso converter | [convert](convert.md) |
| Standard config reader (UTF-8) | [read_config](read_config.md) |
| Blacklist reader (UTF-8) | [read_bleack_list](read_bleack_list.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*