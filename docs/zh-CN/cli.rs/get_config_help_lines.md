# get_config_help_lines 函数 (cli.rs)

返回包含配置文件格式文档模板的静态字符串切片向量。此模板既用于交互式帮助显示，也用于嵌入到转换后的配置文件作为头注释块。

## 语法

```rust
pub fn get_config_help_lines() -> Vec<&'static str>
```

## 参数

此函数不接受任何参数。

## 返回值

`Vec<&'static str>` —— 包含一个或多个静态字符串切片的向量，每个切片都保存着记录配置文件格式的多行注释块。字符串使用 `##` 注释前缀，因此可以直接写入 `.ini` 配置文件中。

## 说明

返回的模板涵盖以下章节：

| 章节 | 描述 |
|---------|-------------|
| **术语** | 定义 P 核、E 核和线程表示法 (`p`、`pp`、`e`)，用于 Intel 混合 CPU 拓扑。 |
| **配置格式** | 记录冒号分隔的规则语法：`process_name:priority:affinity:cpuset:prime_cpus:io_priority:memory_priority:ideal_processor:grade`。 |
| **CPU 指定格式** | 解释所有支持的 CPU 指定语法：范围 (`0-7`)、单独核心 (`0;4;8`)、单个核心 (`7`)、十六进制位掩码 (`0xFF`) 和别名引用 (`*pcore`)。警告 `7` 表示核心 7，而不是 0-2 核心的位掩码。 |
| **优先级级别** | 列出 `priority`、`io_priority` 和 `memory_priority` 字段的有效值。 |
| **理想处理器语法** | 记录基于启动模块匹配的线程到 CPU 分配的 `*alias[@prefix1;prefix2]` 格式。 |
| **进程组** | 解释 `{ }` 语法用于将多个进程分组到单个规则下，支持命名和匿名变体。 |

### 使用场景

- **[print_config_help](print_config_help.md)** 遍历返回的向量并将每个元素记录到控制台。
- **[print_help_all](print_help_all.md)** 调用 `print_config_help` 作为完整帮助输出的第二部分。
- `config.rs` 中的 `convert` 函数将这些行作为生成配置文件中的头注释嵌入，以便用户拥有内联文档。

### 设计说明

函数返回 `Vec<&'static str>` 而非单个 `&'static str`，以便调用方可以独立遍历和处理各个块。目前，向量包含一个元素，但签名支持未来扩展到多个逻辑部分。

## 要求

| | |
|---|---|
| **模块** | `src/cli.rs` |
| **调用方** | [print_config_help](print_config_help.md)、[print_help_all](print_help_all.md)、`config::convert` |
| **被调用方** | 无 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 模块概述 | [cli.rs](README.md) |
| 将配置帮助打印到控制台 | [print_config_help](print_config_help.md) |
| 完整帮助打印函数 | [print_help_all](print_help_all.md) |
| CLI 参数解析器 | [parse_args](parse_args.md) |
| 配置解析器 | [ConfigResult](../config.rs/ConfigResult.md) |

*记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*