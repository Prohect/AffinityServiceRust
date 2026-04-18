# print_config_help 函数 (cli.rs)

将完整的配置文件参考模板打印到当前日志输出。此函数遍历 [get_config_help_lines](get_config_help_lines.md) 返回的行，并使用 `log!` 宏逐行写入。

## 语法

```AffinityServiceRust/src/cli.rs#L262-266
pub fn print_config_help() {
    for line in get_config_help_lines() {
        log!("{}", line);
    }
}
```

## 参数

此函数不接受参数。

## 返回值

此函数不返回值。

## 备注

`print_config_help` 是对 [get_config_help_lines](get_config_help_lines.md) 的轻量包装，通过 `log!` 宏将每行发送到活跃的日志接收器。与 [print_help](print_help.md) 和 [print_help_all](print_help_all.md) 不同，此函数**不会**设置控制台输出标志——调用方需要在调用此函数之前确保输出已路由到所需的目标。

打印的内容涵盖完整的配置文件语法参考，包括：

- P 核、E 核和超线程表示法的术语定义
- 配置行格式和字段描述
- 所有支持的 CPU 规格格式（范围、十六进制掩码、单独索引、别名）
- 进程优先级、I/O 优先级和内存优先级的优先级级别枚举
- 带模块前缀过滤的理想处理器语法
- 使用 `{ }` 块的进程分组语法

此函数由 [print_help_all](print_help_all.md) 作为组合帮助输出的第二部分调用，也可在仅需要配置文件参考时独立使用。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `cli` |
| 调用方 | [print_help_all](print_help_all.md)、外部入口点 |
| 被调用方 | [get_config_help_lines](get_config_help_lines.md)、`log!` 宏 |
| API | 内部 |
| 权限 | 无 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| get_config_help_lines | [get_config_help_lines](get_config_help_lines.md) |
| print_help_all | [print_help_all](print_help_all.md) |
| print_help | [print_help](print_help.md) |
| print_cli_help | [print_cli_help](print_cli_help.md) |
| cli 模块概述 | [README](README.md) |

---
*提交：[29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*