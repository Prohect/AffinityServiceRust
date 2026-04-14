# read_utf16le_file 函数 (config.rs)

读取以 UTF-16 Little Endian 编码的文件，并将其解码为 Rust `String`。此函数用于读取 Process Lasso 配置文件（该文件以 UTF-16 LE 编码存储），作为 [convert](convert.md) 工作流的一部分。

## 语法

```rust
pub fn read_utf16le_file(path: &str) -> Result<String>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `path` | `&str` | UTF-16 LE 编码文件的文件系统路径。直接传递给 `std::fs::read` 以加载原始字节。 |

## 返回值

返回 `Result<String>`：

- **`Ok(String)`** — 文件解码后的 UTF-8 字符串内容。任何无效的 UTF-16 代理对都会通过 `String::from_utf16_lossy` 替换为 Unicode 替换字符（U+FFFD）。
- **`Err(io::Error)`** — 如果文件无法打开或读取，则从 `std::fs::read` 传播错误。

## 备注

### 解码算法

1. 使用 `std::fs::read` 将整个文件读入内存为 `Vec<u8>`。
2. 使用 `chunks_exact(2)` 按每 2 个字节为一组迭代字节向量。
3. 每个块通过 `u16::from_le_bytes` 以小端字节序转换为 `u16` 值。
4. 生成的 `Vec<u16>` 使用 `String::from_utf16_lossy` 解码为 `String`，该方法用 U+FFFD 替换无效代理对，而不是返回错误。

### BOM 处理

此函数**不会**剥离 UTF-16 LE 字节顺序标记（BOM，`U+FEFF` / 字节 `FF FE`）。如果输入文件以 BOM 开头，它将作为返回字符串的第一个字符出现。对于 [convert](convert.md) 的使用场景，这是无害的，因为转换器通过查找特定的行前缀（例如 `NamedAffinities=`）来解析文件，而前导 BOM 字符不会影响后续行的前缀匹配。

### 奇数字节计数

如果文件的字节数为奇数，最后一个字节会被 `chunks_exact(2)` 静默丢弃。对于这种边界情况不会产生错误或警告。

### 有损解码

使用 `from_utf16_lossy` 意味着此函数永远不会因编码问题而失败——只有 I/O 错误才能导致返回失败。格式错误的 UTF-16 序列会被替换为替换字符，这确保即使文件部分损坏，下游解析也能继续进行。

### 使用上下文

此函数仅由 [convert](convert.md) 调用，用于读取 Process Lasso `.ini` 风格的配置文件。Process Lasso 以 UTF-16 LE 编码存储其配置，这在原生 Win32 应用程序编写的 Windows INI 文件中很常见。转换后的输出由 [convert](convert.md) 以 UTF-8 编码写入。

### 与 read_list 和 read_config 的比较

与使用 `BufReader` 逐行读取 UTF-8 文件的 [read_list](read_list.md) 和 [read_config](read_config.md) 不同，`read_utf16le_file` 一次性将整个文件读入内存并执行批量解码。这种方法是合适的，因为：

- Process Lasso 配置文件通常较小。
- UTF-16 解码需要处理字节对，这不适合逐行缓冲读取。

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **可见性** | `pub` |
| **调用者** | [convert](convert.md) |
| **被调用者** | `std::fs::read`、`u16::from_le_bytes`、`String::from_utf16_lossy` |
| **API** | `std::fs::read` 用于文件 I/O |
| **权限** | 需要对指定文件路径的读取权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| Process Lasso 配置转换器 | [convert](convert.md) |
| UTF-8 配置文件读取器 | [read_config](read_config.md) |
| UTF-8 列表文件读取器 | [read_list](read_list.md) |
| 模块概述 | [config 模块](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd