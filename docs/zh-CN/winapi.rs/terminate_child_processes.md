# terminate_child_processes 函数 (winapi.rs)

通过获取系统级进程快照、识别父 PID 与当前进程匹配的条目，并对每个子进程调用 `TerminateProcess` 来终止当前进程的所有子进程。此函数在启动时用于清理孤立的子进程——特别是由 [request_uac_elevation](request_uac_elevation.md) 生成的提权 PowerShell 实例——这些进程可能仍在上一次启动后运行。

## 语法

```rust
pub fn terminate_child_processes()
```

## 参数

无。

## 返回值

无。该函数对每个尝试终止的子进程记录成功或失败信息，并在所有情况下返回。它不会将错误传播给调用者。

## 备注

### 算法

1. 通过 `GetCurrentProcessId` 获取当前进程 ID。
2. 通过 `CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)` 创建系统级进程快照。如果无法创建快照，函数立即返回且不记录日志。
3. 使用 `Process32FirstW` / `Process32NextW` 遍历快照。
4. 对于每个 `th32ParentProcessID` 等于当前进程 PID 的进程条目：
   - 从 `szExeFile`（以空字符结尾的 UTF-16）中提取子进程的映像名称。
   - 以 `PROCESS_TERMINATE` 访问权限通过 `OpenProcess` 打开子进程。
   - 调用 `TerminateProcess(handle, 0)` 强制终止子进程，退出代码为 0。
   - 通过 `CloseHandle` 关闭子进程句柄。
5. 迭代完成后关闭快照句柄。

### 日志记录

终止过程的每个步骤都会产生一条日志消息：

| 结果 | 日志消息 |
|------|----------|
| 子进程成功终止 | `"terminate_child_processes: terminated '<name>' (PID <pid>)"` |
| `TerminateProcess` 失败 | `"terminate_child_processes: failed to terminate '<name>' (PID <pid>)"` |
| `OpenProcess` 失败 | `"terminate_child_processes: failed to open '<name>' (PID <pid>)"` |

### 为何需要清理子进程

当 AffinityServiceRust 通过 [request_uac_elevation](request_uac_elevation.md) 请求 UAC 提权时，它会生成一个 `powershell.exe` 子进程，该子进程随后启动提权实例。未提权的父进程通过 `std::process::exit(0)` 立即退出，但 PowerShell 子进程可能仍在运行。当提权实例启动时，它调用 `terminate_child_processes` 来清理此类孤立的子进程。

此外，在某些 Windows 配置中，控制台宿主进程（`conhost.exe`）可能作为子进程保持连接，也会被此函数清理。

### 父 PID 注意事项

Windows 不维护严格的父子进程树。`PROCESSENTRY32W` 中的 `th32ParentProcessID` 字段记录的是创建该条目的进程的 PID，但需注意：

- 如果父进程已退出，父 PID 可能已被操作系统**回收**并分配给不相关的进程。在这种情况下，此函数会错误地将不相关的进程识别为子进程。
- 在实践中这种风险得到了缓解，因为该函数在启动时立即调用，此时当前 PID 还没有足够的时间被回收和重用。

### 快照安全

在首次调用 `Process32FirstW` 之前，`PROCESSENTRY32W` 结构体的 `dwSize` 字段必须设置为 `size_of::<PROCESSENTRY32W>()`。该函数正确初始化了此字段。快照句柄在最终清理步骤中通过 `CloseHandle` 关闭，涵盖了所有代码路径。

### 不安全代码

整个迭代主体包装在 `unsafe` 块中，因为它调用了 Win32 FFI 函数（`Process32FirstW`、`Process32NextW`、`OpenProcess`、`TerminateProcess`、`CloseHandle`）。安全不变量通过以下方式维持：

- 仅在 `Process32FirstW` / `Process32NextW` 调用成功后才从快照结构中读取数据。
- 仅在 `OpenProcess` 返回 `Ok` 时才解引用进程句柄。
- 在返回前关闭所有句柄。

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **可见性** | `pub` |
| **调用者** | [`main`](../main.rs/README.md)（启动序列，UAC 提权之后） |
| **被调用者** | `GetCurrentProcessId`、`CreateToolhelp32Snapshot`、`Process32FirstW`、`Process32NextW`、`OpenProcess`、`TerminateProcess`、`CloseHandle`（Win32 ToolHelp / Threading） |
| **API** | [`CreateToolhelp32Snapshot`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot)、[`TerminateProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess) |
| **特权** | 对每个子进程需要 `PROCESS_TERMINATE` 访问权限；对受保护的子进程可能需要 `SeDebugPrivilege` |

## 另请参阅

| 主题 | 链接 |
|------|------|
| UAC 提权（生成由此处清理的子进程） | [request_uac_elevation](request_uac_elevation.md) |
| 管理员特权检查 | [is_running_as_admin](is_running_as_admin.md) |
| 进程句柄 RAII 包装器 | [ProcessHandle](ProcessHandle.md) |
| 服务主入口点 | [main 模块](../main.rs/README.md) |
| CreateToolhelp32Snapshot (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot) |
| TerminateProcess (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd