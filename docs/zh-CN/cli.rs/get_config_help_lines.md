# get_config_help_lines 函数 (cli.rs)

返回配置文件帮助模板行的向量，用于嵌入到转换输出文件中。

## 语法

```rust
pub fn get_config_help_lines() -> Vec<&'static str>
```

## 参数

此函数不接受参数。

## 返回值

返回 `Vec<&'static str>`，包含配置文件格式的注释模板行。每一行以 `##` 开头，构成一个完整的配置文件头部注释块。

返回的模板包含以下内容：

- 文件标题横幅
- 文档链接指引
- 配置行格式说明：`process:priority:affinity:cpuset:prime:io:memory:ideal:grade`
- 各字段的取值说明（process、priority、affinity、cpuset、prime、io、memory、ideal、grade）
- CPU 别名示例（`*a`、`*p`、`*e`）
- 分组语法示例

## 备注

此函数以结构化向量形式返回帮助文本，而非直接打印到控制台，使调用者可以灵活地将这些行嵌入到其他输出中。主要消费者是 [`convert`](../config.rs/convert.md) 函数，它在转换 Process Lasso 配置文件时将这些模板行作为文件头部写入输出文件。

[`print_config_help`](print_config_help.md) 是对此函数的简单包装，遍历返回的向量并逐行打印到日志/控制台。

返回的字符串均为 `'static` 生命周期，因为它们是编译时字面量，不涉及任何运行时分配。

### 配置格式摘要

每条规则行使用冒号分隔的九个字段：

| 字段 | 描述 | 示例 |
| --- | --- | --- |
| process | 可执行文件名 | `game.exe` |
| priority | 进程优先级 | `none`、`high`、`real time` |
| affinity | 硬 CPU 亲和性 | `0-7`、`0;4;8`、`0xFF`、`*alias` |
| cpuset | 软 CPU 偏好 | `*p`、`*e`、`*alias` |
| prime | Prime 线程 CPU | `?10*pN01`、`*p@module.dll` |
| io | I/O 优先级 | `none`、`very low`、`normal` |
| memory | 内存优先级 | `none`、`very low`、`normal` |
| ideal | 理想处理器 | `*alias[@prefix]`、`0` |
| grade | 应用频率 | `1`=每次循环，`5`=每5次 |

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/cli.rs |
| **源码行** | L199–L229 |
| **调用者** | [`print_config_help`](print_config_help.md)、[`convert`](../config.rs/convert.md) |
| **被调用** | 无（纯数据返回） |

## 另请参阅

- [print_config_help](print_config_help.md) — 将模板行打印到控制台
- [print_help_all](print_help_all.md) — 打印完整帮助（包含配置格式）
- [convert](../config.rs/convert.md) — Process Lasso 配置转换，使用此函数生成文件头部
- [cli.rs 模块概述](README.md)