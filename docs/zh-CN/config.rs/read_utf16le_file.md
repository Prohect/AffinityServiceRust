# read_utf16le_file 函数 (config.rs)

读取 UTF-16 小端编码的文件，并将其内容作为标准 Rust UTF-8 `String` 返回。由 [convert](convert.md) 函数使用，用于导入 Process Lasso 配置文件，这些文件通常保存为 UTF-16LE 编码。

## 语法

```rust
pub fn read_utf16le_file(path: &str) -> Result<String>
```

## 参数

`path: &str`

要读取的 UTF-16LE 编码文件的路径。路径直接传递给 `std::fs::read` 以进行原始字节加载。

## 返回值

`Result<String>` — 成功时，返回 `Ok(String)`，包含从 UTF-16LE 解码为 UTF-8 的文件内容。失败时，返回从底层 `std::fs::read` 调用传播的 `Err`（例如，文件未找到、权限被拒绝）。

## 备注

### 解码算法

1. 整个文件通过 `std::fs::read` 作为原始字节向量读入内存。
2. 字节被分成两字节对（`chunks_exact(2)`），每对被解释为 little-endian `u16` 代码单元，使用 `u16::from_le_bytes([c[0], c[1]])`。
3. 生成的 `Vec<u16>` 通过 `String::from_utf16_lossy` 解码为 Rust `String`，它将任何无效的 UTF-16 代理序列替换为 Unicode 替换字符（U+FFFD），而不是返回错误。

### BOM 处理

此函数**不**剥离 UTF-16LE 字节顺序标记（BOM，`U+FEFF`）。如果源文件以 BOM 开头，返回字符串的第一个字符将是 `\u{FEFF}`。实际上，这不会影响 [convert](convert.md) 函数，因为 BOM 字符是类似空白的字符，在基于行的解析期间会被忽略。

### 奇数字节计数

如果文件有奇数个字节，最后一个字节会被 `chunks_exact(2)` 静默丢弃。对此边缘情况不会产生警告或错误。

### 损耗转换

使用 `from_utf16_lossy` 意味着此函数永远不会因编码问题而失败——只有 I/O 错误会导致失败。任何损坏的 UTF-16 序列都会被替换为 `�`（U+FFFD），这可能会产生意外输出，但不会panic或返回错误。

### 与标准文件读取的比较

对于采用 UTF-8 编码的文件（AffinityServiceRust 配置文件的默认编码），[read_config](read_config.md) 使用带逐行读取的 `std::io::BufReader`。`read_utf16le_file` 仅用于 Process Lasso 导入，这些导入使用 Windows 原生 UTF-16LE 编码。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **调用方** | [convert](convert.md) |
| **被调用方** | `std::fs::read`, `u16::from_le_bytes`, `String::from_utf16_lossy` |
| **权限** | 指定路径的文件系统读取访问 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [config.rs](README.md) |
| Process Lasso 转换器 | [convert](convert.md) |
| 标准配置读取器（UTF-8） | [read_config](read_config.md) |
| 黑名单读取器（UTF-8） | [read_bleack_list](read_bleack_list.md) |

*文档针对 Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*