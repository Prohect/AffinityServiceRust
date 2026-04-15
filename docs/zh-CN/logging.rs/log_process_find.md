# log_process_find 函数 (logging.rs)

记录从 `-find` 模式中发现的进程名称，每个会话内进行去重。使用 `FINDS_SET` 静态变量确保每个唯一的进程名称在当前应用程序运行期间只记录一次，防止在调度循环迭代中重复发现的进程造成日志泛滥。

## 语法

```rust
#[inline]
pub fn log_process_find(process_name: &str)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `process_name` | `&str` | 要记录的已发现进程的名称。此值既用作去重键（插入 `FINDS_SET`），也用作日志消息载荷。 |

## 返回值

此函数不返回值。

## 备注

### 去重机制

该函数锁定全局 [FINDS_SET](statics.md#finds_set)（`Mutex<HashSet<String>>`）并尝试插入给定的 `process_name`。由于 `HashSet::insert` 仅在值之前不存在时返回 `true`，因此通过 [`log_to_find`](log_to_find.md) 执行的日志写入仅在每个会话中每个进程名称的首次出现时执行。

### 算法

1. 锁定 `FINDS_SET`。
2. 对集合调用 `insert(process_name.to_string())`。
3. 如果 `insert` 返回 `true`（名称是新的），则调用 [`log_to_find`](log_to_find.md) 写入消息 `"find <process_name>"`。
4. 如果 `insert` 返回 `false`（名称已在集合中），则不执行任何操作。

### 日志输出格式

当记录新的进程名称时，输出行的格式为：

```text
[HH:MM:SS]find <process_name>
```

时间戳前缀由 [`log_to_find`](log_to_find.md) 添加。消息根据 [USE_CONSOLE](statics.md#use_console) 的值写入 `.find` 日志文件或 stdout。

### 性能

该函数标记为 `#[inline]`，允许编译器在调用点内联。去重检查为摊销 O(1)（哈希集查找），使其在每个调度周期中对每个已发现的进程调用时开销很低。

### 会话作用域

`FINDS_SET` 在正常运行期间永远不会被清除——它在应用程序的整个生命周期中累积进程名称。这意味着如果一个进程退出并重新启动，它在同一会话中**不会**被重新记录。新的会话（应用程序重启）从空集合开始。

### 与 `-find` 模式的关系

`-find` CLI 模式扫描 CPU 亲和性尚未被显式配置（即其亲和性与系统默认值匹配）的进程。对于每个这样的进程，会调用 `log_process_find`，以便用户可以看到哪些进程是"未配置的"，同时不会在每个轮询周期被重复条目淹没。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `logging.rs` |
| **调用方** | `apply.rs`、`scheduler.rs` — find 模式扫描逻辑 |
| **被调用方** | [`log_to_find`](log_to_find.md) |
| **静态变量** | [FINDS_SET](statics.md#finds_set) |
| **平台** | 跨平台（无直接 Windows API 调用） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| log_to_find 函数 | [log_to_find](log_to_find.md) |
| log_message 函数 | [log_message](log_message.md) |
| FINDS_SET 静态变量 | [statics](statics.md#finds_set) |
| is_affinity_unset 函数 | [is_affinity_unset](../winapi.rs/is_affinity_unset.md) |
| logging 模块概述 | [README](README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
