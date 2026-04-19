# get_thread_start_address 函数 (winapi.rs)

通过 `NtQueryInformationThread` 和 `ThreadQuerySetWin32StartAddress` 信息类（类 9）获取线程的 Win32 起始地址。起始地址标识线程被创建时执行的函数，从而实现基于模块的理想处理器分配和诊断报告。

## 语法

```rust
pub fn get_thread_start_address(thread_handle: HANDLE) -> usize
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `thread_handle` | `HANDLE` | 有效的线程句柄，至少具有 `THREAD_QUERY_INFORMATION` 访问权限。通常是 [ThreadHandle](ThreadHandle.md) 的 `r_handle` 字段。使用 `r_limited_handle`（`THREAD_QUERY_LIMITED_INFORMATION`）**不足以**进行此查询 — 使用类 9 的 `NtQueryInformationThread` 需要完全查询权限。 |

## 返回值

`usize` — 线程入口点在进程地址空间中的虚拟地址。如果查询失败（例如句柄没有足够的访问权限或线程已退出），返回 `0`。

## 说明

### NT API 详情

函数调用未公开的 `NtQueryInformationThread` FFI 绑定（从 `ntdll.dll` 链接），参数如下：

| 参数 | 值 |
|-----------|-------|
| `thread_handle` | 从调用方传入 |
| `thread_information_class` | `9`（`ThreadQuerySetWin32StartAddress`） |
| `thread_information` | `usize` 输出缓冲区的指针 |
| `thread_information_length` | `size_of::<usize>()`（64 位上为 8 字节） |
| `return_length` | `u32` 的指针（调用后未使用） |

函数检查返回值上的 `NTSTATUS.is_ok()`。如果状态表示成功，则返回起始地址；否则返回 `0`。

### 起始地址与入口点

此信息类返回的 Win32 起始地址是传递给 `CreateThread` / `_beginthreadex`（或等效函数）的函数的地址。这可能不同于内核看到的实际线程入口点（`RtlUserThreadStart`），后者在执行 CRT 初始化后跳转到用户指定的启动函数。

### 在服务中的用途

起始地址在两个关键场景中用作：

1. **基于模块的理想处理器分配** — 地址被传递给 [resolve_address_to_module](resolve_address_to_module.md) 以确定线程属于哪个模块（DLL/EXE）。配置中的模块前缀匹配规则随后为匹配模块中的线程分配特定的理想处理器。

2. **诊断报告** — [ThreadStats](../scheduler.rs/ThreadStats.md) 的 `Debug` 实现和 [PrimeThreadScheduler::drop_process_by_pid](../scheduler.rs/PrimeThreadScheduler.md) 中的进程退出报告都将起始地址解析为 `"module.dll+0xOffset"` 字符串，用于人类可读的线程标识。

### 失败情况

在以下情况下，函数返回 `0`：

- 线程句柄没有 `THREAD_QUERY_INFORMATION` 访问权限（仅获得了受限访问）。
- 线程已经退出，句柄已过期。
- NT API 调用返回意外的 `NTSTATUS` 错误。

下游消费者将返回值 `0` 视为"未知"。对于零地址，[resolve_address_to_module](resolve_address_to_module.md) 返回字符串 `"0x0"`。

### 句柄要求

必须使用 [ThreadHandle](ThreadHandle.md) 的 `r_handle` 字段（以 `THREAD_QUERY_INFORMATION` 打开），而不是 `r_limited_handle`。如果 `r_handle` 无效（在线程句柄获取期间访问被拒绝），此函数将返回 `0`。调用方应在此函数调用前检查 `r_handle.is_invalid()`，以避免浪费系统调用。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用方** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md)、[prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md)、[ThreadStats](../scheduler.rs/ThreadStats.md) |
| **被调用方** | `NtQueryInformationThread` (ntdll FFI) |
| **NT API** | `NtQueryInformationThread` 带 `ThreadQuerySetWin32StartAddress`（类 9） |
| **特权** | 需要线程句柄上的 `THREAD_QUERY_INFORMATION` 访问权限；[SeDebugPrivilege](enable_debug_privilege.md) 可能需要获取受保护进程的此句柄 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [winapi.rs](README.md) |
| 地址到模块解析 | [resolve_address_to_module](resolve_address_to_module.md) |
| 线程句柄管理 | [ThreadHandle](ThreadHandle.md)、[get_thread_handle](get_thread_handle.md) |
| 存储起始地址的线程统计 | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| 理想处理器分配 | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |

*文档版本：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*