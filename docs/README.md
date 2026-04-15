# Docs

the docs of this project follows a MSDN style doc schema.
that is: 
each source file is a dir(/<$locale>/<$source_file_name>/, eg. "/en-US/config.rs/") with a README.md which contains an overview that follows [`schema`](#schema-for-overview-for-a-souce-file).
each top level item of a souce file is a markdown in that dir(eg. "\en-US\config.rs\read_config.md") that follows [`schema`](#schema-for-top-level-item-for-a-souce-file).

## Schema for Overview for a Souce File
| Section | Description |
|---------|-------------|
| **Title** | `# ModuleName type (<$project_name>)` |
| **Short description** | One-paragraph summary |
| **<$item_type>** | table for items that is this type |
| **See Also** | table for see-alsos |

## Schema for Top Level Item for a Souce File

| Section | Description |
|---------|-------------|
| **Title** | `# ItemName type (<$source_file_name>)` |
| **Short description** | One-paragraph summary |
| **Syntax** | Rust code block with full signature |
| **Parameters** | Per-parameter description (functions) |
| **Members** | Per-field description (structs/enums) |
| **Return value** | What the function returns |
| **Remarks** | Algorithms, improtant side effect, examples, edge cases, platform notes |
| **Requirements** | Table of module, callers, callees, API, privileges |
| **See Also** | table for see-alsos |

## Cross-references between items use relative markdown links

[ProcessConfig](en-US/config.rs/ProcessConfig.md)

[return value for ProcessSnapshot::take()](en-US/process.rs/ProcessSnapshot.md#return-value)

[log_error_if_new](../docs/en-US/apply.rs/log_error_if_new.md)

## Module Directory

### en-US

| Module | Overview | Items |
|--------|----------|-------|
| [apply.rs](en-US/apply.rs/README.md) | Process/thread settings application | 16 |
| [cli.rs](en-US/cli.rs/README.md) | Command-line argument parsing | 7 |
| [collections.rs](en-US/collections.rs/README.md) | Type aliases and capacity constants | 8 |
| [config.rs](en-US/config.rs/README.md) | Configuration file parsing and hot-reload | 25 |
| [error_codes.rs](en-US/error_codes.rs/README.md) | Win32/NTSTATUS error formatting | 2 |
| [event_trace.rs](en-US/event_trace.rs/README.md) | ETW process start/stop monitoring | 4 |
| [logging.rs](en-US/logging.rs/README.md) | Logging, error deduplication, find output | 10 |
| [main.rs](en-US/main.rs/README.md) | Entry point and main polling loop | 7 |
| [priority.rs](en-US/priority.rs/README.md) | Priority enums (process, IO, memory, thread) | 5 |
| [process.rs](en-US/process.rs/README.md) | Process snapshot via NtQuerySystemInformation | 4 |
| [scheduler.rs](en-US/scheduler.rs/README.md) | Prime thread scheduler and stats tracking | 6 |
| [winapi.rs](en-US/winapi.rs/README.md) | Windows API wrappers and handle management | 27 |

### zh-CN

| 模块 | 概述 | 条目 |
|------|------|------|
| [apply.rs](zh-CN/apply.rs/README.md) | 进程/线程设置应用 | 16 |
| [cli.rs](zh-CN/cli.rs/README.md) | 命令行参数解析 | 7 |
| [collections.rs](zh-CN/collections.rs/README.md) | 类型别名与容量常量 | 8 |
| [config.rs](zh-CN/config.rs/README.md) | 配置文件解析与热重载 | 25 |
| [error_codes.rs](zh-CN/error_codes.rs/README.md) | Win32/NTSTATUS 错误格式化 | 2 |
| [event_trace.rs](zh-CN/event_trace.rs/README.md) | ETW 进程启动/停止监控 | 4 |
| [logging.rs](zh-CN/logging.rs/README.md) | 日志记录、错误去重、查找输出 | 10 |
| [main.rs](zh-CN/main.rs/README.md) | 入口点与主轮询循环 | 7 |
| [priority.rs](zh-CN/priority.rs/README.md) | 优先级枚举（进程、IO、内存、线程） | 5 |
| [process.rs](zh-CN/process.rs/README.md) | 通过 NtQuerySystemInformation 获取进程快照 | 4 |
| [scheduler.rs](zh-CN/scheduler.rs/README.md) | 主线程调度器与统计跟踪 | 6 |
| [winapi.rs](zh-CN/winapi.rs/README.md) | Windows API 封装与句柄管理 | 27 |

## CONTRIBUTING

Update en-US first, do not considering update for all locales at same time.
Translate for other locales after changes to en-US finishing, know about basic ideas about this project before translating.


## Documentation on Commit SHA

Always leave a git commit SHA as a clickable link at the bottom of every doc file.

### Format

```
[<7-char prefix>](https://github.com/Prohect/AffinityServiceRust/tree/<full SHA>)
```

### Example

```
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*
```

### Current commit

[b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)