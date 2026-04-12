# PID_MAP_FAIL_ENTRY_SET 静态变量 (logging.rs)

错误去重的两级映射。按进程 PID 索引，存储每个进程遇到的唯一错误条目及其存活状态，防止同一错误在循环迭代中被反复记录。

## 语法

```rust
static PID_MAP_FAIL_ENTRY_SET: Lazy<Mutex<HashMap<u32, HashMap<ApplyFailEntry, bool>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
```

## 成员

该静态变量在 `Mutex` 后持有一个两级 `HashMap`，结构如下：

- **外层键** — `u32`：进程 PID。
- **内层键** — [`ApplyFailEntry`](ApplyFailEntry.md)：复合键，包含 `(pid, tid, process_name, operation, error_code)`。
- **内层值** — `bool`：存活标志，用于 [`purge_fail_map`](purge_fail_map.md) 的标记-清除算法。

### 示例结构

```
pid 1234 → {
    (1234, 5678, "game.exe", OpenProcess2processQueryLimitedInformation, 5) → true,
    (1234, 5679, "game.exe", SetThreadSelectedCpuSets, 5) → false,
}
pid 4567 → {
    (4567, 0, "app.exe", SetPriorityClass, 5) → true,
}
```

`tid` 字段的存在使得同一进程的不同线程上发生的相同操作错误能够分别去重。例如，线程 5678 上的 `OpenProcess` 错误和线程 5679 上的 `OpenProcess` 错误被视为不同的条目。

## 备注

### 去重流程

1. 当 [`apply_config`](../main.rs/apply_config.md) 过程中遇到错误时，[`is_new_error`](is_new_error.md) 被调用。
2. `is_new_error` 根据参数构造一个 [`ApplyFailEntry`](ApplyFailEntry.md) 键，然后在 `PID_MAP_FAIL_ENTRY_SET` 中查找。
3. 如果该键已存在，返回 `false`——错误已被记录过，调用方跳过日志写入。
4. 如果该键不存在，将其插入映射并返回 `true`——调用方写入一条日志。

### PID 重用处理

Windows 会重用进程 PID。当 `is_new_error` 检测到同一 PID 下的进程名称与映射中已存储的不同时，会清除该 PID 下的所有旧条目，避免将新进程的错误误判为已记录。

### 清除机制

每次主循环迭代中，[`purge_fail_map`](purge_fail_map.md) 执行标记-清除操作：

1. **标记全部死亡** — 将所有条目的 `bool` 值设为 `false`。
2. **标记存活** — 遍历当前运行的进程列表，将匹配的 PID 条目标记为 `true`。
3. **移除死亡** — 删除所有仍标记为 `false` 的 PID 及其全部条目。

这防止映射因已终止进程的累积条目而无限增长。

### 线程安全

所有对映射的访问都通过 `Mutex` 同步。[`is_new_error`](is_new_error.md) 和 [`purge_fail_map`](purge_fail_map.md) 在操作期间持有锁，完成后立即释放。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/logging.rs |
| **源码行** | L70 |
| **写入方** | [`is_new_error`](is_new_error.md)、[`purge_fail_map`](purge_fail_map.md) |
| **读取方** | [`is_new_error`](is_new_error.md) |

## 另请参阅

- [ApplyFailEntry 结构体](ApplyFailEntry.md)
- [Operation 枚举](Operation.md)
- [is_new_error 函数](is_new_error.md)
- [purge_fail_map 函数](purge_fail_map.md)
- [FINDS_FAIL_SET 静态变量](FINDS_FAIL_SET.md)
- [logging.rs 模块概述](README.md)