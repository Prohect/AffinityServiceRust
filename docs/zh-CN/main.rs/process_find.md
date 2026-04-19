# process_find 函数 (main.rs)

通过 Win32 toolhelp 快照枚举所有正在运行的进程，并记录任何具有默认（完整）CPU 亲和性掩码且未被已加载配置或黑名单覆盖的进程。这是 `-find` 模式的每次迭代伴随函数，在每次主循环迭代结束时调用。

## 语法

```rust
fn process_find(
    cli: &CliArgs,
    configs: &ConfigResult,
    blacklist: &[String],
) -> Result<(), windows::core::Error>
```

## 参数

`cli: &CliArgs`

已解析的[命令行参数](../cli.rs/CliArgs.md)。仅检查 `find_mode` 标志——如果为 `false`，函数立即返回 `Ok(())` 且不执行任何工作。

`configs: &ConfigResult`

已加载的 [ConfigResult](../config.rs/ConfigResult.md)。会搜索所有等级的 `process_level_configs` 和 `thread_level_configs`，以确定进程名称是否已被管理。

`blacklist: &[String]`

在发现过程中应被静默忽略的小写进程名称列表。此列表中的进程即使具有默认亲和性也不会被记录。

## 返回值

`Result<(), windows::core::Error>` — 成功时返回 `Ok(())`。仅在 `CreateToolhelp32Snapshot` 失败时返回错误。

## 备注

当 `cli.find_mode` 为 `true` 时，函数执行以下步骤：

1. **快照** — 调用 `CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)` 捕获所有进程的时间点列表。
2. **迭代** — 使用 `Process32FirstW` / `Process32NextW` 遍历每个 `PROCESSENTRY32W` 条目。
3. **规范化** — 将进程名称从以空字符结尾的 UTF-16 `szExeFile` 字段转换为小写 `String`。
4. **过滤 — 已管理** — 检查进程名称是否存在于任何等级的 `configs.process_level_configs` 或 `configs.thread_level_configs` 中。如果找到，则跳过该进程。
5. **过滤 — 已列入黑名单** — 检查进程名称是否出现在 `blacklist` 向量中。如果找到，则跳过该进程。
6. **过滤 — 已记录** — 检查全局 `fail_find_set` 以避免在同一会话中重复记录相同的进程名称。
7. **过滤 — 亲和性检查** — 调用 [`is_affinity_unset`](../winapi.rs/is_affinity_unset.md) 以确定进程是否具有完整（默认）亲和性掩码。只有具有未修改亲和性的进程才被视为"未管理"并值得记录。
8. **记录** — 调用 [`log_process_find`](../logging.rs/log_process_find.md) 将发现的进程名称写入 `.find.log` 文件。
9. **清理** — 通过 `CloseHandle` 关闭快照句柄。

### 去重

全局 `fail_find_set` 防止相同的进程名称在每次轮询迭代中被重复记录。进程名称在首次记录时被添加到该集合中，直到服务重启才会被移除。这使得 `.find.log` 文件保持简洁，便于后续通过 [`process_logs`](process_logs.md) 进行分析。

### 亲和性启发式

该假设是：任何仍以系统默认完整亲和性掩码运行的进程都未被任何外部工具或 AffinityServiceRust 本身管理。这是一个简单的启发式方法；有意使用完整亲和性的进程也会被标记。

## 要求

| | |
|---|---|
| **模块** | `src/main.rs` |
| **调用方** | [main](main.md)（每次循环迭代结束时） |
| **被调用方** | `CreateToolhelp32Snapshot`、`Process32FirstW`、`Process32NextW`、`CloseHandle`、[`is_affinity_unset`](../winapi.rs/is_affinity_unset.md)、[`log_process_find`](../logging.rs/log_process_find.md) |
| **Win32 API** | [`CreateToolhelp32Snapshot`](https://learn.microsoft.com/zh-cn/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot)、[`Process32FirstW`](https://learn.microsoft.com/zh-cn/windows/win32/api/tlhelp32/nf-tlhelp32-process32firstw)、[`Process32NextW`](https://learn.microsoft.com/zh-cn/windows/win32/api/tlhelp32/nf-tlhelp32-process32nextw) |
| **权限** | 除启动时已获取的权限外无需额外权限（调试权限可提供更广泛的进程可见性） |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 会话后日志分析 | [process_logs](process_logs.md) |
| 亲和性检查辅助函数 | [is_affinity_unset](../winapi.rs/is_affinity_unset.md) |
| 发现模式日志记录器 | [log_process_find](../logging.rs/log_process_find.md) |
| 命令行标志 | [CliArgs](../cli.rs/CliArgs.md) |
| 模块概述 | [main.rs](README.md) |

*为提交 [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf) 记录*