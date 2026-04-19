# terminate_child_processes 函数 (winapi.rs)

终止当前进程派生的所有子进程。在服务启动期间调用，以清理孤立的子进程——特别是 [request_uac_elevation](request_uac_elevation.md) 中 UAC 提升流程留下的 PowerShell 控制台主机进程。

## 语法

```rust
pub fn terminate_child_processes()
```

## 参数

此函数不接受参数。它操作当前进程（通过 `GetCurrentProcessId` 识别）。

## 返回值

此函数不返回值。在快照创建、进程枚举或终止期间的失败会被记录，但不会作为错误传播。

## 说明

该函数使用 Windows 工具帮助库枚举系统上的所有进程，并识别那些 `th32ParentProcessID` 与当前进程 PID 匹配的子进程。对于找到的每个子进程，它以 `PROCESS_TERMINATE` 访问权限打开进程，并使用退出代码 `0` 调用 `TerminateProcess`。

### 算法

1. **GetCurrentProcessId** — 获取当前（服务）进程的 PID。
2. **CreateToolhelp32Snapshot** — 创建所有运行进程快照（`TH32CS_SNAPPROCESS`）。如果快照失败，函数静默返回。
3. **Process32FirstW / Process32NextW** — 遍历快照中的每个进程。
4. 对于每个 `th32ParentProcessID == current_pid` 的进程条目：
   - 从 null 终止的 `szExeFile` 字段中提取子进程名称。
   - 通过 `OpenProcess` 以 `PROCESS_TERMINATE` 访问权限打开子进程。
   - 使用退出代码 `0` 调用 `TerminateProcess`。
   - 通过 `CloseHandle` 关闭进程句柄。
   - 记录每一步的成功或失败。
5. **CloseHandle** — 迭代完成后关闭快照句柄。

### 日志记录

每个终止尝试都会产生日志消息：

| 结果 | 日志消息格式 |
|---------|-------------------|
| 成功 | `"terminate_child_processes: terminated '{name}' (PID {pid})"` |
| TerminateProcess 失败 | `"terminate_child_processes: failed to terminate '{name}' (PID {pid})"` |
| OpenProcess 失败 | `"terminate_child_processes: failed to open '{name}' (PID {pid})"` |

### UAC 提升清理

当服务在没有管理员权限的情况下启动时，[request_uac_elevation](request_uac_elevation.md) 通过 `powershell.exe Start-Process -Verb RunAs` 启动新的提升实例，然后退出。PowerShell 子进程可能会作为孤儿持续存在。在下一次启动（现在是提升的）时，`terminate_child_processes` 在主轮询循环开始之前清理任何此类遗留的子进程。

### 安全注意事项

- 该函数不加区分地终止**所有**子进程。它不按进程名称或用途进行过滤。
- `TerminateProcess` 是立即的、非优雅终止 — 子进程不接收关闭通知或运行清理处理程序。
- 如果不存在子进程，函数静默遍历快照并返回，不执行任何操作。
- 即使迭代中途失败，快照句柄也总是在函数返回前关闭。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用方** | `src/main.rs` 中的主启动逻辑 |
| **被调用方** | 无（直接使用 Win32 API 的叶函数） |
| **Win32 API** | [GetCurrentProcessId](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocessid), [CreateToolhelp32Snapshot](https://learn.microsoft.com/zh-cn/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot), [Process32FirstW](https://learn.microsoft.com/zh-cn/windows/win32/api/tlhelp32/nf-tlhelp32-process32firstw), [Process32NextW](https://learn.microsoft.com/zh-cn/windows/win32/api/tlhelp32/nf-tlhelp32-process32nextw), [OpenProcess](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess) (`PROCESS_TERMINATE`), [TerminateProcess](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess), [CloseHandle](https://learn.microsoft.com/zh-cn/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **特权** | 子进程的 `PROCESS_TERMINATE` 访问权限；某些子进程可能需要提升权限 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [winapi.rs](README.md) |
| 创建子进程的 UAC 提升 | [request_uac_elevation](request_uac_elevation.md) |
| 触发提升的管理员检查 | [is_running_as_admin](is_running_as_admin.md) |
| 模块枚举（使用类似的快照模式） | [enumerate_process_modules](enumerate_process_modules.md) |

*文档版本：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*