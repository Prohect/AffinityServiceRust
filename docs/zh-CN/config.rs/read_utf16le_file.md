# read_utf16le_file 函数 (config.rs)

读取 UTF-16 Little Endian 编码的文件，并将其内容作为 Rust `String` 返回。此函数用于读取 Process Lasso 配置文件，这些文件在 Windows 上通常以 UTF-16 LE 编码保存。

## 语法

```AffinityServiceRust/src/config.rs#L888-892
pub fn read_utf16le_file(path: &str) -> Result<String> {
    let bytes = read(path)?;
    let utf16: Vec<u16> = bytes.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
    Ok(String::from_utf16_lossy(&utf16))
}
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `path` | `&str` | 要读取的 UTF-16 LE 编码文件的文件系统路径。 |

## 返回值

类型：`std::io::Result<String>`

成功时，返回包含文件解码后 UTF-8 内容的 `Ok(String)`。失败时，返回从 `std::fs::read` 传播的 `Err`（例如，文件未找到、权限被拒绝）。

## 备注

### 编码转换

该函数执行以下步骤：

1. 通过 `std::fs::read` 将整个文件读入 `Vec<u8>` 字节缓冲区。
2. 使用 `chunks_exact(2)` 将字节分组为配对，并通过 `u16::from_le_bytes` 将每对重建为小端序 `u16` 码元。
3. 使用 `String::from_utf16_lossy` 将结果 `Vec<u16>` 转换为 Rust `String`，该方法会将任何无效的 UTF-16 代理序列替换为 Unicode 替换字符（U+FFFD），而不是返回错误。

### BOM 处理

此函数**不会**去除文件开头的 UTF-16 字节顺序标记（BOM，U+FEFF）（如果存在的话）。BOM 将作为返回字符串的第一个字符出现。如果调用方需要处理带 BOM 前缀的文件，应在必要时从结果中修剪前导的 `\u{FEFF}` 字符。在实际使用中，下游消费者（[`convert`](convert.md)）逐行处理文件，而 BOM（如果存在）仅影响第一行，该行通常是节标题或空行。

### 有损转换

使用 `String::from_utf16_lossy` 而非 `String::from_utf16`（后者返回 `Result`）。这意味着格式错误的 UTF-16 序列——如未配对的代理或由于 `chunks_exact` 静默丢弃尾随字节而导致的奇数字节文件——会被静默替换为 `U+FFFD`，而不是导致错误。这种权衡优先考虑健壮性而非严格正确性，因为第三方配置文件可能包含轻微的编码异常。

### 奇数字节数

如果文件具有奇数个字节，最后一个尾随字节会被 `chunks_exact(2)` 丢弃。此边界情况不会产生警告或错误。

### 平台说明

此函数专为 Windows 环境设计，其中 UTF-16 LE 是常见的文件编码。Process Lasso 和其他 Windows 工具经常对配置和导出文件使用此编码。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 可见性 | `pub` |
| 调用方 | [`convert`](convert.md) |
| 被调用方 | `std::fs::read`、`u16::from_le_bytes`、`String::from_utf16_lossy` |
| API | `std::io::Result` |
| 权限 | 指定路径的文件系统读取权限 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| convert | [convert](convert.md) |
| read_list | [read_list](read_list.md) |
| read_config | [read_config](read_config.md) |
| config 模块概述 | [README](README.md) |

---
*提交：[37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*