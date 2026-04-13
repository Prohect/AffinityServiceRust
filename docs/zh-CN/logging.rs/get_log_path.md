# get_log_path 函数 (logging.rs)

在 `logs/` 目录下构建一个带日期前缀的日志文件路径。该函数从 [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) 读取缓存的本地时间以派生 `YYYYMMDD` 日期组件，并在 `.log` 扩展名之前附加一个可选的后缀。如果 `logs/` 目录不存在，将自动创建。

## 语法

```logging.rs
fn get_log_path(suffix: &str) -> PathBuf
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `suffix` | `&str` | 附加在日期前缀和 `.log` 扩展名之间的字符串。对于主日志文件传递 `""`，对于查找模式日志文件传递 `".find"`。后缀按原样插入——如果需要点分隔符，必须包含在字符串中（例如 `".find"` 而非 `"find"`）。 |

## 返回值

一个 `PathBuf`，表示日志文件的完全限定相对路径，例如 `logs/20250114.log` 或 `logs/20250114.find.log`。

## 备注

- 该函数通过 [get_local_time!](get_local_time.md) 宏获取 [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) 互斥锁，使用 `chrono::Datelike` 提取 `year`、`month` 和 `day` 组件，然后在执行任何文件系统操作之前显式释放守卫。这一点很重要，因为 `create_dir_all` 可能会阻塞在 I/O 上，在 I/O 期间持有时间缓冲区锁会不必要地延迟其他线程。
- `logs/` 目录通过 `std::fs::create_dir_all` 创建（如果尚不存在）。`create_dir_all` 的结果被静默丢弃（`let _ = …`），因此此时创建目录的失败不会产生错误——它将在调用者尝试打开文件时才浮现。
- 此函数**不是** `pub` 的——它是模块私有的（`fn`，而非 `pub fn`）。它仅在 [LOG_FILE](LOG_FILE.md) 和 [FIND_LOG_FILE](FIND_LOG_FILE.md) 的延迟初始化期间被调用，不打算在 `logging` 模块外部使用。
- 文件名的日期部分由函数被调用时 [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) 中的值决定。由于 `LOG_FILE` 和 `FIND_LOG_FILE` 是 `Lazy` 静态变量，路径只计算一次——在首次访问时——生成的文件句柄在整个进程生命周期中被重用。日志文件不会在午夜自动轮换；需要新进程（或服务重启）才能开始新的日期日志文件。

### 路径格式

生成的路径遵循以下模式：

```/dev/null/example.txt#L1-1
logs/{YYYY}{MM}{DD}{suffix}.log
```

示例：

| 后缀 | 生成的路径 |
|------|-----------|
| `""` | `logs/20250114.log` |
| `".find"` | `logs/20250114.find.log` |

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| 可见性 | 模块私有（`fn`，而非 `pub fn`） |
| 调用方 | [LOG_FILE](LOG_FILE.md) 初始化器、[FIND_LOG_FILE](FIND_LOG_FILE.md) 初始化器 |
| 被调用方 | [get_local_time!](get_local_time.md)、`std::fs::create_dir_all` |
| Crate 依赖 | `chrono` (`Datelike`)、`std::path::PathBuf`、`std::fs::create_dir_all` |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 由此函数初始化的主日志文件句柄 | [LOG_FILE](LOG_FILE.md) |
| 由此函数初始化的查找模式日志文件句柄 | [FIND_LOG_FILE](FIND_LOG_FILE.md) |
| 用于日期派生的缓存本地时间 | [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) |
| 带时间戳的日志写入 | [log_message](log_message.md) |
| 查找模式日志写入 | [log_to_find](log_to_find.md) |
| logging 模块概述 | [logging 模块](README.md) |