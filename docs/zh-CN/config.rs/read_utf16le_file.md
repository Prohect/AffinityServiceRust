# read_utf16le_file 函数 (config.rs)

读取 UTF-16 Little Endian 编码的文件，并将其内容作为 Rust `String` 返回。

## 语法

```rust
pub fn read_utf16le_file(path: &str) -> Result<String>
```

## 参数

`path`

要读取的 UTF-16 LE 编码文件的文件系统路径，以字符串切片形式提供。

## 返回值

返回 `Result<String>` —— 成功时返回 `Ok(String)`，包含解码后的文件内容；如果文件无法读取，则返回 `Err`。

函数内部使用 `String::from_utf16_lossy`，因此任何无效的 UTF-16 序列将被替换为 Unicode 替换字符（U+FFFD），而不会导致错误。

## 备注

此函数读取文件的原始字节，将每对字节解释为小端序 `u16` 码元，然后将生成的 UTF-16 序列转换为 Rust 的 UTF-8 `String`。

主要用于读取 Process Lasso 配置文件，该类文件以 UTF-16 LE 编码保存。[convert](convert.md) 函数依赖此函数来解析 Process Lasso 的 INI 风格配置。

函数使用 `chunks_exact(2)` 处理字节，这意味着如果文件包含奇数个字节，最后一个字节将被静默丢弃。

> **注意：** 不执行 BOM（字节顺序标记）的检测或剥离。如果文件包含 BOM（`0xFF 0xFE`），它将作为返回字符串的前导字符出现。

### 示例

```rust
// 读取 Process Lasso 配置文件
let content = read_utf16le_file("C:\\ProgramData\\ProcessLasso\\ProcessLasso.ini")?;
for line in content.lines() {
    println!("{}", line);
}
```

## 要求

| 要求 | 值 |
| --- | --- |
| **所属模块** | src/config.rs |
| **可见性** | `pub` |
| **调用方** | [convert](convert.md) |
| **依赖** | `std::fs::read`、`std::io::Result` |

## 另请参阅

- [convert](convert.md) — 使用此函数的 Process Lasso 配置转换功能
- [read_config](read_config.md) — 读取原生 AffinityServiceRust 配置文件（UTF-8）
- [read_list](read_list.md) — 读取简单列表文件（UTF-8）