# read_utf16le_file function (config.rs)

Reads a file encoded as UTF-16 Little Endian and returns its content as a Rust `String`. This function is used to ingest Process Lasso configuration files, which are typically saved in UTF-16 LE encoding on Windows.

## Syntax

```AffinityServiceRust/src/config.rs#L888-892
pub fn read_utf16le_file(path: &str) -> Result<String> {
    let bytes = read(path)?;
    let utf16: Vec<u16> = bytes.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
    Ok(String::from_utf16_lossy(&utf16))
}
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `path` | `&str` | The filesystem path to the UTF-16 LE encoded file to read. |

## Return value

Type: `std::io::Result<String>`

On success, returns `Ok(String)` containing the decoded UTF-8 content of the file. On failure, returns an `Err` propagated from `std::fs::read` (e.g., file not found, permission denied).

## Remarks

### Encoding conversion

The function performs the following steps:

1. Reads the entire file into a `Vec<u8>` byte buffer via `std::fs::read`.
2. Groups the bytes into pairs using `chunks_exact(2)` and reconstructs each pair as a little-endian `u16` code unit via `u16::from_le_bytes`.
3. Converts the resulting `Vec<u16>` into a Rust `String` using `String::from_utf16_lossy`, which replaces any invalid UTF-16 surrogate sequences with the Unicode replacement character (U+FFFD) rather than returning an error.

### BOM handling

This function does **not** strip the UTF-16 Byte Order Mark (BOM, U+FEFF) if present at the start of the file. The BOM will appear as the first character of the returned string. Callers that need to handle BOM-prefixed files should trim the leading `\u{FEFF}` character from the result if necessary. In practice, the downstream consumer ([`convert`](convert.md)) processes the file line-by-line and the BOM, if present, only affects the first line which is typically a section header or empty line.

### Lossy conversion

`String::from_utf16_lossy` is used instead of `String::from_utf16` (which returns a `Result`). This means malformed UTF-16 sequences — such as unpaired surrogates or files with an odd number of bytes where the trailing byte is silently dropped by `chunks_exact` — are silently replaced with `U+FFFD` rather than causing an error. This trade-off prioritizes robustness over strict correctness, since third-party configuration files may contain minor encoding anomalies.

### Odd byte count

If the file has an odd number of bytes, the final trailing byte is discarded by `chunks_exact(2)`. No warning or error is produced for this edge case.

### Platform notes

This function is designed for Windows environments where UTF-16 LE is a common file encoding. Process Lasso and other Windows utilities frequently use this encoding for configuration and export files.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Visibility | `pub` |
| Callers | [`convert`](convert.md) |
| Callees | `std::fs::read`, `u16::from_le_bytes`, `String::from_utf16_lossy` |
| API | `std::io::Result` |
| Privileges | Filesystem read access to the specified path |

## See Also

| Resource | Link |
|----------|------|
| convert | [convert](convert.md) |
| read_list | [read_list](read_list.md) |
| read_config | [read_config](read_config.md) |
| config module overview | [README](README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*