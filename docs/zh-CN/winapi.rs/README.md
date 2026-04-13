# winapi.rs 模块 (winapi.rs)

`winapi` 模块提供底层 Windows API 封装，用于进程和线程句柄管理、CPU 集枚举与转换、权限提升以及模块地址解析。它是应用程序逻辑与 Win32/NT API 之间的主要接口。

## 概述

本模块封装了应用程序使用的所有直接 Windows API 调用，按以下功能区域组织：

- **计时器分辨率** — [`set_timer_resolution`](set_timer_resolution.md)
- **进程句柄** — [`ProcessHandle`](ProcessHandle.md)、[`get_process_handle`](get_process_handle.md)
- **线程句柄** — [`ThreadHandle`](ThreadHandle.md)、[`get_thread_handle`](get_thread_handle.md)、[`try_open_thread`](try_open_thread.md)
- **CPU 集信息** — [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md)、[`get_cpu_set_information`](get_cpu_set_information.md)、[`CpuSetData`](CpuSetData.md)
- **CPU 集 / 索引 / 掩码转换** — [`cpusetids_from_indices`](cpusetids_from_indices.md)、[`cpusetids_from_mask`](cpusetids_from_mask.md)、[`indices_from_cpusetids`](indices_from_cpusetids.md)、[`mask_from_cpusetids`](mask_from_cpusetids.md)、[`filter_indices_by_mask`](filter_indices_by_mask.md)
- **权限与提升** — [`is_running_as_admin`](is_running_as_admin.md)、[`request_uac_elevation`](request_uac_elevation.md)、[`enable_debug_privilege`](enable_debug_privilege.md)、[`enable_inc_base_priority_privilege`](enable_inc_base_priority_privilege.md)
- **亲和性查询** — [`is_affinity_unset`](is_affinity_unset.md)
- **线程理想处理器** — [`set_thread_ideal_processor_ex`](set_thread_ideal_processor_ex.md)、[`get_thread_ideal_processor_ex`](get_thread_ideal_processor_ex.md)、[`get_thread_start_address`](get_thread_start_address.md)
- **模块解析** — [`MODULE_CACHE`](MODULE_CACHE.md)、[`resolve_address_to_module`](resolve_address_to_module.md)、[`drop_module_cache`](drop_module_cache.md)、[`enumerate_process_modules`](enumerate_process_modules.md)
- **进程清理** — [`terminate_child_processes`](terminate_child_processes.md)

## 项目列表

### 结构体

| 名称 | 说明 |
| --- | --- |
| [CpuSetData](CpuSetData.md) | 存储 CPU 集 ID 及其对应的逻辑处理器索引。 |
| [ProcessHandle](ProcessHandle.md) | 持有进程的读/写 HANDLE，包含受限和完全访问变体。实现 `Drop`。 |
| [ThreadHandle](ThreadHandle.md) | 持有线程的读/写 HANDLE，包含受限和完全访问变体。 |

### 静态变量

| 名称 | 说明 |
| --- | --- |
| [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) | 延迟初始化的、互斥锁保护的系统全部 CPU 集向量。 |
| [MODULE_CACHE](MODULE_CACHE.md) | 每进程的已枚举模块基地址/结束地址和名称缓存。 |

### 函数

| 名称 | 说明 |
| --- | --- |
| [get_process_handle](get_process_handle.md) | 打开进程并返回包含可用访问级别的 [`ProcessHandle`](ProcessHandle.md)。 |
| [get_thread_handle](get_thread_handle.md) | 打开线程并返回包含可用访问级别的 [`ThreadHandle`](ThreadHandle.md)。 |
| [try_open_thread](try_open_thread.md) | 尝试以指定访问权限打开线程，失败时记录去重错误。 |
| [get_cpu_set_information](get_cpu_set_information.md) | 返回全局 [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) 静态变量的引用。 |
| [cpusetids_from_indices](cpusetids_from_indices.md) | 将逻辑处理器索引转换为 CPU 集 ID。 |
| [cpusetids_from_mask](cpusetids_from_mask.md) | 将亲和性位掩码转换为 CPU 集 ID。 |
| [indices_from_cpusetids](indices_from_cpusetids.md) | 将 CPU 集 ID 转换回逻辑处理器索引。 |
| [mask_from_cpusetids](mask_from_cpusetids.md) | 将 CPU 集 ID 转换为亲和性位掩码。 |
| [filter_indices_by_mask](filter_indices_by_mask.md) | 按亲和性掩码过滤 CPU 索引列表，仅保留掩码中存在的索引。 |
| [is_running_as_admin](is_running_as_admin.md) | 检查当前进程是否以管理员权限运行。 |
| [request_uac_elevation](request_uac_elevation.md) | 通过 PowerShell 启动 UAC 提升的应用程序实例。 |
| [enable_debug_privilege](enable_debug_privilege.md) | 为当前进程令牌启用 `SeDebugPrivilege`。 |
| [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) | 为当前进程令牌启用 `SeIncreaseBasePriorityPrivilege`。 |
| [is_affinity_unset](is_affinity_unset.md) | 检查进程是否具有默认（全核心）亲和性掩码。 |
| [get_thread_start_address](get_thread_start_address.md) | 通过 `NtQueryInformationThread` 查询线程起始地址。 |
| [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) | 按组和编号设置线程的理想处理器。 |
| [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) | 查询线程当前的理想处理器。 |
| [resolve_address_to_module](resolve_address_to_module.md) | 将内存地址解析为 `"module.dll+0xABC"` 格式的字符串。 |
| [drop_module_cache](drop_module_cache.md) | 从 [`MODULE_CACHE`](MODULE_CACHE.md) 中移除进程条目。 |
| [terminate_child_processes](terminate_child_processes.md) | 终止 UAC 提升过程中产生的孤立控制台宿主进程。 |
| [enumerate_process_modules](enumerate_process_modules.md) | 枚举进程的所有已加载模块，返回基地址/结束地址和名称。 |
| [set_timer_resolution](set_timer_resolution.md) | 通过 `NtSetTimerResolution` 设置系统计时器分辨率。 |

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | `src/winapi.rs` |
| **调用方** | [`apply_config_process_level`](../main.rs/apply_config_process_level.md)、[`apply_config_thread_level`](../main.rs/apply_config_thread_level.md)、[`apply.rs`](../apply.rs/README.md) 中的函数、[`main`](../main.rs/main.md) |
| **关键依赖** | `windows` crate、[`ProcessConfig`](../config.rs/ProcessConfig.md)、[`Operation`](../logging.rs/Operation.md)、[`is_new_error`](../logging.rs/is_new_error.md) |