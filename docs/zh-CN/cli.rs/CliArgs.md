# CliArgs 结构体 (cli.rs)

AffinityServiceRust 接受的所有命令行参数的容器。每个运行时选项——轮询间隔、模式标志、文件路径、权限开关和调试选项——都作为此结构体的公共字段存储。`CliArgs` 实例在启动时通过 `CliArgs::new()` 创建一次，由 [parse_args](parse_args.md) 填充，然后在整个服务生命周期内以共享引用 (`&CliArgs`) 的形式传递，用于控制轮询循环、热重载逻辑和实用工具模式中的行为。

## 语法

```rust
#[derive(Debug, Default)]
pub struct CliArgs {
    pub interval_ms: u64,
    pub help_mode: bool,
    pub help_all_mode: bool,
    pub convert_mode: bool,
    pub autogroup_mode: bool,
    pub find_mode: bool,
    pub validate_mode: bool,
    pub process_logs_mode: bool,
    pub dry_run: bool,
    pub config_file_name: String,
    pub blacklist_file_name: Option<String>,
    pub in_file_name: Option<String>,
    pub out_file_name: Option<String>,
    pub no_uac: bool,
    pub loop_count: Option<u32>,
    pub time_resolution: u32,
    pub log_loop: bool,
    pub skip_log_before_elevation: bool,
    pub no_debug_priv: bool,
    pub no_inc_base_priority: bool,
    pub no_etw: bool,
}
```

## 成员

`interval_ms` (`u64`)

主循环连续迭代之间的轮询间隔，以毫秒为单位。对应 `-interval <ms>` CLI 标志。默认值：**5000**。[parse_args](parse_args.md) 强制的最小值：**16**。

`help_mode` (`bool`)

为 `true` 时，[main](../main.rs/main.md) 函数通过 [print_help](print_help.md) 打印基本帮助信息并立即退出。由 `-help`、`--help`、`-?`、`/?` 或 `?` 设置。默认值：**false**。

`help_all_mode` (`bool`)

为 `true` 时，[main](../main.rs/main.md) 函数通过 [print_help_all](print_help_all.md) 打印包含 CLI 和配置文件的详细组合帮助信息并立即退出。由 `-helpall` 或 `--helpall` 设置。默认值：**false**。

`convert_mode` (`bool`)

为 `true` 时，服务以转换模式运行，将 Process Lasso 配置文件转换为 AffinityServiceRust 格式。使用 `in_file_name` 作为输入，`out_file_name` 作为输出。由 `-convert` 设置。默认值：**false**。

`autogroup_mode` (`bool`)

为 `true` 时，服务以自动分组模式运行，读取配置文件，将具有相同设置的规则分组为命名组块，并将结果写入输出文件。使用 `in_file_name` 作为输入，`out_file_name` 作为输出。由 `-autogroup` 设置。默认值：**false**。

`find_mode` (`bool`)

为 `true` 时，在每次轮询迭代中启用未管理进程发现功能。具有默认（系统范围）CPU affinity 且不在配置或黑名单中的进程将被记录到 `.find.log` 文件中。由 `-find` 设置。默认值：**false**。

`validate_mode` (`bool`)

为 `true` 时，服务加载并验证配置文件的语法错误和未定义的别名，打印报告后退出，不进入轮询循环。同时强制使用控制台输出。由 `-validate` 设置。默认值：**false**。

`process_logs_mode` (`bool`)

为 `true` 时，服务以日志处理模式运行。它扫描日志目录中的 `.find.log` 文件，过滤掉已知/黑名单中的进程，使用 Everything 搜索 (`es.exe`) 解析可执行文件路径，并将结果写入文件。由 `-processlogs` 设置。默认值：**false**。

`dry_run` (`bool`)

为 `true` 时，服务模拟所有更改而不进行任何 Win32 API 调用。[ApplyConfigResult](../apply.rs/ApplyConfigResult.md) 记录将会被更改的内容，服务在一次迭代后退出。由 `-dryrun`、`-dry-run` 或 `--dry-run` 设置。默认值：**false**。

`config_file_name` (`String`)

配置文件的路径。对应 `-config <file>`。默认值：**`"config.ini"`**。

`blacklist_file_name` (`Option<String>`)

可选的黑名单文件路径，包含要从管理和发现模式中排除的进程名称。对应 `-blacklist <file>`。默认值：**`None`**。

`in_file_name` (`Option<String>`)

可选的输入文件路径。语义取决于当前活动的模式：`-convert` 和 `-autogroup` 的源配置，或 `-processlogs` 的日志目录。对应 `-in <file>`。默认值：**`None`**（每种模式应用各自的默认值，例如 `-processlogs` 的默认值为 `"logs"`）。

`out_file_name` (`Option<String>`)

可选的输出文件路径。语义取决于当前活动的模式：`-convert`、`-autogroup` 和 `-processlogs` 的目标文件。对应 `-out <file>`。默认值：**`None`**（每种模式应用各自的默认值，例如 `-processlogs` 的默认值为 `"new_processes_results.txt"`）。

`no_uac` (`bool`)

为 `true` 时，在服务检测到未以管理员身份运行时，抑制自动 UAC 提升请求。服务将以有限权限继续运行并记录警告。由 `-noUAC` 或 `-nouac` 设置。默认值：**false**。

`loop_count` (`Option<u32>`)

可选的最大轮询迭代次数。为 `Some(n)` 时，服务在完成 `n` 次循环后退出。适用于测试和脚本化运行。强制最小值：**1**。对应 `-loop <count>`。默认值：**`None`**（无限循环）。

`time_resolution` (`u32`)

系统计时器分辨率，单位为 100 纳秒。值 `5210` 对应 0.5210 毫秒。非零时，在启动时调用 [set_timer_resolution](../winapi.rs/set_timer_resolution.md)。对应 `-resolution <t>`。默认值：**0**（不更改计时器分辨率）。

`log_loop` (`bool`)

为 `true` 时，在每次轮询迭代开始时输出一条日志消息，包括循环编号。用于调试计时和循环行为。由 `-logloop` 设置。默认值：**false**。

`skip_log_before_elevation` (`bool`)

为 `true` 时，在 UAC 提升之前的启动阶段抑制日志输出。这可以防止服务以提升权限重新启动时出现重复或混乱的日志条目。由 `-skip_log_before_elevation` 设置。默认值：**false**。

`no_debug_priv` (`bool`)

为 `true` 时，服务在启动时不请求 `SeDebugPrivilege`。这会限制服务打开受保护进程句柄的能力。由 `-noDebugPriv` 或 `-nodebugpriv` 设置。默认值：**false**。

`no_inc_base_priority` (`bool`)

为 `true` 时，服务在启动时不请求 `SeIncreaseBasePriorityPrivilege`。这会阻止为其他进程设置高或实时进程优先级。由 `-noIncBasePriority` 或 `-noincbasepriority` 设置。默认值：**false**。

## 备注

### 构造

`CliArgs::new()` 返回一个具有合理默认值的结构体：

- `interval_ms` = 5000
- `config_file_name` = `"config.ini"`
- 所有 `bool` 字段 = `false`
- 所有 `Option` 字段 = `None`
- 所有 `u32` 字段 = 0

其余字段通过 `..Default::default()` 设置为其 `Default` trait 值。结构体上的 `#[derive(Default)]` 属性提供此行为。

### 模式互斥性

模式标志（`help_mode`、`help_all_mode`、`convert_mode`、`autogroup_mode`、`validate_mode`、`process_logs_mode`、`dry_run`、`find_mode`）在解析层面并不互斥。但是，[main](../main.rs/main.md) 函数按优先级顺序检查它们，并在处理完第一个活动模式后退出：

1. `help_mode`
2. `help_all_mode`
3. `convert_mode`
4. `autogroup_mode`
5. `validate_mode`（在配置加载后检查）
6. `process_logs_mode`（在配置和黑名单加载后检查）

`find_mode` 和 `dry_run` 标志与主轮询循环兼容，不会导致提前退出。如果设置了多个互斥的模式标志，只有优先级最高的那个会生效。

### 生命周期

单个 `CliArgs` 实例在 [main](../main.rs/main.md) 的栈上创建，存在于程序的整个持续时间内。它以共享引用的形式传递给 [hotreload_config](../config.rs/hotreload_config.md)、[hotreload_blacklist](../config.rs/hotreload_blacklist.md)、[process_find](../main.rs/process_find.md) 和 [set_timer_resolution](../winapi.rs/set_timer_resolution.md) 等函数。该结构体在 [parse_args](parse_args.md) 返回后不再被修改。

### 控制台输出

有两个标志会强制使用控制台输出（`*get_use_console!() = true`）：`-console`（直接在 [parse_args](parse_args.md) 中处理）和 `-validate`（隐含 `-console`）。`-console` 标志在 `CliArgs` 中没有对应字段，因为它直接写入 [logging](../logging.rs/README.md) 模块中的全局 `USE_CONSOLE` 静态变量。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `cli` |
| 调用者 | [main](../main.rs/main.md)、[hotreload_config](../config.rs/hotreload_config.md)、[hotreload_blacklist](../config.rs/hotreload_blacklist.md)、[process_find](../main.rs/process_find.md)、[set_timer_resolution](../winapi.rs/set_timer_resolution.md) |
| 填充者 | [parse_args](parse_args.md) |
| 派生 | `Debug`、`Default` |
| 权限 | 不适用（仅数据结构） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 参数解析器 | [parse_args](parse_args.md) |
| 基本帮助输出 | [print_help](print_help.md) |
| 完整帮助输出 | [print_help_all](print_help_all.md) |
| 消费 CliArgs 的入口点 | [main](../main.rs/main.md) |
| 配置加载 | [read_config](../config.rs/read_config.md) |
| 配置热重载 | [hotreload_config](../config.rs/hotreload_config.md) |
| 黑名单热重载 | [hotreload_blacklist](../config.rs/hotreload_blacklist.md) |
