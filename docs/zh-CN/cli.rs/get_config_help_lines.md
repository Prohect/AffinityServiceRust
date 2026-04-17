# get_config_help_lines 函数 (cli.rs)

返回一个包含完整配置文件参考模板的静态字符串切片向量。此模板记录了 INI 风格的配置格式，包括术语、字段描述、CPU 规格格式、优先级级别、理想处理器语法和进程分组语法。返回的行适合嵌入到转换后的配置文件顶部或打印到控制台。

## 语法

```AffinityServiceRust/src/cli.rs#L168-170
pub fn get_config_help_lines() -> Vec<&'static str> {
    vec![
        r#"
```

## 参数

无。

## 返回值

类型：`Vec<&'static str>`

一个包含一个或多个静态字符串切片的向量。每个切片是一个多行的注释前缀文档文本块（行以 `##` 开头）。内容涵盖：

| 部分 | 描述 |
|------|------|
| **术语** | P-core、E-core 的定义，以及 Intel 混合 CPU 的 `p`、`pp`、`e` 简写。 |
| **配置格式** | 完整的字段顺序：`process_name:priority:affinity:cpuset:prime_cpus[@prefixes]:io_priority:memory_priority:ideal_processor:grade`。 |
| **CPU 规格格式** | 所有支持的格式，包括范围（`0-7`）、单独 CPU（`0;4;8`）、十六进制位掩码（`0xFF`）和别名引用（`*alias`）。 |
| **优先级级别** | 进程优先级、I/O 优先级和内存优先级字段的有效值。 |
| **理想处理器语法** | `*alias[@prefix1;prefix2;...]` 格式及多段链接示例。 |
| **进程分组** | 命名和匿名 `{ }` 分组语法，用于将单条规则应用于多个进程。 |

## 备注

- 返回的字符串使用 `##` 作为注释前缀（双井号），这是 AffinityServiceRust 配置解析器识别的注释语法（以 `#` 开头的行是注释）。
- 此函数由 [convert](../config.rs/convert.md) 调用，用于在转换后的 Process Lasso 配置文件前添加帮助头部，也由 [print_config_help](print_config_help.md) 调用以在控制台上显示参考信息。
- 内容作为 `&'static str` 字面量编译到二进制文件中，因此除了 `Vec` 本身之外没有文件 I/O 或额外的内存分配。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `cli` |
| 调用方 | [print_config_help](print_config_help.md)、[print_help_all](print_help_all.md)、[convert](../config.rs/convert.md) |
| 被调用方 | 无 |
| API | 无 |
| 权限 | 无 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| print_config_help | [print_config_help](print_config_help.md) |
| print_help_all | [print_help_all](print_help_all.md) |
| convert | [convert](../config.rs/convert.md) |
| read_config | [read_config](../config.rs/read_config.md) |

---
*提交：[37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
