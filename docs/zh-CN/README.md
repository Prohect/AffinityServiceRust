# AffinityServiceRust 文档

本目录包含 AffinityServiceRust 代码库的完整文档。

每个源代码模块（`src/*.rs`）以目录形式组织文档，每个顶层项（函数、结构体、枚举、静态变量）拥有独立的 markdown 文件，遵循类 MSDN 文档架构。

## 结构

| 目录 | 源文件 | 描述 |
|------|--------|------|
| [main.rs/](main.rs/) | `src/main.rs` | 入口点、主循环和编排 |
| [cli.rs/](cli.rs/) | `src/cli.rs` | 命令行参数解析和帮助文本 |
| [config.rs/](config.rs/) | `src/config.rs` | 配置解析、CPU 规格和工具 |
| [apply.rs/](apply.rs/) | `src/apply.rs` | 配置应用逻辑 |
| [scheduler.rs/](scheduler.rs/) | `src/scheduler.rs` | 基于滞后的 Prime 线程调度器 |
| [process.rs/](process.rs/) | `src/process.rs` | 进程和线程枚举 |
| [priority.rs/](priority.rs/) | `src/priority.rs` | 优先级级别定义 |
| [winapi.rs/](winapi.rs/) | `src/winapi.rs` | Windows API 包装器 |
| [logging.rs/](logging.rs/) | `src/logging.rs` | 日志和错误跟踪 |
| [error_codes.rs/](error_codes.rs/) | `src/error_codes.rs` | 错误代码转换 |

## 文档架构

每个项文件遵循以下结构：

| 章节 | 描述 |
|------|------|
| **标题** | `# 项名 类型 (module.rs)` |
| **简短描述** | 单段摘要 |
| **语法** | 完整 Rust 签名代码块 |
| **参数** | 逐参数描述（函数） |
| **成员** | 逐字段描述（结构体/枚举） |
| **返回值** | 函数返回内容 |
| **备注** | 算法、示例、边界情况、平台说明 |
| **要求** | 模块、调用者、被调用者、Windows API、特权等 |

项之间使用相对 markdown 链接进行交叉引用：

```
[ProcessConfig](config.rs/ProcessConfig.md)
[apply_priority](apply.rs/apply_priority.md)
[PrimeThreadScheduler](scheduler.rs/PrimeThreadScheduler.md)
```

## 快速参考

### 用户指南

- **配置格式：**参见 [read_config](config.rs/read_config.md) 和 [parse_cpu_spec](config.rs/parse_cpu_spec.md)
- **命令行选项：**参见 [CliArgs](cli.rs/CliArgs.md) 和 [parse_args](cli.rs/parse_args.md)
- **优先级级别：**参见 [ProcessPriority](priority.rs/ProcessPriority.md)、[IOPriority](priority.rs/IOPriority.md)、[MemoryPriority](priority.rs/MemoryPriority.md)
- **Prime 线程调度：**参见 [PrimeThreadScheduler](scheduler.rs/PrimeThreadScheduler.md)

### 开发者指南

- **主循环：**参见 [main](main.rs/main.md) 和 [apply_config](main.rs/apply_config.md)
- **添加新设置：**参见 [apply.rs/](apply.rs/) 了解 apply 函数模式
- **错误处理：**参见 [is_new_error](logging.rs/is_new_error.md) 和 [Operation](logging.rs/Operation.md)
- **Windows API 集成：**参见 [winapi.rs/](winapi.rs/) 了解句柄管理和 CPU 集操作
- **进程枚举：**参见 [ProcessSnapshot](process.rs/ProcessSnapshot.md)