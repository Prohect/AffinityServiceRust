# get_log_path 函数 (logging.rs)

基于当前日期和给定后缀生成日志文件路径。如果 `logs/` 目录不存在，则自动创建。

## 语法

```rust
fn get_log_path(suffix: &str) -> PathBuf
```

## 参数

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `suffix` | `&str` | 插入在日期与 `.log` 扩展名之间的后缀字符串。例如 `""` 生成常规日志，`".find"` 生成发现日志。 |

## 返回值

返回 `PathBuf`，指向格式为 `logs/YYYYMMDD{suffix}.log` 的日志文件路径。该路径相对于可执行文件所在目录。

## 备注

`get_log_path` 是日志子系统内部使用的辅助函数，用于为 [`LOG_FILE`](LOG_FILE.md) 和 [`FIND_LOG_FILE`](FIND_LOG_FILE.md) 构造日志文件路径。

### 路径格式

生成的路径格式为：

```
logs/YYYYMMDD{suffix}.log
```

示例：

- `get_log_path("")` → `logs/20250115.log`
- `get_log_path(".find")` → `logs/20250115.find.log`

日期部分使用本地时间的年月日，格式为八位数字（`YYYYMMDD`）。

### 目录创建

函数在返回路径之前会检查 `logs/` 目录是否存在。如果不存在，将自动创建该目录。这确保了在首次运行时日志文件可以被成功写入，无需手动创建目录。

### 调用时机

该函数在静态变量 [`LOG_FILE`](LOG_FILE.md) 和 [`FIND_LOG_FILE`](FIND_LOG_FILE.md) 的 `Lazy` 初始化器中被调用，即在首次访问日志文件句柄时执行。由于使用 `Lazy`，该函数在应用程序生命周期内对每个日志文件仅调用一次。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/logging.rs |
| **源码行** | L174–L183 |
| **调用方** | [`LOG_FILE`](LOG_FILE.md)、[`FIND_LOG_FILE`](FIND_LOG_FILE.md) 的 `Lazy` 初始化器 |
| **依赖** | `chrono::Local`、`std::path::PathBuf`、`std::fs::create_dir_all` |

## 另请参阅

- [LOG_FILE 静态变量](LOG_FILE.md)
- [FIND_LOG_FILE 静态变量](FIND_LOG_FILE.md)
- [log_message 函数](log_message.md)
- [log_to_find 函数](log_to_find.md)
- [logging.rs 模块概述](README.md)