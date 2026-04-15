# terminate_child_processes 函数 (winapi.rs)

终止当前进程派生的所有子进程。此函数在启动期间被调用，用于清理孤立的子进程，特别是通过 [request_uac_elevation](request_uac_elevation.md) 进行 UAC 提升后可能残留的提升 PowerShell 控制台宿主实例。

## 语法

```rust
pub fn terminate_child_processes()
```

## 参数

此函数不接受参数。

## 返回值

此函数不返回值。每个子进程的成功或失败状态通过 [`log_message`](../logging.rs/log_message.md)（经由 `log!` 宏）报告。

## 备注

### 算法

1. 通过 `GetCurrentProcessId` 获取当前进程 ID。
2. 通过 `CreateToolhelp32Snapshot` 使用 `TH32CS_SNAPPROCESS` 创建系统中所有进程的快照。
3. 使用 `Process32FirstW` / `Process32NextW` 遍历快照。
4. 对于每个 `th32ParentProcessID` 与当前进程 ID 匹配的进程条目：
   - 从 `szExeFile`（以 null 结尾的 UTF-16 数组）中提取子进程名称。
   - 尝试以 `PROCESS_TERMINATE` 访问权限通过 `OpenProcess` 打开子进程。
   - 调用 `TerminateProcess` 并以退出代码 `0` 强制终止子进程。
   - 通过 `CloseHandle` 关闭子进程句柄。
5. 关闭快照句柄。

### 日志输出

| 条件 | 日志消息 |
|------|----------|
| 子进程成功终止 | `terminate_child_processes: terminated '<name>' (PID <pid>)` |
| `TerminateProcess` 失败 | `terminate_child_processes: failed to terminate '<name>' (PID <pid>)` |
| `OpenProcess` 失败 | `terminate_child_processes: failed to open '<name>' (PID <pid>)` |

### 重要副作用

- 此函数**强制终止**子进程，不会给它们执行清理操作的机会。它使用退出代码 `0` 调用 `TerminateProcess`，该操作不会调用目标进程中的 DLL 分离例程或刷新 I/O 缓冲区。
- 此函数终止当前进程的**所有**直接子进程，而不仅仅是特定的子进程。任何 `th32ParentProcessID` 与当前 PID 匹配的进程都将成为目标。
- 如果 `CreateToolhelp32Snapshot` 失败，函数将静默返回，不会记录日志。

### 为什么需要此函数

当 AffinityServiceRust 通过 [request_uac_elevation](request_uac_elevation.md) 请求 UAC 提升时，它会使用 `Start-Process -Verb RunAs` 派生一个 `powershell.exe` 子进程。原始（未提升的）进程随后调用 `exit(0)`，但与 PowerShell 命令关联的控制台宿主进程（`conhost.exe`）可能会变成孤立进程。在提升后的实例启动时调用 `terminate_child_processes` 可以清理这些孤立进程。

### 平台说明

- **仅限 Windows。** 使用工具帮助库（`CreateToolhelp32Snapshot`、`Process32FirstW`、`Process32NextW`）和进程管理 API（`OpenProcess`、`TerminateProcess`、`CloseHandle`）。
- `PROCESSENTRY32W` 结构体使用 260 个宽字符（`MAX_PATH`）的固定大小 `szExeFile` 数组。进程名称通过查找此数组中的第一个 null 终止符来提取。
- 快照代表某一时间点的视图。在快照和终止尝试之间启动或退出的进程可能导致 `OpenProcess` 或 `TerminateProcess` 失败，这些情况通过日志记录进行优雅处理。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `winapi.rs` |
| **调用者** | `main.rs` — 启动清理 |
| **被调用者** | `GetCurrentProcessId`、`CreateToolhelp32Snapshot`、`Process32FirstW`、`Process32NextW`、`OpenProcess`、`TerminateProcess`、`CloseHandle`（Win32 API）；`log!` 宏 |
| **Win32 API** | [CreateToolhelp32Snapshot](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot)、[TerminateProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess) |
| **权限** | 需要对子进程具有 `PROCESS_TERMINATE` 访问权限。以管理员身份运行并启用 `SeDebugPrivilege` 可确保对所有子进程的操作成功。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| request_uac_elevation | [request_uac_elevation](request_uac_elevation.md) |
| is_running_as_admin | [is_running_as_admin](is_running_as_admin.md) |
| enable_debug_privilege | [enable_debug_privilege](enable_debug_privilege.md) |
| enumerate_process_modules | [enumerate_process_modules](enumerate_process_modules.md) |
| logging 模块 | [logging.rs](../logging.rs/README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
