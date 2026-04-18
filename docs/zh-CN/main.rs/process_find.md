# process_find 函数 (main.rs)

使用 Windows Toolhelp32 快照 API 枚举所有正在运行的进程，并记录任何尚未被已加载配置覆盖、未出现在黑名单中、且亲和性掩码未被显式设置的进程。此函数实现了 `-find` CLI 模式，帮助管理员发现可能需要进行亲和性或优先级调优的进程。

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
| `cli` | `&CliArgs` | 已解析的命令行参数。函数会检查 `cli.find_mode`；如果为 `false`，函数会立即返回而不枚举进程。 |
| `configs` | `&ConfigResult` | 完整解析的配置结果，包含按等级和进程名称索引的 `process_level_configs` 和 `thread_level_configs`。用于判断已发现的进程是否已被管理。 |
| `blacklist` | `&[String]` | 小写进程名称字符串的切片，在发现过程中应被静默忽略。 |

## 返回值

成功时返回 `Ok(())`。如果调用 `CreateToolhelp32Snapshot` 失败，则返回 `Err(windows::core::Error)`。

## 备注

- 该函数仅在 `cli.find_mode` 为 `true` 时执行其主体逻辑，否则作为空操作返回 `Ok(())`。
- 函数内部调用 `CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)` 获取所有正在运行的进程的快照，然后通过 `Process32FirstW` / `Process32NextW` 进行迭代。
- 每个进程名称通过 `String::from_utf16_lossy` 转换为小写，并在 `szExeFile` 缓冲区的第一个空字符处截断。
- 仅当满足以下**所有**条件时，进程才会被记录（通过 `log_process_find`）：
  1. 进程名称**不在**内部查找失败集合中（该集合包含先前发现失败的名称，抑制这些名称以减少日志噪音）。
  2. 进程名称**未出现**在任何等级的 `configs.process_level_configs` 或 `configs.thread_level_configs` 中。
  3. 进程名称**未包含**在 `blacklist` 中。
  4. `is_affinity_unset(pid, name)` 返回 `true`，表示该进程仍然使用系统默认亲和性掩码，且未被其他工具修改。
- 迭代完成后，快照句柄通过 `CloseHandle` 关闭。
- 此函数在每次主循环迭代中调用一次，不受等级限制，因此发现输出可能在每个轮询间隔出现。
- 此函数中的所有 Win32 调用均在 `unsafe` 块内进行。

## 要求

| 要求 | 值 |
|------|------|
| 模块 | `main.rs` |
| 调用者 | [main](main.md)（每次轮询迭代调用一次） |
| 被调用者 | `CreateToolhelp32Snapshot`、`Process32FirstW`、`Process32NextW`、`CloseHandle`、[is_affinity_unset](../winapi.rs/is_affinity_unset.md)、[log_process_find](../logging.rs/log_process_find.md) |
| 权限 | 建议拥有 `SeDebugPrivilege` 以获得完整的进程枚举可见性 |
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
*提交：[29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
