# MemoryPriorityInformation 结构体 (priority.rs)

用于与 Win32 `MEMORY_PRIORITY_INFORMATION` 结构布局直接互操作的 `#[repr(C)]` 包装器，包装一个 `u32` 值。在通过 `SetProcessInformation` / `GetProcessInformation` 调用使用 `ProcessMemoryPriority` 信息类时用到，该类需要一个指向单个 `ULONG` 字段的指针。

## 语法

```rust
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct MemoryPriorityInformation(pub u32);
```

## 成员

| 成员 | 类型 | 描述 |
|--------|------|-------------|
| `0`（元组字段） | `u32` | 原始内存优先级值。对应于 Windows 定义的 `MEMORY_PRIORITY_*` 常量之一（例如 `MEMORY_PRIORITY_VERY_LOW` 到 `MEMORY_PRIORITY_NORMAL`）。 |

## 备注

此结构体的存在完全是为了提供一个大小正确且对齐正确的 FFI 类型，用于 `NtSetInformationProcess` / `SetProcessInformation` 调用，以设置内存优先级。`#[repr(C)]` 属性确保内存布局与 Win32 API 所期望的一致——单个 `ULONG`（4 字节，自然对齐）。

该结构体派生了 `PartialEq` 和 `Eq` 用于比较（例如，检查当前内存优先级是否匹配期望值后再进行修改），并派生了 `Clone` / `Copy` 以支持按值传递的便捷语义。

### 与 MemoryPriority 枚举的关系

[`MemoryPriority`](MemoryPriority.md) 是配置和日志记录中使用的类型安全、人类可读的枚举。当需要一个原始 `u32` 值用于 Win32 互操作时，`MemoryPriority::as_win_const()` 返回一个 `Option<MEMORY_PRIORITY>`，可以包装在 `MemoryPriorityInformation` 中进行 FFI 调用。这两种类型服务于互补的角色：

| 类型 | 用途 |
|------|---------|
| [`MemoryPriority`](MemoryPriority.md) | 配置解析、字符串显示、变体匹配 |
| `MemoryPriorityInformation` | 用于 Win32 `SetProcessInformation` 调用的 FFI 安全布局 |

### Win32 对应关系

Windows SDK 定义 `MEMORY_PRIORITY_INFORMATION` 如下：

```c
typedef struct _MEMORY_PRIORITY_INFORMATION {
    ULONG MemoryPriority;
} MEMORY_PRIORITY_INFORMATION;
```

`MemoryPriorityInformation(pub u32)` 是一个具有相同布局的 Rust newtype，使其可以安全地作为 `SetProcessInformation` 的 `lpProcessInformation` 参数传递。

## 要求

| | |
|---|---|
| **模块** | [`src/priority.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/priority.rs) |
| **调用方** | [`apply_memory_priority`](../apply.rs/apply_memory_priority.md) |
| **依赖项** | 无（纯 newtype 包装器） |
| **Win32 API** | [`SetProcessInformation`](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessinformation)，使用 `ProcessMemoryPriority` |
| **权限** | `PROCESS_SET_INFORMATION`（写入时由调用方需要） |

## 参见

| 主题 | 链接 |
|-------|------|
| 类型安全的内存优先级枚举 | [`MemoryPriority`](MemoryPriority.md) |
| 内存优先级应用函数 | [`apply_memory_priority`](../apply.rs/apply_memory_priority.md) |
| 模块概览 | [priority.rs](README.md) |
| 进程优先级枚举 | [`ProcessPriority`](ProcessPriority.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*