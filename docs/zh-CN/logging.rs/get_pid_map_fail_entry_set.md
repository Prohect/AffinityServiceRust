# get_pid_map_fail_entry_set! 宏 (logging.rs)

便捷宏，锁定 [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) 互斥锁并返回 `MutexGuard<HashMap<u32, HashMap<ApplyFailEntry, bool>>>`。这提供了对全局按 PID 组织的失败去重映射的符合人体工程学的、一致的访问方式，无需调用者拼写完整的锁定与 unwrap 表达式。

## 语法

```logging.rs
#[macro_export]
macro_rules! get_pid_map_fail_entry_set {
    () => {
        $crate::logging::PID_MAP_FAIL_ENTRY_SET.lock().unwrap()
    };
}
```

## 返回值

返回 `MutexGuard<HashMap<u32, HashMap<ApplyFailEntry, bool>>>`。守卫在其生命周期内持有锁，并解引用为内部的 `HashMap`。当守卫被丢弃（离开作用域）时，互斥锁自动释放。

## 备注

- 该宏对互斥锁的 lock 结果调用 `.unwrap()`。如果互斥锁被污染（先前的持有者在持有锁时发生了 panic），这将导致 panic。在 AffinityServiceRust 中，正常运行期间不会出现互斥锁污染，因为服务在日志记录路径中不使用基于 `panic` 的错误处理。
- 返回的守卫提供对两级映射的读取和写入访问。[is_new_error](is_new_error.md) 和 [purge_fail_map](purge_fail_map.md) 等调用者使用此宏在映射中插入、更新和移除条目。
- `#[macro_export]` 属性使此宏在 crate 根级别可用。其他模块的调用者无需模块前缀即可调用——例如 `get_pid_map_fail_entry_set!()`——而非 `logging::get_pid_map_fail_entry_set!()`。
- 由于该宏获取互斥锁，调用者应尽量缩小返回守卫的作用域，避免不必要地持有锁过长时间。特别要避免在持有此守卫的同时调用其他获取锁的函数（如访问 [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) 或 [USE_CONSOLE](USE_CONSOLE.md) 的函数），以防止潜在的死锁。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| Crate 依赖 | `std::sync::Mutex`、`std::collections::HashMap` |
| 底层静态变量 | [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) |
| 使用者 | [is_new_error](is_new_error.md)、[purge_fail_map](purge_fail_map.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 全局失败跟踪映射 | [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) |
| 失败条目键结构体 | [ApplyFailEntry](ApplyFailEntry.md) |
| 错误去重检查 | [is_new_error](is_new_error.md) |
| 过期条目清理 | [purge_fail_map](purge_fail_map.md) |
| Windows API 操作标识符 | [Operation](Operation.md) |
| logging 模块概述 | [logging 模块](README.md) |