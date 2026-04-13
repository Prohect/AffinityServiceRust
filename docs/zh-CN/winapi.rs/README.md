# winapi 模块 (AffinityServiceRust)

`winapi` 模块为 AffinityServiceRust 中使用的 Windows API 函数提供安全的 Rust 封装，涵盖进程和线程句柄管理、CPU 集合转换、特权提权以及底层系统查询。所有持有句柄的类型均实现了 `Drop` 以自动清理资源，所有公共函数均将 Windows 错误码转换为 Rust 友好的返回类型。该模块还公开了延迟初始化的全局缓存，用于 CPU 集合拓扑和每进程模块映射。

## 结构体

| 结构体 | 描述 |
|--------|------|
| [CpuSetData](CpuSetData.md) | 将 Windows CPU Set ID 与其对应的逻辑处理器索引配对。 |
| [ProcessHandle](ProcessHandle.md) | RAII 容器，持有最多四个不同访问级别的进程句柄。析构时自动关闭所有有效句柄。 |
| [ThreadHandle](ThreadHandle.md) | RAII 容器，持有最多四个不同访问级别的线程句柄。析构时自动关闭所有有效句柄。 |

## 函数

| 函数 | 描述 |
|------|------|
| [get_process_handle](get_process_handle.md) | 以多种访问级别打开进程，并返回一个 [ProcessHandle](ProcessHandle.md)。 |
| [get_thread_handle](get_thread_handle.md) | 以多种访问级别打开线程，并返回一个 [ThreadHandle](ThreadHandle.md)。 |
| [try_open_thread](try_open_thread.md) | 尝试以单一指定访问权限打开线程，失败时返回无效句柄。 |
| [get_cpu_set_information](get_cpu_set_information.md) | 返回对延迟初始化的系统 CPU 集合数据的引用。 |
| [cpusetids_from_indices](cpusetids_from_indices.md) | 将逻辑 CPU 索引转换为 Windows CPU Set ID。 |
| [cpusetids_from_mask](cpusetids_from_mask.md) | 将亲和性掩码转换为 Windows CPU Set ID。 |
| [indices_from_cpusetids](indices_from_cpusetids.md) | 将 Windows CPU Set ID 转换回逻辑 CPU 索引。 |
| [mask_from_cpusetids](mask_from_cpusetids.md) | 将 Windows CPU Set ID 转换为亲和性掩码。 |
| [filter_indices_by_mask](filter_indices_by_mask.md) | 过滤 CPU 索引切片，仅保留亲和性掩码中存在的索引。 |
| [is_running_as_admin](is_running_as_admin.md) | 检查当前进程是否以管理员（提权）特权运行。 |
| [request_uac_elevation](request_uac_elevation.md) | 通过 `Start-Process -Verb RunAs` 重新启动进程以请求 UAC 提权。 |
| [enable_debug_privilege](enable_debug_privilege.md) | 在当前进程令牌上启用 `SeDebugPrivilege`。 |
| [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) | 在当前进程令牌上启用 `SeIncreaseBasePriorityPrivilege`。 |
| [is_affinity_unset](is_affinity_unset.md) | 检查进程的亲和性掩码是否等于系统默认值（所有 CPU）。 |
| [get_thread_start_address](get_thread_start_address.md) | 通过 `NtQueryInformationThread` 查询线程的起始地址。 |
| [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) | 以处理器组感知的方式设置线程的理想处理器。 |
| [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) | 以处理器组感知的方式获取线程当前的理想处理器。 |
| [resolve_address_to_module](resolve_address_to_module.md) | 将内存地址解析为模块名加偏移量的字符串（例如 `"kernel32.dll+0x1A3"`）。 |
| [drop_module_cache](drop_module_cache.md) | 清除特定进程的已缓存模块列表。 |
| [terminate_child_processes](terminate_child_processes.md) | 终止当前进程的所有子进程。 |
| [enumerate_process_modules](enumerate_process_modules.md) | 枚举目标进程中已加载的模块，返回每个模块的基地址、大小和名称。 |
| [set_timer_resolution](set_timer_resolution.md) | 通过 `NtSetTimerResolution` 设置系统定时器分辨率。 |

## 静态变量

| 静态变量 | 描述 |
|----------|------|
| [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) | 延迟初始化的、受互斥锁保护的 [CpuSetData](CpuSetData.md) 条目向量，表示系统的 CPU 集合拓扑。 |
| [MODULE_CACHE](MODULE_CACHE.md) | 每进程的已枚举模块地址范围和名称缓存，由 [resolve_address_to_module](resolve_address_to_module.md) 使用。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 进程快照和枚举 | [process 模块](../process.rs/README.md) |
| 使用句柄和 CPU 集合的规则应用 | [apply 模块](../apply.rs/README.md) |
| 主线程调度器（使用线程句柄和 CPU 集合） | [scheduler 模块](../scheduler.rs/README.md) |
| 配置解析（CPU 规格、进程规则） | [config 模块](../config.rs/README.md) |
| 优先级和 IO/内存优先级枚举 | [priority 模块](../priority.rs/README.md) |
| 错误码格式化辅助函数 | [error_codes 模块](../error_codes.rs/README.md) |
| CLI 参数解析 | [cli 模块](../cli.rs/README.md) |
| 服务主循环 | [main 模块](../main.rs/README.md) |