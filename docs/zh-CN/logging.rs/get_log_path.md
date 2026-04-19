# get_log_path 函数 (logging.rs)

在 `logs/` 目录下构造一个带日期戳的日志文件路径，如果目录不存在则创建它。

## 语法

```rust
fn get_log_path(suffix: &str) -> PathBuf
```

## 参数

`suffix: &str`

一个字符串，插入在日期戳和 `.log` 扩展名之间。对于主日志文件传递 `""`，对于查找模式的日志文件传递 `".find"`。

| 后缀值 | 生成的文件名 |
|---|---|
| `""` | `logs/YYYYMMDD.log` |
| `".find"` | `logs/YYYYMMDD.find.log` |

## 返回值

`PathBuf` — 完全构造的日志文件路径，相对于服务的当前工作目录。路径遵循模式 `logs/YYYYMMDD{suffix}.log`，其中 `YYYYMMDD` 是从 [LOCAL_TIME_BUFFER](README.md) 获取的当前本地日期。

## 备注

- 函数从 `LOCAL_TIME_BUFFER` 静态变量通过 `get_local_time!()` 宏读取当前日期，然后在执行文件系统操作之前立即释放锁。
- 如果 `logs/` 目录不存在，函数通过 `std::fs::create_dir_all` 创建它（以及任何必要的父目录）。目录创建失败会被静默忽略；调用方在尝试打开文件时会遇到错误。
- 此函数**不是** `pub` — 它是模块私有的，仅在 [LOG_FILE](README.md) 和 [FIND_LOG_FILE](README.md) 静态变量的惰性初始化期间被调用。
- 因为 `LOG_FILE` 和 `FIND_LOG_FILE` 是在整个进程生命周期中惰性初始化一次的，所以日志文件的日期在第一次使用时确定。服务**不会**在午夜切换到新文件；需要重新启动才能开始写入新日期的日志文件。

### 日期格式

日期部分使用零填充的四位数年份、两位数月份和两位数日期，没有分隔符：2025 年 1 月 15 日为 `20250115`。

## 要求

| | |
|---|---|
| **模块** | `src/logging.rs` |
| **可见性** | 私有（模块内部） |
| **调用方** | `LOG_FILE` 静态初始化器，`FIND_LOG_FILE` 静态初始化器 |
| **依赖** | `LOCAL_TIME_BUFFER`，`chrono::Datelike`，`std::fs::create_dir_all` |
| **权限** | 工作目录的文件系统写入访问权限 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [logging.rs](README.md) |
| 主日志输出 | [log_message](log_message.md) |
| 查找日志输出 | [log_to_find](log_to_find.md) |

*记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
```

<tool_call>
<function=edit_file>
<parameter=display_description>
Translate log_message.md from English to Chinese
</parameter>
<parameter=path>
AffinityServiceRust/docs/zh-CN/logging.rs/log_message.md
</parameter>
<parameter=mode>
overwrite
</parameter>
</function>
</tool_call>
<tool_call>
<function=edit_file>
<parameter=display_description>
Translate log_process_find.md from English to Chinese
</parameter>
<parameter=path>
AffinityServiceRust/docs/zh-CN/logging.rs/log_process_find.md
</parameter>
<parameter=mode>
overwrite
</parameter>
</function>
</tool_call>
<tool_call>
<function=edit_file>
<parameter=display_description>
Translate log_pure_message.md from English to Chinese
</parameter>
<parameter=path>
AffinityServiceRust/docs/zh-CN/logging.rs/log_pure_message.md
</parameter>
<parameter=mode>
overwrite
</parameter>
</function>
</tool_call>
<tool_call>
<function=edit_file>
<parameter=display_description>
Translate log_to_find.md from English to Chinese
</parameter>
<parameter=path>
AffinityServiceRust/docs/zh-CN/logging.rs/log_to_find.md
</parameter>
<parameter=mode>
overwrite
</parameter>
</function>
</tool_call>
<tool_call>
<function=edit_file>
<parameter=display_description>
Translate purge_fail_map.md from English to Chinese
</parameter>
<parameter=path>
AffinityServiceRust/docs/zh-CN/logging.rs/purge_fail_map.md
</parameter>
<parameter=mode>
overwrite
</parameter>
</function>
</tool_call>