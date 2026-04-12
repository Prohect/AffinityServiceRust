# print_config_help 函数 (cli.rs)

打印配置文件格式帮助信息到控制台或日志。

## 语法

```rust
pub fn print_config_help()
```

## 参数

此函数不接受参数。

## 返回值

此函数不返回值。

## 备注

`print_config_help` 遍历 [`get_config_help_lines`](get_config_help_lines.md) 返回的所有静态帮助行，并逐行通过 `log!` 宏输出。

此函数是 [`get_config_help_lines`](get_config_help_lines.md) 的便捷包装器，将帮助行向量转换为格式化的控制台/日志输出。它不会自行设置 `USE_CONSOLE` 标志——调用者需确保输出目标已正确配置。

### 输出内容

输出包含配置文件的格式说明，涵盖以下内容：

- 配置行字段格式：`process:priority:affinity:cpuset:prime:io:memory:ideal:grade`
- 各字段的可选值说明（优先级、亲和性、CPU 集、prime 线程、I/O 优先级、内存优先级、理想处理器、grade）
- CPU 别名定义方式
- 分组语法示例

### 调用场景

- 由 [`print_help_all`](print_help_all.md) 在打印 CLI 帮助之后调用，作为完整帮助的一部分
- 可被其他需要输出配置帮助的上下文独立调用

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/cli.rs |
| **源码行** | L231–L235 |
| **调用者** | [`print_help_all`](print_help_all.md) |
| **调用** | [`get_config_help_lines`](get_config_help_lines.md) |

## 另请参阅

- [cli.rs 模块概述](README.md)
- [get_config_help_lines](get_config_help_lines.md) — 返回帮助行的底层函数
- [print_help_all](print_help_all.md) — 打印完整帮助（CLI + 配置格式）
- [print_help](print_help.md) — 打印基本帮助
- [print_cli_help](print_cli_help.md) — 打印详细 CLI 帮助