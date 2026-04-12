# request_uac_elevation 函数 (winapi.rs)

通过 PowerShell 的 `Start-Process -Verb RunAs` 启动一个新的提升（管理员）应用程序实例，触发 Windows UAC 同意对话框。

## 语法

```rust
pub fn request_uac_elevation(console: bool) -> io::Result<()>
```

## 参数

`console`

当为 `true` 时，提升的进程将以可见的控制台窗口启动。当为 `false` 时，提升的进程可能在不带控制台的情况下启动，具体取决于 PowerShell 的调用标志。

## 返回值

如果提升的进程成功启动，则返回 `Ok(())`。如果 PowerShell 无法启动或 `Start-Process` 命令失败（例如用户拒绝了 UAC 提示），则返回 `io::Error`。

## 备注

此函数是应用程序在检测到未以管理员权限运行时进行自我提升的机制。流程如下：

1. 函数构造当前可执行文件路径及其命令行参数。
2. 调用 PowerShell 的 `Start-Process -Verb RunAs` 以提升权限重新启动应用程序。
3. 如果用户接受 UAC 同意对话框，新的提升进程启动，当前（未提升的）进程预期随后退出。
4. 如果用户拒绝 UAC 对话框，PowerShell 报告错误，函数返回 `io::Error`。

成功提升后，原始（未提升的）进程应调用 [`terminate_child_processes`](terminate_child_processes.md) 来清理在 PowerShell 调用过程中产生的孤立控制台宿主进程。

`console` 参数控制重新启动的进程是否获得可见的控制台窗口，这与交互式使用和后台/服务运行的场景相关。

**安全说明：** 仅当 [`is_running_as_admin`](is_running_as_admin.md) 返回 `false` 且未设置 `--no-uac` CLI 标志时才请求 UAC 提升。[`CliArgs`](../cli.rs/CliArgs.md) 中的 `--no-uac` 标志允许用户在不希望出现 UAC 提示的环境中禁止提升。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L468–L501 |
| **调用方** | [`main`](../main.rs/main.md) |
| **调用** | PowerShell `Start-Process -Verb RunAs` |
| **Windows API** | [ShellExecuteW](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecutew)（通过 PowerShell） |

## 另请参阅

- [is_running_as_admin](is_running_as_admin.md)
- [terminate_child_processes](terminate_child_processes.md)
- [CliArgs](../cli.rs/CliArgs.md)（`no_uac` 标志）
- [winapi.rs 模块概述](README.md)