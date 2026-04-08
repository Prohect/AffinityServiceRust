# AffinityServiceRust 文档

本目录包含 AffinityServiceRust 代码库的完整文档。

## 结构

文档文件与源代码结构并行：

| 文档文件 | 源文件 | 描述 |
|----------|-------------|-------------|
| [main.md](main.md) | `src/main.rs` | 入口点和主循环 |
| [cli.md](cli.md) | `src/cli.rs` | CLI 解析和帮助文本 |
| [config.md](config.md) | `src/config.rs` | 配置解析和工具 |
| [apply.md](apply.md) | `src/apply.rs` | 配置应用逻辑 |
| [scheduler.md](scheduler.md) | `src/scheduler.rs` | Prime 线程调度器 |
| [process.md](process.md) | `src/process.rs` | 进程枚举 |
| [priority.md](priority.md) | `src/priority.rs` | 优先级级别定义 |
| [winapi.md](winapi.md) | `src/winapi.rs` | Windows API 包装器 |
| [logging.md](logging.md) | `src/logging.rs` | 日志和错误跟踪 |
| [error_codes.md](error_codes.md) | `src/error_codes.rs` | 错误代码转换 |

## 文档格式

每个文件包括：

- **概述** - 模块目的
- **调用者** - 调用者关系
- **数据结构** - 带字段的重要类型
- **函数** - 详细函数文档包括：
  - 目的
  - 参数
  - 返回值
  - 示例
  - 被调用者
- **依赖** - 所需模块/crate

## 快速参考

### 对于用户

- **配置格式：**参见 [cli.md](cli.md#configuration) 和 [config.md](config.md)
- **CPU 规格：**参见 [config.md](config.md#cpu-specification-parsing)
- **Prime 线程调度：**参见 [scheduler.md](scheduler.md)
- **优先级级别：**参见 [priority.md](priority.md)

### 对于开发者

- **架构概述：**参见 [main.md](main.md#main-loop)
- **添加新设置：**参见 [apply.md](apply.md#apply-functions)
- **错误处理：**参见 [logging.md](logging.md#error-deduplication)
- **Windows API 集成：**参见 [winapi.md](winapi.md)

## 源代码中的文档注释

基本文档保留在源代码中作为 `///` 文档注释：

- 算法解释
- 安全说明
- 需要上下文的内联示例
- 复杂逻辑描述

用户-facing 文档（帮助文本、配置格式指南）已移至本目录。
