# apply_config_process_level 函数 (main.rs)

将进程级设置应用到目标进程。这些设置是每进程一次性的——在首次检测到进程时应用一次，后续迭代不再重复应用，除非配置被重新加载。

## 语法

```rust
fn apply_config_process_level(
    pid: u32,
    config: &ProcessConfig,
    process: &mut ProcessEntry,
    dry_run: bool,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid` — 目标进程 ID。

`config` — 指向 [`ProcessConfig`](../config.rs/ProcessConfig.md) 的引用。

`process` — 指向 [`ProcessEntry`](../process.rs/ProcessEntry.md) 的可变引用。

`dry_run` — 为 `true` 时仅记录更改，不实际应用。

`apply_config_result` — 指向 [`ApplyConfigResult`](../apply.rs/ApplyConfigResult.md) 的可变引用，收集更改和错误。

## 备注

按以下顺序应用设置：
1. [`apply_priority`](../apply.rs/apply_priority.md) — 进程优先级类
2. [`apply_affinity`](../apply.rs/apply_affinity.md) — 硬 CPU 亲和性掩码（附带线程理想处理器重置）
3. [`apply_process_default_cpuset`](../apply.rs/apply_process_default_cpuset.md) — 软 CPU 集偏好
4. [`apply_io_priority`](../apply.rs/apply_io_priority.md) — I/O 优先级
5. [`apply_memory_priority`](../apply.rs/apply_memory_priority.md) — 内存页面优先级

主循环通过 `process_level_applied: HashSet<u32>` 跟踪哪些 PID 已应用进程级设置。ETW 进程启动事件将 PID 添加到 `process_level_pending`，触发立即应用，绕过 grade 调度。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/main.rs |
| **源码行** | L48–L74 |
| **调用方** | [`main`](main.md) 主循环 |

## 另请参阅

- [apply_config_thread_level](apply_config_thread_level.md)
- [apply.rs 模块概述](../apply.rs/README.md)
- [main.rs 模块概述](README.md)