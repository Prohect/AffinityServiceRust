# PID_MAP_FAIL_ENTRY_SET 静态变量 (logging.rs)

全局按 PID 组织的 [ApplyFailEntry](ApplyFailEntry.md) 记录映射，用于对 Windows API 操作错误进行去重。映射中的每个条目跟踪特定的进程、线程、操作和错误代码组合是否已被记录，防止在服务轮询循环期间重复相同的错误消息泛滥日志输出。

## 语法

```logging.rs
pub static PID_MAP_FAIL_ENTRY_SET: Lazy<Mutex<HashMap<u32, HashMap<ApplyFailEntry, bool>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
```

## 成员

| 组件 | 类型 | 描述 |
|------|------|------|
| 外层 | `Lazy<…>` | 通过 `once_cell::sync::Lazy` 延迟初始化。映射在首次访问时创建为空。 |
| 中层 | `Mutex<…>` | 提供内部可变性和来自服务循环线程的线程安全访问。 |
| 内层 | `HashMap<u32, HashMap<ApplyFailEntry, bool>>` | 两级映射：外层键为进程 ID（`u32`）；内层映射键为 [ApplyFailEntry](ApplyFailEntry.md) 记录，`bool` 值指示该条目是否仍然"存活"（关联的进程仍在运行）。 |

## 备注

此静态变量是 AffinityServiceRust 错误去重系统的核心数据结构。服务循环反复向运行中的进程应用配置规则，许多操作可能在每次迭代中都以相同的错误失败（例如，对受保护进程返回 `ACCESS_DENIED`）。如果没有去重机制，日志文件将被这些重复错误所主导。

### 两级映射结构

- **外层映射 (`HashMap<u32, …>`)：** 以进程 ID 为键。每个至少遇到过一次错误的运行中进程在此处都有一个条目。
- **内层映射 (`HashMap<ApplyFailEntry, bool>`)：** 以 [ApplyFailEntry](ApplyFailEntry.md) 为键，后者组合了 `tid`、`process_name`、`operation` 和 `error_code`。`bool` 值跟踪存活状态：`true` 表示该进程在最近的快照中仍在运行，`false` 表示未在运行。

### 生命周期

1. **插入：** [is_new_error](is_new_error.md) 在遇到给定 PID 尚未存在的错误组合时插入新条目。它返回 `true` 以通知调用者该错误应被记录。
2. **去重：** 在后续调用中，[is_new_error](is_new_error.md) 找到已存在的条目并返回 `false`，抑制重复的日志消息。该条目的 `alive` 标志被设置为 `true`，表示进程仍然活跃。
3. **清除：** [purge_fail_map](purge_fail_map.md) 被定期调用以移除不再运行的进程的条目。它将所有条目标记为死亡，然后将仍在运行的进程的条目重新标记为存活，最后移除所有仍然为死亡状态的条目。

### PID 重用处理

如果一个 PID 被一个不同名称的新进程重用，[is_new_error](is_new_error.md) 通过不变量检查（一个 PID 的内层映射中所有条目应共享相同的 `process_name`）检测到名称不匹配。当发现不匹配时，内层映射在插入新条目之前被**清空**，防止过时的去重状态抑制新进程的合法首次错误。

### 宏访问

[get_pid_map_fail_entry_set!](get_pid_map_fail_entry_set.md) 宏提供了一个便捷包装器，用于锁定互斥锁并返回 `MutexGuard`：

```logging.rs
macro_rules! get_pid_map_fail_entry_set {
    () => {
        $crate::logging::PID_MAP_FAIL_ENTRY_SET.lock().unwrap()
    };
}
```

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| Crate 依赖 | `once_cell`（`Lazy`）、`std::sync::Mutex`、`std::collections::HashMap` |
| 写入方 | [is_new_error](is_new_error.md) |
| 清除方 | [purge_fail_map](purge_fail_map.md) |
| 读取方 | [is_new_error](is_new_error.md)、[purge_fail_map](purge_fail_map.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 失败条目键结构体 | [ApplyFailEntry](ApplyFailEntry.md) |
| 错误去重逻辑 | [is_new_error](is_new_error.md) |
| 过期条目清理 | [purge_fail_map](purge_fail_map.md) |
| Windows API 操作标识符 | [Operation](Operation.md) |
| 受保护访问的宏 | [get_pid_map_fail_entry_set!](get_pid_map_fail_entry_set.md) |
| logging 模块概述 | [logging 模块](README.md) |