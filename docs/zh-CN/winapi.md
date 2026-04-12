# WinAPI 模块文档

Windows API 包装器和工具函数。

## 概述

本模块围绕 Windows API 提供安全包装器：
- 进程/线程句柄管理
- 特权管理
- CPU 集操作
- 模块枚举和地址解析
- UAC 提升

## 调用者

- [main.rs](main.md) - UAC 提升、计时器分辨率、特权启用
- [apply.rs](apply.md) - 进程/线程句柄、CPU 集、理想处理器
- [scheduler.rs](scheduler.md) - 线程跟踪的模块解析
- [logging.rs](logging.md) - 错误跟踪

## NTDLL 导入

从 ntdll.dll 导入的原始 NT API 函数：

```rust
#[link(name = "ntdll")]
unsafe extern "system" {
    pub fn NtQueryInformationProcess(...)
    pub fn NtQueryInformationThread(...)
    pub fn NtSetInformationProcess(...)
    pub fn NtSetTimerResolution(...)
}
```

**被调用者：**
- [`apply_io_priority()`](apply.md#apply_io_priority) - 进程 I/O 优先级查询/设置
- [`get_thread_start_address()`](winapi.md#get_thread_start_address) - 线程起始地址查询
- [main.rs](main.md) - 计时器分辨率设置

## 数据结构

### ProcessHandle

带自动清理的安全句柄容器。

```rust
pub struct ProcessHandle {
    pub r_limited_handle: HANDLE,      // PROCESS_QUERY_LIMITED_INFORMATION
    pub r_handle: Option<HANDLE>,      // PROCESS_QUERY_INFORMATION（可选）
    pub w_limited_handle: HANDLE,      // PROCESS_SET_LIMITED_INFORMATION
    pub w_handle: Option<HANDLE>,      // PROCESS_SET_INFORMATION（可选）
}
```

**Drop 实现：**自动关闭所有有效句柄。

**注意：**受限句柄始终存在。完整句柄对于受保护进程或无提升可能为 `None`。

### ThreadHandle

带自动清理的线程安全句柄容器。

```rust
pub struct ThreadHandle {
    pub r_limited_handle: HANDLE,      // THREAD_QUERY_LIMITED_INFORMATION（始终有效）
    pub r_handle: HANDLE,              // THREAD_QUERY_INFORMATION（失败时无效）
    pub w_limited_handle: HANDLE,      // THREAD_SET_LIMITED_INFORMATION（失败时无效）
    pub w_handle: HANDLE,              // THREAD_SET_INFORMATION（失败时无效）
}
```

**Drop 实现：**自动关闭所有有效句柄。

**注意：**当 ThreadHandle 存在时，`r_limited_handle` 始终有效。其他句柄可能无效（使用前用 `is_invalid()` 检查）。

### CpuSetData

CPU 集 ID 到逻辑处理器映射。

```rust
pub struct CpuSetData {
    id: u32,                      // CPU 集 ID（Windows 内部）
    logical_processor_index: u8,  // 逻辑处理器号（0, 1, 2...）
}
```

## 句柄管理

### get_process_handle

使用适当的访问权限打开进程句柄。

```rust
pub fn get_process_handle(pid: u32, process_name: &str) -> Option<ProcessHandle>
```

**请求的访问权限：**
1. `PROCESS_QUERY_LIMITED_INFORMATION`（始终）
2. `PROCESS_SET_LIMITED_INFORMATION`（始终）
3. `PROCESS_QUERY_INFORMATION`（尽力）
4. `PROCESS_SET_INFORMATION`（尽力）

**错误映射：**`is_new_error()` 的内部错误代码：
- `0` - `PROCESS_QUERY_LIMITED_INFORMATION` 失败
- `1` - `PROCESS_SET_LIMITED_INFORMATION` 失败
- `2` - `PROCESS_QUERY_INFORMATION` 失败（警告）
- `3` - `PROCESS_SET_INFORMATION` 失败（警告）

**被调用者：**源中的 `apply_config()`（编排函数）

### get_thread_handle

使用适当的访问权限打开线程句柄。

```rust
pub fn get_thread_handle(tid: u32, pid: u32, process_name: &str) -> Option<ThreadHandle>
```

**参数：**
- `tid` - 要打开的线程 ID
- `pid` - 父进程 ID（用于错误日志）
- `process_name` - 进程名（用于错误日志）

**请求的访问权限：**
1. `THREAD_QUERY_LIMITED_INFORMATION`（必需）
2. `THREAD_QUERY_INFORMATION`（尽力）
3. `THREAD_SET_LIMITED_INFORMATION`（尽力）
4. `THREAD_SET_INFORMATION`（尽力）

**错误映射：**`is_new_error()` 的内部错误代码：
- `0` - `THREAD_QUERY_LIMITED_INFORMATION` 失败
- `1` - `THREAD_QUERY_INFORMATION` 失败
- `2` - `THREAD_SET_LIMITED_INFORMATION` 失败
- `3` - `THREAD_SET_INFORMATION` 失败

**返回：**如果至少可以打开 `r_limited_handle` 则返回 `Some(ThreadHandle)`，否则返回 `None`。

**被调用者：**
- [`reset_thread_ideal_processors()`](apply.md#reset_thread_ideal_processors) - 线程理想处理器操作
- [`prefetch_all_thread_cycles()`](apply.md#prefetch_all_thread_cycles) - 周期时间查询
- [`apply_prime_threads_promote()`](apply.md#apply_prime_threads_promote) - Prime 线程操作
- [`apply_prime_threads_demote()`](apply.md#apply_prime_threads_demote) - Prime 线程降级
- [`apply_ideal_processors()`](apply.md#apply_ideal_processors) - 理想处理器分配

## CPU 集操作

### get_cpu_set_information

返回全局 CPU 集信息。

```rust
pub fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>>
```

**延迟初始化：**首次调用通过 `GetSystemCpuSetInformation` 查询系统。

**缓存：**结果在进程生命周期内缓存。

### cpusetids_from_indices

将逻辑 CPU 索引转换为 CPU 集 ID。

```rust
pub fn cpusetids_from_indices(cpu_indices: &[u32]) -> Vec<u32>
```

**示例：**
```rust
cpusetids_from_indices(&[0, 1, 2])  // → [cpu_set_id_0, cpu_set_id_1, cpu_set_id_2]
```

**被调用者：**
- [`apply_process_default_cpuset()`](apply.md#apply_process_default_cpuset) - 进程 CPU 集
- [`apply_prime_threads_promote()`](apply.md#apply_prime_threads_promote) - 线程 CPU 集

### indices_from_cpusetids

将 CPU 集 ID 转换回逻辑索引。

```rust
pub fn indices_from_cpusetids(cpuids: &[u32]) -> Vec<u32>
```

**被调用者：**[apply.rs](apply.md) - 记录 CPU 集更改

### filter_indices_by_mask

按亲和性掩码过滤 CPU 索引。

```rust
pub fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> Vec<u32>
```

**目的：**确保 prime CPU 分配尊重进程亲和性。

**示例：**
```rust
filter_indices_by_mask(&[0, 1, 2, 3], 0x05)  // → [0, 2]（位 0 和 2 设置）
```

## 特权管理

### is_running_as_admin

检查当前进程是否具有提升的令牌。

```rust
pub fn is_running_as_admin() -> bool
```

**方法：**打开进程令牌并查询 `TokenElevation`。

### request_uac_elevation

以管理员特权重启进程。

```rust
pub fn request_uac_elevation(console: bool) -> io::Result<()>
```

**机制：**生成带有 `Start-Process -Verb RunAs` 的 PowerShell。

**参数：**
- `console` - 是否请求了控制台输出（用于警告消息）

**副作用：**当前进程在生成提升的子进程后退出。

**被调用者：**启动时如果未以管理员运行且未设置 `-noUAC` 的 [main.rs](main.md)

### enable_debug_privilege

为当前进程启用 `SeDebugPrivilege`。

```rust
pub fn enable_debug_privilege()
```

**目的：**允许访问受保护进程以读取线程起始地址。

**步骤：**
1. 使用 `TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY` 打开进程令牌
2. 查找 `SE_DEBUG_NAME` LUID
3. 调整令牌特权以启用

**被调用者：**启动时的 [main.rs](main.md)（除非 `-noDebugPriv`）

### enable_inc_base_priority_privilege

启用 `SeIncreaseBasePriorityPrivilege`。

```rust
pub fn enable_inc_base_priority_privilege()
```

**目的：**I/O 优先级 "high" 设置所需。

**被调用者：**启动时的 [main.rs](main.md)（除非 `-noIncBasePriority`）

## 亲和性工具

### is_affinity_unset

检查进程是否具有默认亲和性（所有系统 CPU）。

```rust
pub fn is_affinity_unset(pid: u32, process_name: &str) -> bool
```

**返回：**如果 `current_mask == system_mask` 则返回 `true`

**被调用者：**[main.rs](main.md#-find-flag) `-find` 模式以识别未配置进程

**错误处理：**每个进程名只向 `.find.log` 记录一次访问被拒绝错误。

## 线程操作

### get_thread_start_address

通过 NT API 查询线程起始地址。

```rust
pub fn get_thread_start_address(thread_handle: HANDLE) -> usize
```

**返回：**起始地址或失败时为 0

**信息类：**9 (`ThreadQuerySetWin32StartAddress`)

**被调用者：**用于模块标识的 [`prefetch_all_thread_cycles()`](apply.md#prefetch_all_thread_cycles)

### set_thread_ideal_processor_ex

设置线程的首选处理器。

```rust
pub fn set_thread_ideal_processor_ex(
    thread_handle: HANDLE,
    group: u16,
    number: u8
) -> Result<PROCESSOR_NUMBER, Error>
```

**返回：**之前的理想处理器

**被调用者：**
- [`reset_thread_ideal_processors()`](apply.md#reset_thread_ideal_processors) - 亲和性更改后
- [`apply_ideal_processors()`](apply.md#apply_ideal_processors) - 理想处理器分配

### get_thread_ideal_processor_ex

获取当前理想处理器。

```rust
pub fn get_thread_ideal_processor_ex(thread_handle: HANDLE) -> Result<PROCESSOR_NUMBER, Error>
```

**被调用者：**用于延迟设置优化的 [`apply_ideal_processors()`](apply.md#apply_ideal_processors)

## 模块解析

### resolve_address_to_module

将内存地址映射到带偏移的模块名。

```rust
pub fn resolve_address_to_module(pid: u32, address: usize) -> String
```

**返回：**
- `"module.dll+0xABC"` - 如果地址在已知模块内
- `"0x7FF12345"` - 如果不在任何模块内
- `"0x0"` - 如果地址为 0

**缓存：**模块列表在 `MODULE_CACHE` 中每进程缓存。

**被调用者：**
- [`apply_prime_threads_promote()`](apply.md#apply_prime_threads_promote) - 用于更改日志
- [`apply_prime_threads_demote()`](apply.md#apply_prime_threads_demote) - 用于更改日志
- [scheduler.rs](scheduler.md) - 用于线程跟踪报告

### drop_module_cache

清除进程的模块缓存。

```rust
pub fn drop_module_cache(pid: u32)
```

**被调用者：**
- [main.rs](main.md#main-loop) - 每次循环 prime 调度前
- [scheduler.rs](scheduler.md) - 进程退出时

## 进程管理

### terminate_child_processes

终止当前进程的子进程。

```rust
pub fn terminate_child_processes()
```

**目的：**UAC 提升后清理孤立的控制台主机进程。

**被调用者：**[main.rs](main.md) 启动时

**目标：**任何 `th32ParentProcessID == current_pid` 的进程

## 静态数据

### CPU_SET_INFORMATION

延迟初始化的全局 CPU 集数据。

```rust
static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>>
```

在首次调用 `get_cpu_set_information()` 时填充。

### MODULE_CACHE

每进程模块枚举缓存。

```rust
static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>>
```

映射：`pid → [(base_address, size, module_name), ...]`

## 依赖

- `crate::error_codes` - 错误代码转换
- `crate::log` - 日志宏
- `crate::logging` - 错误跟踪
- `once_cell` - 延迟静态初始化
- `windows` - Win32 API

## 安全说明

本模块包含大量用于 Windows API 互操作的 `unsafe` 代码。关键不变式：

1. **句柄有效性：**所有 `HANDLE` 值在使用前检查 `is_invalid()`
2. **指针安全：**模块枚举使用适当的缓冲区大小
3. **生命周期管理：**`ProcessHandle` Drop 实现确保句柄关闭
4. **线程安全：**静态缓存受 `Mutex` 保护
