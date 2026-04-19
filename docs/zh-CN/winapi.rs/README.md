# winapi 模块 (AffinityServiceRust)

`winapi` 模块为 AffinityServiceRust 中使用的 Windows API 函数提供安全的 Rust 封装，用于进程和线程句柄管理、CPU 集合拓扑查询和转换、特权提升、亲和性检查、理想处理器分配、模块枚举、地址解析、计时器分辨率控制和子进程清理。所有持有句柄的类型都通过 `Drop` 实现 RAII，以防止资源泄漏。

## 外部 FFI

该模块直接链接到 `ntdll.dll`，以使用 `windows` crate 未公开的 NT API：

| 函数 | 描述 |
|----------|-------------|
| `NtQueryInformationProcess` | 查询进程级别的信息类（IO 优先级、内存优先级等）。 |
| `NtQueryInformationThread` | 查询线程级别的信息类（通过 `ThreadQuerySetWin32StartAddress` 获取起始地址）。 |
| `NtSetInformationProcess` | 设置进程级别的信息类（IO 优先级、内存优先级）。 |
| `NtSetTimerResolution` | 通过指定的间隔设置全局 Windows 计时器分辨率。 |

## 静态变量

| 名称 | 类型 | 描述 |
|------|------|-------------|
| `CPU_SET_INFORMATION` | `Lazy<Mutex<Vec<CpuSetData>>>` | 从 `GetSystemCpuSetInformation` 获取的缓存系统 CPU 集合拓扑。在首次访问时填充一次，并在所有 CPU 索引 ↔ CPU 集合 ID 转换中重复使用。 |
| `MODULE_CACHE` | `Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>>` | 按 PID 缓存的模块基地址范围和名称。在每次 [resolve_address_to_module](resolve_address_to_module.md) 调用时填充，并通过 [drop_module_cache](drop_module_cache.md) 在进程退出时清除。 |

## 结构体

| 名称 | 描述 |
|------|-------------|
| [CpuSetData](CpuSetData.md) | 轻量级缓存条目，将 CPU 集合 ID 与其逻辑处理器索引配对。 |
| [ProcessHandle](ProcessHandle.md) | RAII 封装，包含最多四个不同访问级别的进程 HANDLE。 |
| [ThreadHandle](ThreadHandle.md) | RAII 封装，包含最多四个不同访问级别的线程 HANDLE。 |

## 函数 — 句柄管理

| 名称 | 描述 |
|------|-------------|
| [get_process_handle](get_process_handle.md) | 以多个访问级别打开进程，返回 [ProcessHandle](ProcessHandle.md)。 |
| [get_thread_handle](get_thread_handle.md) | 以多个访问级别打开线程，返回 [ThreadHandle](ThreadHandle.md)。 |
| [try_open_thread](try_open_thread.md) | 低级辅助函数，以指定的访问权限打开单个线程句柄。 |

## 函数 — CPU 集合转换

| 名称 | 描述 |
|------|-------------|
| [cpusetids_from_indices](cpusetids_from_indices.md) | 将逻辑 CPU 索引（0、1、2…）转换为不透明的 Windows CPU 集合 ID。 |
| [cpusetids_from_mask](cpusetids_from_mask.md) | 将亲和性位掩码转换为 CPU 集合 ID。 |
| [indices_from_cpusetids](indices_from_cpusetids.md) | 将 CPU 集合 ID 转换回逻辑 CPU 索引。 |
| [mask_from_cpusetids](mask_from_cpusetids.md) | 将 CPU 集合 ID 转换为亲和性位掩码。 |
| [filter_indices_by_mask](filter_indices_by_mask.md) | 筛选列表中的 CPU 索引，仅保留在亲和性掩码中存在的索引。 |

## 函数 — 特权与提升

| 名称 | 描述 |
|------|-------------|
| [is_running_as_admin](is_running_as_admin.md) | 检查当前进程是否具有提升的管理员令牌。 |
| [request_uac_elevation](request_uac_elevation.md) | 使用 `runas` 动词重新启动当前进程以触发 UAC 提升。 |
| [enable_debug_privilege](enable_debug_privilege.md) | 在进程令牌上启用 `SeDebugPrivilege`。 |
| [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) | 在进程令牌上启用 `SeIncreaseBasePriorityPrivilege`。 |

## 函数 — 进程与线程检查

| 名称 | 描述 |
|------|-------------|
| [is_affinity_unset](is_affinity_unset.md) | 检查进程的亲和性掩码是否等于完整系统掩码（即未应用任何限制）。 |
| [get_thread_start_address](get_thread_start_address.md) | 通过 `NtQueryInformationThread` 获取线程的 Win32 起始地址。 |
| [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) | `SetThreadIdealProcessorEx` 的封装，设置理想处理器提示。 |
| [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) | `GetThreadIdealProcessorEx` 的封装，查询当前理想处理器。 |

## 函数 — 模块解析

| 名称 | 描述 |
|------|-------------|
| [resolve_address_to_module](resolve_address_to_module.md) | 使用缓存的模块数据将虚拟地址解析为 `"module.dll+0xOffset"` 字符串。 |
| [drop_module_cache](drop_module_cache.md) | 移除特定 PID 的缓存模块数据。 |
| [enumerate_process_modules](enumerate_process_modules.md) | 列出进程中的所有模块，包括基地址、大小和名称。 |

## 函数 — 系统工具

| 名称 | 描述 |
|------|-------------|
| [terminate_child_processes](terminate_child_processes.md) | 终止当前进程的所有子进程（启动时清理）。 |
| [set_timer_resolution](set_timer_resolution.md) | 通过 `NtSetTimerResolution` 设置 Windows 全局计时器分辨率。 |

## 参见

| 主题 | 链接 |
|-------|------|
| 进程快照模块 | [process.rs](../process.rs/README.md) |
| Prime 线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 配置应用 | [apply.rs](../apply.rs/README.md) |
| 优先级类型定义 | [priority.rs](../priority.rs/README.md) |
| CLI 参数解析 | [cli.rs](../cli.rs/README.md) |
| 错误代码格式化 | [error_codes.rs](../error_codes.rs/README.md) |
| 日志和错误去重 | [logging.rs](../logging.rs/README.md) |

*文档针对提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*