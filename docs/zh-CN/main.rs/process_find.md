# process_find 函数 (main.rs)

使用 Windows Toolhelp32 快照 API 枚举所有正在运行的进程，并记录任何尚未被已加载配置覆盖、不在黑名单中、且亲和性掩码尚未被显式设置的进程。此函数实现了 `-find` CLI 模式，帮助管理员发现可能受益于亲和性或优先级调优的进程。

## 语法

```rust
fn process_find(
    cli: &CliArgs,
    configs: &ConfigResult,
    blacklist: &[String],
) -> Result<(), windows::core::Error>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cli` | `&CliArgs` | 解析后的命令行参数。函数检查 `cli.find_mode`；如果为 `false`，函数立即返回而不枚举进程。 |
| `configs` | `&ConfigResult` | 完全解析的配置结果，包含按等级和进程名称索引的 `process_level_configs` 和 `thread_level_configs`。用于确定发现的进程是否已被管理。 |
| `blacklist` | `&[String]` | 在发现过程中应被静默忽略的小写进程名称字符串切片。 |

## 返回值

成功时返回 `Ok(())`。如果 `CreateToolhelp32Snapshot` 调用失败，则返回 `Err(windows::core::Error)`。

## 备注

- 该函数仅在 `cli.find_mode` 为 `true` 时执行其主体。否则为无操作，直接返回 `Ok(())`。
- 内部函数调用 `CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)` 获取所有运行进程的快照，然后使用 `Process32FirstW` / `Process32NextW` 进行迭代。
- 每个进程名称通过 `String::from_utf16_lossy` 转换为小写，并在 `szExeFile` 缓冲区中的第一个空字符处截断。
- 仅当**以下所有条件**都满足时，进程才会被记录（通过 `log_process_find`）：
  1. 进程名称**不在**内部失败发现集合中（该集合包含之前发现失败的名称，被抑制以减少日志噪音）。
  2. 进程名称**未出现**在任何等级的 `configs.process_level_configs` 或 `configs.thread_level_configs` 中。
  3. 进程名称**不包含**在 `blacklist` 中。
  4. `is_affinity_unset(pid, name)` 返回 `true`，表示进程仍具有系统默认亲和性掩码，且未被其他工具修改。
- 迭代完成后，快照句柄通过 `CloseHandle` 关闭。
- 此函数在每次主循环迭代中被调用一次，不受等级限制，因此发现输出可能在每个轮询间隔中出现。
- 此函数中的所有 Win32 调用均在 `unsafe` 块内执行。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `main.rs` |
| 调用者 | [main](main.md)（每个轮询迭代调用一次） |
| 被调用者 | `CreateToolhelp32Snapshot`、`Process32FirstW`、`Process32NextW`、`CloseHandle`、[is_affinity_unset](../winapi.rs/is_affinity_unset.md)、[log_process_find](../logging.rs/log_process_find.md) |
| 权限 | 建议具有 `SeDebugPrivilege` 以获得完整的进程枚举可见性 |
| 平台 | 仅限 Windows（`windows` crate，`Win32::System::Diagnostics::ToolHelp`） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| main 入口点 | [main](main.md) |
| process_logs（配套发现模式） | [process_logs](process_logs.md) |
| apply_config | [apply_config](apply_config.md) |
| scheduler 模块 | [scheduler.rs README](../scheduler.rs/README.md) |
| priority 模块 | [priority.rs README](../priority.rs/README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
