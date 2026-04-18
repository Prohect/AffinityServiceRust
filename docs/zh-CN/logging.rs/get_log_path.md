# get_log_path 函数 (logging.rs)

在 `logs/` 目录下构建带日期戳的日志文件路径。该函数从 [`LOCAL_TIME_BUFFER`](statics.md#local_time_buffer) 静态变量读取当前本地时间，以 `YYYYMMDD<suffix>.log` 的格式构造文件名，并在返回路径之前确保 `logs/` 目录存在。

## 语法

```rust
fn get_log_path(suffix: &str) -> PathBuf
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `suffix` | `&str` | 附加在文件名日期部分之后、`.log` 扩展名之前的字符串。主日志文件传入 `""`，find 模式日志文件传入 `".find"`。 |

## 返回值

返回一个指向日志文件的 `PathBuf`。路径相对于工作目录，格式为 `logs/YYYYMMDD<suffix>.log`。

### 示例

| `suffix` | 日期 | 生成的路径 |
|----------|------|------------|
| `""` | 2025-01-15 | `logs/20250115.log` |
| `".find"` | 2025-01-15 | `logs/20250115.find.log` |

## 备注

- 此函数为**模块私有**（`fn` 不带 `pub`）。它在 [`LOG_FILE`](statics.md#log_file) 和 [`FIND_LOG_FILE`](statics.md#find_log_file) 静态变量的 `Lazy` 初始化期间被调用，在 `logging` 模块外部无法访问。

- 该函数锁定 [`LOCAL_TIME_BUFFER`](statics.md#local_time_buffer) 互斥量以读取缓存的本地时间。锁在继续创建目录之前被显式释放（通过 `drop(time)`），以最小化锁的持有时间。

- `logs/` 目录通过 `std::fs::create_dir_all` 创建（如果不存在）。如果目录创建失败，错误会被静默忽略（`let _ = create_dir_all(...)`），返回的路径可能指向一个不存在的目录。后续使用此路径的文件打开操作将在那时失败。

- 日期组件使用 `chrono::Datelike` trait 方法（`year()`、`month()`、`day()`）提取，并进行零填充格式化以确保一致的 8 位日期字符串。

### 算法

1. 锁定 `LOCAL_TIME_BUFFER` 互斥量并提取 `(year, month, day)`。
2. 释放锁。
3. 构造 `logs/` 目录的 `PathBuf`。
4. 如果目录不存在，尝试创建它（包括父目录）。
5. 将格式化后的文件名 `YYYYMMDD<suffix>.log` 与目录路径拼接。
6. 返回生成的 `PathBuf`。

### 调用位置

此函数在程序初始化期间恰好被调用两次：

- [`LOG_FILE`](statics.md#log_file) 的 `Lazy` 初始化器：`get_log_path("")`
- [`FIND_LOG_FILE`](statics.md#find_log_file) 的 `Lazy` 初始化器：`get_log_path(".find")`

由于这些静态变量是延迟初始化的，`get_log_path` 直到第一条日志消息被写入或第一条 find 日志条目被记录时才会被调用。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `logging.rs` |
| **可见性** | 私有（模块内部） |
| **调用方** | `LOG_FILE` 和 `FIND_LOG_FILE` 的 `Lazy` 初始化器 |
| **被调用方** | `LOCAL_TIME_BUFFER.lock()`、`chrono::Datelike` 方法、`std::fs::create_dir_all`、`PathBuf::join` |
| **依赖** | `chrono`、`std::fs`、`std::path::PathBuf` |
| **平台** | 跨平台（无 Windows 特定 API 调用） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| log_message 函数 | [log_message](log_message.md) |
| log_to_find 函数 | [log_to_find](log_to_find.md) |
| LOG_FILE 静态变量 | [statics](statics.md#log_file) |
| FIND_LOG_FILE 静态变量 | [statics](statics.md#find_log_file) |
| LOCAL_TIME_BUFFER 静态变量 | [statics](statics.md#local_time_buffer) |
| logging 模块概述 | [README](README.md) |

---
*提交：[29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
