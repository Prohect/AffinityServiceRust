# request_uac_elevation 函数 (winapi.rs)

通过 PowerShell 的 `Start-Process -Verb RunAs` 启动一个提升权限的进程副本，触发 Windows UAC（用户帐户控制）提示，以管理员权限重新启动当前进程。生成提升权限的子进程后，当前进程随即退出。

## 语法

```rust
pub fn request_uac_elevation(console: bool) -> io::Result<()>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `console` | `bool` | 指示进程是否在控制台模式下运行。当为 `true` 时，会记录一条警告，说明提升权限后的输出将不会显示在当前控制台会话中。 |

## 返回值

返回 `io::Result<()>`。成功时，此函数**不会返回**——它在生成提升权限的子进程后调用 `std::process::exit(0)`。失败时（例如无法启动 PowerShell），返回一个描述生成失败原因的 `io::Error`。

## 备注

### 提升权限机制

该函数构造如下形式的 PowerShell 命令：

```text
powershell.exe -Command "Start-Process -FilePath '<exe_path>' -Verb RunAs -ArgumentList '<args>'"
```

其中 `<exe_path>` 是当前运行的可执行文件路径（通过 `std::env::current_exe()` 获取），`<args>` 是从当前调用转发的所有命令行参数。

### 跳过日志标志

在生成提升权限的子进程之前，该函数会将 `-skip_log_before_elevation` 追加到参数列表中。此标志防止提升权限的实例重复输出非提升权限实例已经写入的启动日志消息。

### 控制台模式警告

当 `console` 为 `true` 时，该函数会记录一条警告，说明提升权限的进程将在新窗口/会话中运行，因此后续日志输出将不会在原始控制台中可见。这是 UAC 提升的固有限制——新进程会获得一个新的控制台宿主。

### 进程生命周期

1. 该函数记录 `"Requesting UAC elevation..."`。
2. 使用构造的命令将 `powershell.exe` 作为子进程生成。
3. 成功生成后，记录确认消息并调用 `exit(0)`。
4. 生成失败时，记录错误并返回 `io::Error`。

### 边界情况

- 如果 `std::env::current_exe()` 失败（例如可执行文件在运行时被删除），该函数在尝试生成 PowerShell 之前返回相应的 `io::Error`。
- 如果用户拒绝了 UAC 提示，PowerShell 的 `Start-Process` 命令会静默失败，而原始进程（已经退出）不会感知到。
- 该函数**不会**等待提升权限的子进程启动或确认成功后才退出。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `winapi.rs` |
| **调用方** | `main.rs` — 需要管理员权限但当前未持有时的启动逻辑 |
| **被调用方** | `std::env::current_exe`、`std::env::args`、`std::process::Command::spawn`、`std::process::exit`、[`log_message`](../logging.rs/log_message.md)（通过 `log!` 宏） |
| **外部依赖** | `powershell.exe` 必须在系统 PATH 中可用 |
| **平台** | 仅限 Windows |
| **权限** | 调用本身不需要特殊权限；会触发 UAC 提示，由用户授予新进程管理员权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| is_running_as_admin | [is_running_as_admin](is_running_as_admin.md) |
| enable_debug_privilege | [enable_debug_privilege](enable_debug_privilege.md) |
| enable_inc_base_priority_privilege | [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) |
| terminate_child_processes | [terminate_child_processes](terminate_child_processes.md) |
| logging 模块 | [logging.rs](../logging.rs/README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
