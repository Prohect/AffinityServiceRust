# FINDS_FAIL_SET 静态变量 (logging.rs)

一个全局的、延迟初始化的、互斥锁保护的 `HashSet<String>`，用于跟踪在当前会话中 `-find` 模式发现已经失败的进程名称。这防止了同一失败查找操作的重复日志记录，在进程无法匹配时保持日志输出的简洁性。

## 语法

```logging.rs
pub static FINDS_FAIL_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
```

## 成员

| 组件 | 类型 | 描述 |
|------|------|------|
| 外层 | `Lazy<…>` | 通过 `once_cell::sync::Lazy` 延迟初始化。`HashSet` 在首次访问时创建为空。 |
| 中层 | `Mutex<…>` | 提供内部可变性和从服务循环进行的线程安全访问。 |
| 内层 | `HashSet<String>` | 包含已记录过查找失败的进程名称字符串。 |

## 备注

- `FINDS_FAIL_SET` 是 [FINDS_SET](FINDS_SET.md) 的补充，后者对*成功的*进程发现进行去重。两者结合确保成功和失败的查找操作对于每个唯一的进程名称在每个会话中最多只记录一次。
- 该集合通过 [get_fail_find_set!](get_fail_find_set.md) 宏访问，该宏获取互斥锁并返回 `MutexGuard<HashSet<String>>`。
- 该集合在正常运行期间永远不会被显式清除——它在服务进程的整个生命周期内累积条目。这是可以接受的，因为典型配置中不同进程名称的数量较少（最多数百个）。
- 条目是普通的 `String` 值，表示小写或按配置格式的进程名称，与配置解析器使用的格式一致。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| Crate 依赖 | `once_cell` (`Lazy`)、`std::sync::Mutex`、`std::collections::HashSet` |
| 访问器宏 | [get_fail_find_set!](get_fail_find_set.md) |
| 调用者 | [process_find](../main.rs/README.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 成功查找的去重集合 | [FINDS_SET](FINDS_SET.md) |
| 每 PID 操作失败跟踪 | [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) |
| 查找模式日志输出函数 | [log_to_find](log_to_find.md) |
| 查找模式进程日志记录 | [log_process_find](log_process_find.md) |
| logging 模块概述 | [logging 模块](README.md) |