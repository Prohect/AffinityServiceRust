# FINDS_SET 静态变量 (logging.rs)

一个延迟初始化的、互斥锁保护的 `HashSet<String>`，用于跟踪在 `-find` 模式会话期间已经记录过的进程名称。此集合提供按会话去重功能，使每个被发现的进程仅写入查找日志一次，无论轮询循环遇到多少次。

## 语法

```logging.rs
pub static FINDS_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
```

## 成员

| 组件 | 类型 | 描述 |
|------|------|------|
| 外层 | `Lazy<…>` | 通过 `once_cell::sync::Lazy` 延迟初始化。集合在首次访问时创建为空。 |
| 中层 | `Mutex<…>` | 提供内部可变性和来自主服务循环的线程安全访问。 |
| 内层 | `HashSet<String>` | 包含当前会话期间已记录过的小写进程名称。 |

## 备注

- 集合初始为空，在进程生命周期内单调递增。没有清除机制——条目会持续存在直到服务重新启动。这是设计上的选择：`-find` 模式是一种诊断工具，旨在生成单次运行期间观察到的所有进程的去重列表。
- [log_process_find](log_process_find.md) 是唯一的写入方。它调用 `FINDS_SET.lock().unwrap().insert(process_name)`，仅当 `insert` 返回 `true`（即名称之前不存在）时才继续写入日志行。
- 便捷宏 [get_fail_find_set!](get_fail_find_set.md) **不会**锁定 `FINDS_SET`；它锁定的是另一个独立的 [FINDS_FAIL_SET](FINDS_FAIL_SET.md) 静态变量。对 `FINDS_SET` 的直接访问在 [log_process_find](log_process_find.md) 中内联完成。
- 由于 `-find` 模式通常枚举约数百个不同的进程名称，此集合的内存占用可以忽略不计。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| Crate 依赖 | `once_cell` (`Lazy`)、`std::sync::Mutex`、`std::collections::HashSet` |
| 写入方 | [log_process_find](log_process_find.md) |
| 读取方 | [log_process_find](log_process_find.md) |
| 清除方 | *（从不清除——生命周期与进程一致）* |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 写入此集合的函数 | [log_process_find](log_process_find.md) |
| 查找模式日志文件句柄 | [FIND_LOG_FILE](FIND_LOG_FILE.md) |
| 失败查找操作的去重集合 | [FINDS_FAIL_SET](FINDS_FAIL_SET.md) |
| 查找模式入口点 | [process_find](../main.rs/README.md) |
| logging 模块概述 | [logging 模块](README.md) |