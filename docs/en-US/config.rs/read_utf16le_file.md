# read_utf16le_file function (config.rs)

Reads a file encoded as UTF-16 Little Endian and decodes it into a Rust `String`. This function is used to read Process Lasso configuration files, which are stored in UTF-16 LE encoding, as part of the [convert](convert.md) workflow.

## Syntax

```rust
pub fn read_utf16le_file(path: &str) -> Result<String>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `path` | `&str` | File system path to the UTF-16 LE encoded file. Passed directly to `std::fs::read` to load the raw bytes. |

## Return value

Returns `Result<String>`:

- **`Ok(String)`** — The decoded UTF-8 string content of the file. Any invalid UTF-16 surrogate pairs are replaced with the Unicode replacement character (U+FFFD) via `String::from_utf16_lossy`.
- **`Err(io::Error)`** — Propagated from `std::fs::read` if the file cannot be opened or read.

## Remarks

### Decoding algorithm

1. The entire file is read into memory as a `Vec<u8>` using `std::fs::read`.
2. The byte vector is iterated in exact 2-byte chunks using `chunks_exact(2)`.
3. Each chunk is converted to a `u16` value in little-endian byte order via `u16::from_le_bytes`.
4. The resulting `Vec<u16>` is decoded to a `String` using `String::from_utf16_lossy`, which replaces invalid surrogate pairs with U+FFFD rather than returning an error.

### BOM handling

This function does **not** strip a UTF-16 LE Byte Order Mark (BOM, `U+FEFF` / bytes `FF FE`). If the input file begins with a BOM, it will appear as the first character of the returned string. For the [convert](convert.md) use case this is harmless because the converter parses the file by looking for specific line prefixes (e.g., `NamedAffinities=`), and a leading BOM character does not affect prefix matching on subsequent lines.

### Odd byte count

If the file has an odd number of bytes, the final byte is silently dropped by `chunks_exact(2)`. No error or warning is produced for this edge case.

### Lossy decoding

The use of `from_utf16_lossy` means this function never fails due to encoding issues — only I/O errors can cause a failure return. Malformed UTF-16 sequences are replaced with the replacement character, which ensures downstream parsing can proceed even with partially corrupted files.

### Usage context

This function is called exclusively by [convert](convert.md) to read Process Lasso `.ini`-style configuration files. Process Lasso stores its configuration in UTF-16 LE encoding, which is common for Windows INI files written by native Win32 applications. The converted output is written in UTF-8 by [convert](convert.md).

### Comparison with read_list and read_config

Unlike [read_list](read_list.md) and [read_config](read_config.md), which use `BufReader` for line-by-line reading of UTF-8 files, `read_utf16le_file` reads the entire file into memory at once and performs a bulk decode. This approach is appropriate because:

- Process Lasso config files are typically small.
- UTF-16 decoding requires processing pairs of bytes, which does not lend itself to line-buffered reading.

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Visibility** | `pub` |
| **Callers** | [convert](convert.md) |
| **Callees** | `std::fs::read`, `u16::from_le_bytes`, `String::from_utf16_lossy` |
| **API** | `std::fs::read` for file I/O |
| **Privileges** | Read access to the specified file path |

## See Also

| Topic | Link |
|-------|------|
| Process Lasso config converter | [convert](convert.md) |
| UTF-8 config file reader | [read_config](read_config.md) |
| UTF-8 list file reader | [read_list](read_list.md) |
| Config module overview | [README](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd