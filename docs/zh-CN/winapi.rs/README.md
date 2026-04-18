# winapi 模块 (AffinityServiceRust)

`winapi` 模块提供底层 Windows API 封装，用于进程和线程句柄管理、CPU 集合操作、权限提升、线程检查、模块解析和定时器配置。它作为 AffinityServiceRust 服务逻辑与 Windows 操作系统之间的主要接口，将不安全的 FFI 调用封装在安全的 Rust 抽象之后。

## 函数

| 函数 | 描述 |
|----------|-------------|
| [get_process_handle](get_process_handle.md) | 为给定 PID 打开多个进程句柄（读/写、受限/完整）。 |
| [get_thread_handle](get_thread_handle.md) | 为给定 TID 打开多个线程句柄（读/写、受限/完整）。 |
| [try_open_thread](try_open_thread.md) | 尝试使用指定的访问权限打开单个线程句柄。 |
| [get_cpu_set_information](get_cpu_set_information.md) | 返回对延迟初始化的系统 CPU 集合信息缓存的引用。 |
| [cpusetids_from_indices](cpusetids_from_indices.md) | 将逻辑 CPU 索引转换为 Windows CPU Set ID。 |
| [cpusetids_from_mask](cpusetids_from_mask.md) | 将亲和性位掩码转换为 Windows CPU Set ID。 |
| [indices_from_cpusetids](indices_from_cpusetids.md) | 将 Windows CPU Set ID 转换回逻辑 CPU 索引。 |
| [mask_from_cpusetids](mask_from_cpusetids.md) | 将 Windows CPU Set ID 转换为亲和性位掩码。 |
| [filter_indices_by_mask](filter_indices_by_mask.md) | 过滤 CPU 索引，仅保留亲和性掩码允许的索引。 |
| [is_running_as_admin](is_running_as_admin.md) | 检查当前进程是否以管理员权限运行。 |
| [request_uac_elevation](request_uac_elevation.md) | 通过 UAC 提示以提升的权限重新启动进程。 |
| [enable_debug_privilege](enable_debug_privilege.md) | 在当前进程令牌上启用 `SeDebugPrivilege`。 |
| [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) | 在当前进程令牌上启用 `SeIncreaseBasePriorityPrivilege`。 |
| [is_affinity_unset](is_affinity_unset.md) | 检查进程是否使用默认（全 CPU）亲和性掩码。 |
| [get_thread_start_address](get_thread_start_address.md) | 通过 `NtQueryInformationThread` 获取线程的起始地址。 |
| [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) | 使用处理器组和编号设置线程的理想处理器。 |
| [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) | 获取线程的理想处理器。 |
| [resolve_address_to_module](resolve_address_to_module.md) | 将内存地址解析为带偏移量的模块名称（例如 `kernel32.dll+0x345`）。 |
| [drop_module_cache](drop_module_cache.md) | 移除给定 PID 的缓存模块列表。 |
| [terminate_child_processes](terminate_child_processes.md) | 终止当前进程产生的任何子进程。 |
| [enumerate_process_modules](enumerate_process_modules.md) | 枚举进程的所有已加载模块，返回基地址、大小和名称。 |
| [set_timer_resolution](set_timer_resolution.md) | 通过 `NtSetTimerResolution` 设置系统定时器分辨率。 |

## 结构体

| 结构体 | 描述 |
|--------|-------------|
| [CpuSetData](CpuSetData.md) | 持有 CPU Set ID 及其对应的逻辑处理器索引。 |
| [ProcessHandle](ProcessHandle.md) | 一组具有不同访问级别的进程句柄的 RAII 包装器。 |
| [ThreadHandle](ThreadHandle.md) | 一组具有不同访问级别的线程句柄的 RAII 包装器。 |

## 静态变量

| 静态变量 | 描述 |
|--------|-------------|
| [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) | 从 `GetSystemCpuSetInformation` 查询的系统 CPU 集合数据的延迟初始化缓存。 |
| [MODULE_CACHE](MODULE_CACHE.md) | 用于地址解析的每 PID 已加载模块信息（基地址、大小、名称）缓存。 |

## 另请参阅

| 参考 | 链接 |
|-----------|------|
| process 模块 | [process.rs](../process.rs/README.md) |
| event_trace 模块 | [event_trace.rs](../event_trace.rs/README.md) |
| logging 模块 | [logging.rs](../logging.rs/README.md) |
| collections 模块 | [collections.rs](../collections.rs/README.md) |
| error_codes 模块 | [error_codes.rs](../error_codes.rs/README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
