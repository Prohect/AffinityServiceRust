# error_from_ntstatus 函数 (error_codes.rs)

将 NTSTATUS 代码映射为人类可读的符号名称字符串。NTSTATUS 值由 NT 原生 API 函数（如 `NtSetInformationProcess` 和 `NtQueryInformationProcess`）返回，AffinityServiceRust 使用这些函数进行 I/O 优先级管理。此函数将最常遇到的状态代码翻译为其众所周知的常量名称，用于诊断日志记录。

## 语法

```error_codes.rs
pub fn error_from_ntstatus(status: i32) -> String
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `status` | `i32` | NT 原生 API 调用返回的 NTSTATUS 代码。NTSTATUS 值是有符号 32 位整数，其中负值（高位置位）表示错误，零表示成功，正值表示信息性或警告状态。 |

## 返回值

包含给定 NTSTATUS 代码符号名称的 `String`。如果代码无法识别，则返回格式为 `"NTSTATUS_0x{code:08X}"` 的十六进制回退字符串。

## 备注

该函数通过 `i32::cast_unsigned` 将状态代码转换为无符号表示，然后使用 `match` 表达式进行匹配，以处理 NTSTATUS 错误代码（如 `0xC0000022`）自然以无符号十六进制字面量表达的常见模式。

可识别以下 NTSTATUS 代码：

| 代码 | 符号名称 | 描述 |
|------|----------|------|
| `0x00000000` | `STATUS_SUCCESS` | 操作成功完成。 |
| `0x00000001` | `STATUS_WAIT_1` | 调用方指定的等待在对象索引 1 上完成。 |
| `0xC0000001` | `STATUS_UNSUCCESSFUL` | 不成功的通用状态。 |
| `0xC0000002` | `STATUS_NOT_IMPLEMENTED` | 请求的操作未实现。 |
| `0xC0000003` | `STATUS_INVALID_INFO_CLASS` | 为该操作指定的信息类无效。 |
| `0xC0000004` | `STATUS_INFO_LENGTH_MISMATCH` | 提供的缓冲区长度对于该信息类不正确。 |
| `0xC0000008` | `STATUS_INVALID_HANDLE` | 指定了无效的 HANDLE。 |
| `0xC000000D` | `STATUS_INVALID_PARAMETER` | 向服务或函数传递了无效的参数。 |
| `0xC0000017` | `STATUS_NO_MEMORY` | 虚拟内存或页面文件配额不足。 |
| `0xC0000018` | `STATUS_CONFLICTING_ADDRESSES` | 指定的地址范围与现有分配冲突。 |
| `0xC0000022` | `STATUS_ACCESS_DENIED` | 调用方没有所需的访问权限。 |
| `0xC0000023` | `STATUS_BUFFER_TOO_SMALL` | 提供的缓冲区太小，无法接收请求的数据。 |
| `0xC0000034` | `STATUS_OBJECT_NAME_NOT_FOUND` | 命名对象不存在。 |
| `0xC000004B` | `STATUS_THREAD_IS_TERMINATING` | 目标线程正在终止过程中。 |
| `0xC0000061` | `STATUS_PRIVILEGE_NOT_HELD` | 调用方未持有所需的特权。 |
| `0xC00000BB` | `STATUS_NOT_SUPPORTED` | 不支持该请求。 |
| `0xC000010A` | `STATUS_PROCESS_IS_TERMINATING` | 目标进程正在终止过程中。 |

### AffinityServiceRust 中的常见场景

- **`STATUS_ACCESS_DENIED` (0xC0000022)：** 在没有足够特权的情况下尝试对受保护进程设置 I/O 优先级时返回。
- **`STATUS_PROCESS_IS_TERMINATING` (0xC000010A)：** 目标进程在句柄获取和 API 调用之间退出时返回——这是轮询循环中一种良性的竞态条件。
- **`STATUS_THREAD_IS_TERMINATING` (0xC000004B)：** 线程级别的类似竞态条件。
- **`STATUS_PRIVILEGE_NOT_HELD` (0xC0000061)：** 在没有 `SeIncreaseBasePriorityPrivilege` 的情况下尝试设置 `High` I/O 优先级时返回。

### 与 Win32 错误代码的区别

NTSTATUS 代码使用与 Win32 错误代码不同的编号方案。虽然两者可以表示相同的概念性错误（例如访问被拒绝），但它们在数值上是不同的，不能互换使用。对于 Win32 `GetLastError` 风格的代码，请使用 [error_from_code_win32](error_from_code_win32.md)；对于来自 NT 原生 API 调用的 NTSTATUS 值，请使用本函数。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `error_codes` |
| 调用方 | [apply_io_priority](../apply.rs/apply_io_priority.md)、[apply_memory_priority](../apply.rs/apply_memory_priority.md) |
| 被调用方 | *（无——纯数据映射）* |
| NT API | `NtSetInformationProcess`、`NtQueryInformationProcess`（调用方产生此处翻译的状态代码） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| Win32 错误代码翻译 | [error_from_code_win32](error_from_code_win32.md) |
| I/O 优先级枚举（主要用例） | [IOPriority](../priority.rs/IOPriority.md) |
| 内存优先级枚举 | [MemoryPriority](../priority.rs/MemoryPriority.md) |
| 日志记录和错误去重 | [logging 模块](../logging.rs/README.md) |
| 模块概述 | [error_codes 模块](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd