# request_uac_elevation 函数 (winapi.rs)

通过 PowerShell `Start-Process -Verb RunAs` 命令以管理员特权重新启动当前进程，请求用户账户控制（UAC）提权。如果提权请求成功发出，当前（未提权的）进程将立即退出。此函数在成功时不会返回。

## 语法

```rust
pub fn request_uac_elevation(console: bool) -> io::Result<()>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `console` | `bool` | 指示当前进程是否在可见的控制台窗口中运行。当为 `true` 时，会额外记录一条警告，提醒用户提权后的日志输出不会出现在当前控制台会话中，因为提权后的进程运行在新窗口中。 |

## 返回值

| 值 | 描述 |
|----|------|
| `Ok(())` | 实际上不可达——函数在成功生成提权子进程后会调用 `std::process::exit(0)`。 |
| `Err(io::Error)` | `powershell.exe` 子进程无法生成（例如 `powershell.exe` 不在 `PATH` 中，或系统严重资源不足）。错误在返回前也会被记录。 |

## 备注

### 提权机制

该函数构造如下形式的 PowerShell 命令行：

```
powershell.exe -Command "Start-Process -FilePath '<exe_path>' -Verb RunAs -ArgumentList '<args>'"
```

其中：

- `<exe_path>` 是当前可执行文件的完整路径，通过 `std::env::current_exe()` 获取。
- `<args>` 是原始命令行参数（去除 `argv[0]`），并追加了 `-skip_log_before_elevation`。

`-Verb RunAs` 会触发 Windows UAC 同意对话框。如果用户同意，Windows 将以完整管理员令牌启动新进程。如果用户拒绝提权，PowerShell 命令将静默失败（它不会将错误传回当前进程）。

### `-skip_log_before_elevation` 标志

在提权后的子进程启动主循环之前，通常会输出启动日志消息。`-skip_log_before_elevation` 标志被追加到参数列表中，用于通知 [CLI 解析器](../cli.rs/README.md) 这是一次重新启动，某些提权前的日志条目（如"正在请求提权"消息）不应在日志文件中重复。

### 进程退出行为

成功时，函数调用 `std::process::exit(0)` 来终止未提权的父进程。这意味着：

- 在调用时存活的对象的 **`Drop` 实现不会运行**。
- 任何尚未刷新的缓冲日志输出可能会丢失。
- 调用者应确保在调用此函数之前持久化关键状态。

### 控制台警告

当 `console` 为 `true` 且进程未以管理员身份运行且未设置 `noUAC` 标志时，会记录一条警告：

> "Warning: process is running as non-administrator without 'noUAC' flag with 'console' flag, the log after elevation will not be shown in current session."

这是因为提权后的进程在新的控制台窗口中生成，用户需要切换到该窗口才能看到后续输出。

### 典型调用流程

在主循环（[`main.rs`](../main.rs/README.md)）中：

1. [is_running_as_admin](is_running_as_admin.md) 返回 `false`。
2. `noUAC` CLI 标志**未**设置。
3. 调用 `request_uac_elevation`。
4. 当前进程退出；提权后的子进程接管。

### 错误情况

| 场景 | 行为 |
|------|------|
| `std::env::current_exe()` 失败 | 从 `current_exe()` 返回 `Err(io::Error)`。 |
| `powershell.exe` 无法找到或生成 | 从 `Command::spawn()` 返回 `Err(io::Error)`。 |
| 用户拒绝 UAC 提示 | `Start-Process` 命令在 PowerShell 内部失败，但 `Command::spawn()` 已经成功——当前进程已经退出。提权后的子进程不会启动。 |

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **可见性** | `pub` |
| **调用者** | [`main`](../main.rs/README.md)（启动流程） |
| **被调用者** | `std::env::current_exe`、`std::env::args`、`std::process::Command::spawn`、`std::process::exit` |
| **外部程序** | `powershell.exe`（必须在 `PATH` 中） |
| **特权** | 调用无需特权；该函数通过 UAC **请求**提权 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 管理员特权检查 | [is_running_as_admin](is_running_as_admin.md) |
| 调试特权启用（提权后） | [enable_debug_privilege](enable_debug_privilege.md) |
| 基础优先级特权启用（提权后） | [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) |
| CLI 参数解析和标志 | [cli 模块](../cli.rs/README.md) |
| 服务主入口点 | [main 模块](../main.rs/README.md) |