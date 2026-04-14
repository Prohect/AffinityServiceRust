# print_help_all 函数 (cli.rs)

通过将详细的 CLI 选项（通过 [print_cli_help](print_cli_help.md)）和配置文件格式模板（通过 [print_config_help](print_config_help.md)）合并为一个统一输出，打印 AffinityServiceRust 的完整帮助参考。这是命令行中可用的最全面的帮助信息，当用户传递 `-helpall` 或 `--helpall` 时调用。

## 语法

```rust
pub fn print_help_all()
```

## 参数

此函数不接受参数。

## 返回值

此函数没有返回值。

## 备注

### 实现

该函数执行三个步骤：

1. **强制控制台输出** — 通过 `*get_use_console!() = true` 将全局 `USE_CONSOLE` 静态变量设置为 `true`。这确保此函数及其被调用函数中所有后续的 `log!()` 调用都写入 stdout 而非日志文件，因为帮助输出始终面向交互式查看。

2. **打印 CLI 帮助** — 调用 [print_cli_help](print_cli_help.md) 输出完整的命令行参考，包括基本参数、操作模式、调试/测试选项和实用调试示例。

3. **打印分隔符** — 记录一个空行（`log!("")`）以在 CLI 参考和配置模板之间进行视觉分隔。

4. **打印配置帮助** — 调用 [print_config_help](print_config_help.md) 输出配置文件格式模板，描述冒号分隔的规则语法、每个字段的值选项、CPU 别名定义和组块语法。

### 强制控制台输出

此函数是除 [print_help](print_help.md) 之外唯一直接设置 `USE_CONSOLE` 全局变量的帮助函数。这样做是因为 [print_cli_help](print_cli_help.md) 和 [print_config_help](print_config_help.md) 都不会自行设置该全局变量——它们依赖调用方预先配置输出目标。通过在顶部设置一次，`print_help_all` 确保两个子函数都输出到控制台。

### 输出结构

此函数渲染的合并输出遵循以下布局：

```
=== COMMAND LINE OPTIONS ===
  （基本参数）
  （操作模式）
  （调试与测试选项）
  （调试示例）

=== CONFIGURATION FILE FORMAT ===
  ## ============================================
  ## AffinityServiceRust Configuration File
  ## ============================================
  ## Format: process:priority:affinity:cpuset:prime:io:memory:ideal:grade
  ## （字段描述）
  ## （别名示例）
  ## （组语法）
  ## ============================================
```

### 与其他帮助函数的关系

| 函数 | 范围 | 调用方 |
|------|------|--------|
| [print_help](print_help.md) | 仅常用选项和模式 | `-help`、`--help`、`-?`、`/?`、`?` |
| [print_cli_help](print_cli_help.md) | 包含调试选项的完整 CLI 参考 | **print_help_all**（本函数） |
| [print_config_help](print_config_help.md) | 配置文件格式模板 | **print_help_all**（本函数） |
| **print_help_all**（本函数） | CLI 参考 + 配置模板合并 | `-helpall`、`--helpall` |

### 调用流程

在 [main](../main.rs/main.md) 中，help-all 模式检查在加载任何配置之前、紧接在基本帮助检查之后执行：

```rust
if cli.help_all_mode {
    print_help_all();
    return Ok(());
}
```

这意味着该函数执行后程序立即退出，不会涉及配置文件、黑名单、权限或其他任何子系统。即使在配置文件不存在或格式错误的环境中也可以安全调用。

### 何时使用 `-helpall` 与 `-help`

- 使用 `-help`（调用 [print_help](print_help.md)）可以快速查看最常用的选项和模式名称。
- 使用 `-helpall`（调用本函数）可以在一次输出中获取完整的标志参考、调试选项、示例命令和配置文件语法模板。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `cli` |
| 调用方 | [main](../main.rs/main.md)（当 `cli.help_all_mode` 为 `true` 时） |
| 被调用方 | `get_use_console!()` 宏、[print_cli_help](print_cli_help.md)、[print_config_help](print_config_help.md)、`log!()` 宏 |
| API | 无 |
| 权限 | 不适用 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 基本帮助输出 | [print_help](print_help.md) |
| 详细 CLI 帮助（内部调用） | [print_cli_help](print_cli_help.md) |
| 配置文件帮助（内部调用） | [print_config_help](print_config_help.md) |
| 配置帮助行提供函数 | [get_config_help_lines](get_config_help_lines.md) |
| CLI 参数结构体 | [CliArgs](CliArgs.md) |
| 参数解析器 | [parse_args](parse_args.md) |
| 入口点 | [main](../main.rs/main.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd