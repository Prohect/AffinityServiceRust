# terminate_child_processes 函数 (winapi.rs)

终止在 UAC 提升流程中作为副作用产生的孤立控制台宿主进程，防止它们在未提升的实例退出后继续残留。

## 语法

```rust
pub fn terminate_child_processes()
```

## 参数

此函数不接受参数。

## 返回值

此函数不返回值。

## 备注

当应用程序通过 [`request_uac_elevation`](request_uac_elevation.md) 触发 UAC 提升时，会启动一个 PowerShell 进程，该进程再通过 `Start-Process -Verb RunAs` 启动应用程序的提升副本。这一进程创建链可能会留下孤立的子进程——特别是 `conhost.exe` 实例——它们在原始（未提升的）进程退出后仍继续运行。

此函数枚举当前进程的子进程并终止它们。它在 [`request_uac_elevation`](request_uac_elevation.md) 成功返回后、未提升实例退出之前由该实例调用。这确保了向提升实例的干净交接，不会留下僵尸进程。

该函数使用 Windows `CreateToolhelp32Snapshot` API 配合 `TH32CS_SNAPPROCESS` 枚举所有正在运行的进程，识别父 PID 与当前进程匹配的进程，并对每个匹配的进程调用 `TerminateProcess`。

### 调用时机

典型流程如下：

1. [`main`](../main.rs/main.md) 通过 [`is_running_as_admin`](is_running_as_admin.md) 检测到进程未以管理员身份运行。
2. 调用 [`request_uac_elevation`](request_uac_elevation.md)，启动新的提升实例。
3. 调用 `terminate_child_processes` 清理已产生的辅助进程。
4. 未提升的实例退出。

### 安全性

该函数仅终止父 PID 与当前进程匹配的进程，因此不会影响无关进程。如果快照或终止调用失败，错误将被静默忽略，因为未提升的进程即将退出。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L715–L765 |
| **调用方** | [`main`](../main.rs/main.md) |
| **Windows API** | [CreateToolhelp32Snapshot](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot)、[Process32First](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32first)、[Process32Next](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32next)、[TerminateProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess) |

## 另请参阅

- [request_uac_elevation](request_uac_elevation.md)
- [is_running_as_admin](is_running_as_admin.md)
- [winapi.rs 模块概述](README.md)