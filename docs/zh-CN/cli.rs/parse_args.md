# parse_args 函数 (cli.rs)

将原始命令行参数切片解析为 [CliArgs](CliArgs.md) 结构体。解析器对参数向量执行单次线性扫描，将每个元素与已知的标志字符串进行匹配，并为带值标志额外消耗一个元素。未知标志会被静默忽略，使解析器对未来的新增保持前向兼容。

## 语法

```rust
pub fn parse_args(args: &[String], cli: &mut CliArgs) -> Result<()>
```

## 参数

`args`

命令行参数字符串切片，通常通过 `std::env::args().collect()` 获取。第一个元素（`args[0]`）是可执行文件路径，会被跳过——解析从索引 1 开始。

`cli`

对已通过 `CliArgs::new()` 初始化为默认值的 [CliArgs](CliArgs.md) 结构体的可变引用。解析器在遇到匹配的标志时覆盖相应的字段。未出现对应标志的字段保留其默认值。

## 返回值

无条件返回 `Ok(())`。`Result<()>` 返回类型（其中 `Result` 为 `windows::core::Result`）的存在是为了将来能够报告格式错误的参数，但当前实现永远不会返回错误。未识别的标志以及带值标志缺少值（当该标志是最后一个参数时）的情况会被静默忽略。

## 备注

### 标志格式

所有标志使用单横线前缀（例如 `-help`、`-config`）。少数标志也接受双横线变体（例如 `--help`、`--helpall`、`--dry-run`），以及传统的 `?` 和 `/?` 形式用于帮助。标志匹配区分大小写，在需要的地方提供了显式的大小写变体别名：

| 规范标志 | 别名 |
|---------|------|
| `-help` | `--help`、`-?`、`/?`、`?` |
| `-helpall` | `--helpall` |
| `-noUAC` | `-nouac` |
| `-dryrun` | `-dry-run`、`--dry-run` |
| `-noDebugPriv` | `-nodebugpriv` |
| `-noIncBasePriority` | `-noincbasepriority` |
| `-no_etw` | `-noetw` |

### 带值标志

需要值的标志会消耗参数向量中的下一个元素（`args[i + 1]`）。解析器在每个匹配分支中通过 `if i + 1 < args.len()` 条件防止越界访问。如果该标志作为最后一个参数出现且后面没有跟随值，则匹配分支不会触发，该标志被视为未知令牌（静默忽略）。

| 标志 | 值类型 | 默认值 | 约束 |
|------|--------|--------|------|
| `-interval` | `u64`（毫秒） | 5000 | 通过 `.max(16)` 限制最小值为 16 |
| `-loop` | `u32`（迭代次数） | `None`（无限） | 通过 `.max(1)` 限制最小值为 1 |
| `-resolution` | `u32`（100 纳秒单位） | 0 | 无限制；0 表示不设置 |
| `-config` | `String`（文件路径） | `"config.ini"` | 无验证 |
| `-blacklist` | `String`（文件路径） | `None` | 包装为 `Some` |
| `-in` | `String`（文件路径） | `None` | 包装为 `Some` |
| `-out` | `String`（文件路径） | `None` | 包装为 `Some` |

对于数值标志（`-interval`、`-loop`、`-resolution`），值通过 `str::parse()` 解析。如果解析失败，`.unwrap_or()` 提供默认值（interval 为 5000，loop 为 1，resolution 为 0）。

### 副作用

两个标志直接修改全局状态，而不是（或除了）在 `CliArgs` 上设置字段：

- **`-console`** — 通过 `get_use_console!()` 宏将全局 `USE_CONSOLE` 静态变量设置为 `true`。`CliArgs` 中没有对应的字段，因为控制台模式由日志基础设施消费，而非主循环。
- **`-validate`** — 设置 `cli.validate_mode = true` **并且** 将 `USE_CONSOLE` 设置为 `true`，因为验证输出始终用于交互式审查。

### 未知标志处理

任何不匹配已知标志的参数会被静默跳过。这意味着：

- 拼写错误的标志（例如 `-consle`）会被忽略且不发出警告。
- 没有前置标志的位置参数或裸值会被忽略。
- 将来添加到解析器的新标志不会破坏传递未知标志的现有脚本。

### 解析顺序

解析器从左到右处理参数。如果一个标志出现多次，对于带值标志，最后一次出现的值生效（因为每次出现都会覆盖字段）。对于布尔标志，它们只能被设置为 `true`——没有机制在标志被设置后将其取消。

### 典型用法

```rust
let args: Vec<String> = env::args().collect();
let mut cli = CliArgs::new();
parse_args(&args, &mut cli)?;
```

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `cli` |
| 调用者 | [main](../main.rs/main.md) |
| 被调用者 | `get_use_console!()` 宏（全局日志状态） |
| API | 无（纯参数解析，有一个全局副作用） |
| 权限 | 不适用 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 参数容器结构体 | [CliArgs](CliArgs.md) |
| 基本帮助输出 | [print_help](print_help.md) |
| 详细帮助输出 | [print_cli_help](print_cli_help.md) |
| 组合帮助输出 | [print_help_all](print_help_all.md) |
| 调用 parse_args 的入口点 | [main](../main.rs/main.md) |


## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd