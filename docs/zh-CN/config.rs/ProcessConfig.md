# ProcessConfig 结构体 (config.rs)

单个进程的完整配置，定义优先级、CPU 亲和性、CPU 集合、主线程调度、I/O 和内存优先级以及理想处理器分配规则。

## 语法

```rust
pub struct ProcessConfig {
    pub name: String,
    pub priority: ProcessPriority,
    pub affinity_cpus: Vec<u32>,
    pub cpu_set_cpus: Vec<u32>,
    pub cpu_set_reset_ideal: bool,
    pub prime_threads_cpus: Vec<u32>,
    pub prime_threads_prefixes: Vec<PrimePrefix>,
    pub track_top_x_threads: i32,
    pub io_priority: IOPriority,
    pub memory_priority: MemoryPriority,
    pub ideal_processor_rules: Vec<IdealProcessorRule>,
}
```

## 参数

`name`

小写的进程可执行文件名称（例如 `"chrome.exe"`）。用作匹配运行中进程的查找键。

`priority`

要应用的 [ProcessPriority](../priority.rs/ProcessPriority.md) 优先级类。设置为 `ProcessPriority::None` 表示不更改优先级。

`affinity_cpus`

用于进程亲和性掩码的 CPU 索引列表。通过 `SetProcessAffinityMask` 应用。空向量表示不更改亲和性。通过 [resolve_cpu_spec](resolve_cpu_spec.md) 从配置规则的 affinity 字段解析。

`cpu_set_cpus`

用于进程默认 CPU 集合的 CPU 索引列表。通过 `SetProcessDefaultCpuSets` 应用。空向量表示不更改 CPU 集合。从配置规则的 cpuset 字段解析。

`cpu_set_reset_ideal`

当为 `true` 时，应用 CPU 集合后会调用 [reset_thread_ideal_processors](../apply.rs/reset_thread_ideal_processors.md)，将线程的理想处理器分布到 `cpu_set_cpus` 上。通过在配置规则的 cpuset 字段值前添加 `@` 前缀启用。

`prime_threads_cpus`

主线程调度器将顶级线程提升到的 CPU 索引列表。空向量表示禁用主线程调度。

`prime_threads_prefixes`

[PrimePrefix](PrimePrefix.md) 规则列表，根据线程的启动模块控制哪些线程有资格进行主线程调度。当仅存在默认条目（空前缀）时，所有线程均符合条件。

`track_top_x_threads`

按 CPU 周期使用量跟踪的顶级线程数。正值同时启用跟踪和主线程调度。负值仅启用跟踪（不执行提升/降级）。零表示禁用跟踪。

`io_priority`

要应用的 [IOPriority](../priority.rs/IOPriority.md)。设置为 `IOPriority::None` 表示不更改 I/O 优先级。

`memory_priority`

要应用的 [MemoryPriority](../priority.rs/MemoryPriority.md)。设置为 `MemoryPriority::None` 表示不更改内存优先级。

`ideal_processor_rules`

[IdealProcessorRule](IdealProcessorRule.md) 条目列表，用于将理想处理器分配给线程。规则按顺序评估；第一个匹配的规则（按模块前缀）决定用于轮询分配理想处理器的 CPU 集合。

## 返回值

不适用。此为数据结构。

## 备注

`ProcessConfig` 实例由 [parse_and_insert_rules](parse_and_insert_rules.md) 在配置文件解析期间构造，并存储在 [ConfigResult](ConfigResult.md).configs 中，以等级和进程名称为键。

[read_config](read_config.md) 解析的配置规则格式为：

```
name:priority:affinity:cpuset:prime:io_priority:memory_priority:ideal:grade
```

每个字段直接映射到一个 `ProcessConfig` 字段。`affinity` 之后的字段均为可选，默认值为无操作。

在运行时，[apply_config](../main.rs/apply_config.md) 将此结构体传递给 [apply](../apply.rs/README.md) 模块中的各个应用函数：[apply_priority](../apply.rs/apply_priority.md)、[apply_affinity](../apply.rs/apply_affinity.md)、[apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md)、[apply_prime_threads](../apply.rs/apply_prime_threads.md)、[apply_io_priority](../apply.rs/apply_io_priority.md)、[apply_memory_priority](../apply.rs/apply_memory_priority.md) 和 [apply_ideal_processors](../apply.rs/apply_ideal_processors.md)。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/config.rs |
| **定义位置** | 第 36 行 |
| **派生宏** | `Debug`, `Clone` |
| **使用者** | [apply_config](../main.rs/apply_config.md)、[parse_and_insert_rules](parse_and_insert_rules.md)、[ConfigResult](ConfigResult.md) |