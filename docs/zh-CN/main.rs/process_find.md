# process_find 函数 (main.rs)

使用 Win32 Toolhelp API 枚举所有正在运行的进程，并记录 CPU affinity 与系统默认值（所有核心）匹配的进程，表明该进程未被任何外部工具或策略管理。此函数实现了 `-find` 模式，当 `-find` 处于活动状态时，还会在每次轮询迭代结束时被调用，持续发现新的未管理进程。

## 语法

```rust
fn process_find(
    cli: &CliArgs,
    configs: &HashMap<u32, HashMap<String, ProcessConfig>>,
    blacklist: &[String],
) -> Result<(), windows::core::Error>
```

## 参数

`cli`

对 [CliArgs](../../zh-CN/cli.rs/CliArgs.md) 结构体的引用。函数检查 `cli.find_mode`；如果为 `false`，函数不执行任何操作并立即返回 `Ok(())`。

`configs`

对按等级分组的配置映射的引用。外层 `HashMap<u32, ...>` 以等级值为键，内层 `HashMap<String, ProcessConfig>` 以小写进程名为键。出现在任何等级中的进程被视为已管理，并从查找结果中排除。

`blacklist`

小写进程名的切片，这些进程应从发现结果中排除。被列入黑名单的进程是有意不管理的，不应被记录。

## 返回值

成功时返回 `Ok(())`，如果 `CreateToolhelp32Snapshot` 失败则返回 `windows::core::Error`。单个进程查询产生的错误（例如检查 affinity 时访问被拒绝）不会导致函数返回错误；这些进程通过 fail-find 集合被静默跳过。

## 备注

### 枚举方法

与主轮询循环使用 `NtQuerySystemInformation`（通过 [ProcessSnapshot](../process.rs/ProcessSnapshot.md)）获取详细线程级数据不同，此函数使用更轻量的 Toolhelp32 快照 API（`CreateToolhelp32Snapshot` 配合 `TH32CS_SNAPPROCESS`，然后使用 `Process32FirstW`/`Process32NextW`）。这已经足够，因为查找模式只需要进程名称和 PID——不需要线程枚举。

快照句柄在枚举完成后通过 `CloseHandle` 关闭。

### 过滤管道

对于快照中的每个进程，函数按以下顺序应用过滤器：

1. **Fail-find 集合** — 检查进程名是否在静态 `HashSet`（来自 `logging` 模块的 `FINDS_FAIL_SET`）中。之前 affinity 查询失败的进程（例如由于访问被拒绝）被排除，以避免重复的日志噪音。
2. **配置成员检查** — 检查进程名是否在 `configs` 的所有等级中。如果在任何等级中存在规则，则该进程已被管理，将被跳过。
3. **黑名单成员检查** — 检查进程名是否在 `blacklist` 中。被列入黑名单的进程将被跳过。
4. **Affinity 检查** — 调用 [is_affinity_unset](../winapi.rs/is_affinity_unset.md) 来确定进程的 CPU affinity 掩码是否与完整的系统掩码匹配（即所有逻辑处理器）。只有具有默认 affinity 的进程才被视为未管理。

如果进程通过了所有四个过滤器，则通过 [log_process_find](../logging.rs/log_process_find.md) 记录，该函数写入 `.find.log` 文件并将进程名添加到去重集合中，使其在每个会话中只被记录一次。

### 进程名规范化

进程名从 `PROCESSENTRY32W.szExeFile` 字段（以 null 结尾的 UTF-16 数组）中提取，通过 `String::from_utf16_lossy` 转换为 Rust `String`，并转换为小写。这确保了与配置规则和黑名单的大小写不敏感匹配。

### 调用上下文

此函数在主循环中每次轮询迭代结束时被调用，在所有进程级和线程级设置被应用之后。当以纯 `-find` 模式配合空配置运行时，它实际上也是服务的唯一目的——服务仍然轮询，但只发现进程而不应用任何设置。

### 与 `-processlogs` 的交互

此函数的日志记录所产生的 `.find.log` 文件是 [process_logs](process_logs.md) 函数（通过 `-processlogs` 模式调用）的输入。典型工作流程为：

1. 运行一段时间的 `-find` 模式以积累 `.find.log` 数据。
2. 运行 `-processlogs` 模式来分析日志，根据配置/黑名单进行过滤，并通过 Everything 搜索解析可执行文件路径。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `main` |
| 调用者 | [main](main.md)（每次轮询迭代结束时） |
| 被调用者 | `CreateToolhelp32Snapshot`、`Process32FirstW`、`Process32NextW`、`CloseHandle`、[is_affinity_unset](../winapi.rs/is_affinity_unset.md)、[log_process_find](../logging.rs/log_process_find.md) |
| API | Win32 Toolhelp32 (`TH32CS_SNAPPROCESS`)、`GetProcessAffinityMask`（通过 `is_affinity_unset`） |
| 权限 | `SeDebugPrivilege`（建议，用于查询受保护进程的 affinity） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 查找结果的日志处理 | [process_logs](process_logs.md) |
| Affinity 检查辅助函数 | [is_affinity_unset](../winapi.rs/is_affinity_unset.md) |
| 查找日志记录函数 | [log_process_find](../logging.rs/log_process_find.md) |
| CLI 参数和查找模式标志 | [CliArgs](../../zh-CN/cli.rs/CliArgs.md) |
| 主入口点和轮询循环 | [main](main.md) |