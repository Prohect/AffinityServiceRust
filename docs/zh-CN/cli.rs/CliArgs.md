# CliArgs 结构体 (cli.rs)

解析后的命令行参数集合。包含控制应用程序行为的所有运行时选项，包括操作模式、文件路径、调试开关和调度参数。

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
}
```

## 成员

### 调度与计时

`interval_ms`

主循环每次迭代之间的检查间隔（毫秒）。默认值为 `5000`。最小有效值为 `16`（由 [`parse_args`](parse_args.md) 强制）。通过 `-interval <ms>` 设置。

`time_resolution`

Windows 计时器分辨率，单位为 100 纳秒。值 `5210` 表示 0.5210ms。默认值为 `0`，表示不设置计时器分辨率。通过 `-resolution <t>` 设置。

`loop_count`

限制主循环执行次数。`None` 表示无限循环（默认），`Some(n)` 表示执行 `n` 次后退出。最小值为 `1`。通过 `-loop <count>` 设置，主要用于测试。

### 操作模式

`help_mode`

当为 `true` 时，打印基本帮助信息后立即退出。由 `-help`、`--help`、`-?`、`/?`、`?` 触发。

`help_all_mode`

当为 `true` 时，打印完整帮助信息（CLI 选项 + 配置格式）后退出。由 `-helpall`、`--helpall` 触发。

`convert_mode`

当为 `true` 时，将 Process Lasso 配置文件转换为本程序格式。需要 `-in <file>` 和 `-out <file>`。由 `-convert` 触发。

`autogroup_mode`

当为 `true` 时，将具有相同设置的规则自动分组为命名组块。需要 `-in <file>` 和 `-out <file>`。由 `-autogroup` 触发。

`find_mode`

当为 `true` 时，启用进程发现模式——在每次循环中扫描使用默认系统亲和性的进程，并将发现结果记录到 `.find.log` 文件。由 `-find` 触发。

`validate_mode`

当为 `true` 时，仅验证配置文件语法，不运行服务。同时自动启用控制台输出。由 `-validate` 触发。

`process_logs_mode`

当为 `true` 时，处理由 `-find` 模式生成的日志文件，发现新进程并使用 `es.exe` 搜索其可执行路径。由 `-processlogs` 触发。

`dry_run`

当为 `true` 时，仅显示将要执行的更改，但不实际应用任何配置。由 `-dryrun`、`-dry-run`、`--dry-run` 触发。

### 文件路径

`config_file_name`

配置文件路径。默认值为 `"config.ini"`。通过 `-config <file>` 设置。

`blacklist_file_name`

可选的黑名单文件路径，用于 `-find` 模式排除已知进程。通过 `-blacklist <file>` 设置。

`in_file_name`

可选的输入文件路径。用于 `-convert`（Process Lasso 配置文件）、`-autogroup`（配置文件）和 `-processlogs`（日志目录）模式。通过 `-in <file>` 设置。

`out_file_name`

可选的输出文件路径。用于 `-convert`、`-autogroup` 和 `-processlogs` 模式的输出结果。通过 `-out <file>` 设置。

### 权限与调试

`no_uac`

当为 `true` 时，禁止 UAC 提权请求。程序将以当前权限运行，可能无法管理所有进程。由 `-noUAC`、`-nouac` 触发。

`log_loop`

当为 `true` 时，在每次循环开始时记录一条日志消息（`"Loop N started"`），用于调试。由 `-logloop` 触发。

`skip_log_before_elevation`

当为 `true` 时，在 UAC 提权完成前抑制日志输出。由 `-skip_log_before_elevation` 触发。

`no_debug_priv`

当为 `true` 时，不请求 `SeDebugPrivilege` 特权。由 `-noDebugPriv`、`-nodebugpriv` 触发。

`no_inc_base_priority`

当为 `true` 时，不请求 `SeIncreaseBasePriorityPrivilege` 特权。由 `-noIncBasePriority`、`-noincbasepriority` 触发。

## 方法

| 方法 | 签名 | 描述 |
| --- | --- | --- |
| **new** | `pub fn new() -> Self` | 创建具有默认值的实例：`interval_ms = 5000`，`config_file_name = "config.ini"`，其余为 `Default`。 |

## 备注

`CliArgs` 在 [`main`](../main.rs/main.md) 函数开始时通过 `CliArgs::new()` 创建，然后传递给 [`parse_args`](parse_args.md) 进行填充。

该结构体派生了 `Default`，所有 `bool` 字段默认为 `false`，`Option` 字段默认为 `None`，数值字段默认为 `0`。`new()` 构造函数在此基础上覆盖了 `interval_ms` 和 `config_file_name` 的合理默认值。

### 模式优先级

[`main`](../main.rs/main.md) 按以下顺序检查模式标志，首个匹配的模式立即执行并退出：

1. `help_mode` → 打印基本帮助
2. `help_all_mode` → 打印完整帮助
3. `convert_mode` → 转换配置文件
4. `autogroup_mode` → 自动分组规则
5. `validate_mode` → 验证配置后退出
6. `process_logs_mode` → 处理发现日志

若未匹配任何工具模式，则进入主循环。`find_mode`、`dry_run` 等标志在主循环内持续生效。

### 快速调试命令

非管理员模式快速测试：

```
AffinityServiceRust.exe -console -noUAC -logloop -loop 3 -interval 2000 -config test.ini
```

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/cli.rs |
| **源码行** | L5–L36 |
| **构造者** | [`main`](../main.rs/main.md) |
| **填充者** | [`parse_args`](parse_args.md) |

## 另请参阅

- [cli.rs 模块概述](README.md)
- [parse_args](parse_args.md) — 解析命令行参数填充此结构体
- [print_help](print_help.md) — 打印基本帮助信息
- [print_help_all](print_help_all.md) — 打印完整帮助信息
- [main](../main.rs/main.md) — 使用此结构体驱动应用逻辑