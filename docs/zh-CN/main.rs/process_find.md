# process_find 函数 (main.rs)

扫描所有运行中的进程，发现未被配置规则或黑名单覆盖的进程并记录以供审查。

## 语法

```rust
fn process_find(
    cli: &CliArgs,
    configs: &HashMap<u32, HashMap<String, ProcessConfig>>,
    blacklist: &[String],
) -> Result<(), windows::core::Error>
```

## 备注

仅在启用 `-find` 模式时执行。使用 Win32 ToolHelp32 API 枚举进程，检查每个进程是否在配置和黑名单中。具有默认亲和性的未管理进程通过 [`log_process_find`](../logging.rs/log_process_find.md) 记录。

之前内联在 `main()` 中，现已提取为独立函数。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/main.rs |
| **源码行** | L207–L241 |
| **调用方** | [`main`](main.md) 主循环 |

## 另请参阅

- [main.rs 模块概述](README.md)