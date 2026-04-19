# request_uac_elevation 函数 (winapi.rs)

通过启动 PowerShell `Start-Process -Verb RunAs` 命令来以管理员权限重新启动当前进程，该命令会触发 Windows UAC 同意对话框。提升的子进程启动后，当前（未提升）进程将退出。

## 语法

```rust
pub fn request_uac_elevation(console: bool) -> io::Result<()>
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `console` | `bool` | 服务是否使用 `console` CLI 标志启动。当为 `true` 时，会记录警告，说明提升的进程输出将出现在新的控制台窗口中，而不是当前窗口中。 |

## 返回值

`io::Result<()>` — 成功时，此函数**不会返回**，因为当前进程在提升子进程启动后调用 `std::process::exit(0)`。失败时（例如无法启动 PowerShell），返回描述启动失败的 `io::Error`。

## 说明

### 提升机制

函数构造如下形式的 PowerShell 命令：

```
Start-Process -FilePath '<current_exe_path>' -Verb RunAs -ArgumentList '<original_args> -skip_log_before_elevation'
```

- 当前可执行文件路径通过 `std::env::current_exe()` 获取。
- 所有原始命令行参数（跳过 `argv[0]`）都转发到提升的实例。
- 附加标志 `-skip_log_before_elevation` 以防止提升的进程重复未提升实例中已经发生的日志初始化。

### 控制台模式警告

当 `console` 为 `true` 时，函数记录一条警告，解释提升的进程将在单独的控制台窗口中运行。这是 `runas` 命令的固有特性 — Windows 为提升的进程创建一个新的控制台，当前控制台会话将不再接收服务的任何输出。

### 进程退出

成功调用 `Command::spawn()` 后，当前进程立即调用 `exit(0)`。这确保只有一个服务实例在运行（新启动的提升实例）。退出是无条件的 — 调用方中此函数调用后的清理代码不会执行。

### 错误处理

如果 `Command::spawn()` 失败（例如找不到 PowerShell，或用户在进程启动前拒绝了 UAC 提示），错误将被记录并作为 `io::Result<Err>` 传播。调用方可以决定是继续在没有提升的情况下运行还是中止。

### UAC 提示行为

实际的 UAC 同意对话框由 Windows 在提升的 PowerShell 实例执行 `Start-Process -Verb RunAs` 命令时显示。如果用户点击**否**或对话框超时，`Start-Process` cmdlet 从 spawning（未提升）进程的角度静默失败 — `spawn()` 调用本身成功，因为它只启动 PowerShell，而不是提升的目标。当前进程此时已经退出。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用方** | `src/main.rs` 中的主启动逻辑，受 `!is_running_as_admin() && !cli.no_uac` 保护 |
| **被调用方** | `std::env::current_exe`、`std::env::args`、`std::process::Command::spawn`、`std::process::exit` |
| **Win32 API** | 无直接调用；依赖 PowerShell 的 `Start-Process -Verb RunAs`，其内部调用 [ShellExecuteW](https://learn.microsoft.com/zh-cn/windows/win32/api/shellapi/nf-shellapi-shellexecutew) |
| **特权** | 无需调用特权；UAC 对话框授予子进程的提升权限 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [winapi.rs](README.md) |
| 控制此调用的管理员检查 | [is_running_as_admin](is_running_as_admin.md) |
| 提升后启用的调试权限 | [enable_debug_privilege](enable_debug_privilege.md) |
| 提升后启用的基础优先级权限 | [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) |
| 启动时的子进程清理 | [terminate_child_processes](terminate_child_processes.md) |

*文档版本：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*