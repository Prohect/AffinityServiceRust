# set_timer_resolution 函数 (winapi.rs)

通过 NT 原生 API `NtSetTimerResolution` 将系统定时器分辨率设置为调用者指定的值。这允许 AffinityServiceRust 通过请求比默认 ~15.6 ms 更小的定时器间隔来提高系统范围内的计时精度（例如 `Sleep`、可等待定时器）。

## 语法

```rust
pub fn set_timer_resolution(cli: &CliArgs)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cli` | `&CliArgs` | 对已解析的命令行参数的引用。`cli.time_resolution` 字段（`u32`）指定所需的定时器分辨率，单位为 100 纳秒（例如 `5000` = 0.5 ms）。 |

## 返回值

此函数不返回值。成功或失败通过 `log!` 宏写入的日志消息进行通信。

## 备注

### 机制

该函数调用 `NtSetTimerResolution(desired_resolution, true, &mut current_resolution)`，其中：

- `desired_resolution` 为 `cli.time_resolution`（单位为 100 纳秒）。
- 第二个参数（`set_resolution: true`）请求**设置**分辨率而非查询。
- `current_resolution` 接收更改**之前**生效的定时器分辨率（"先前"分辨率）。

### NTSTATUS 处理

| 条件 | 行为 |
|------|------|
| `NTSTATUS < 0`（失败） | 记录 `"Failed to set timer resolution: 0x{NTSTATUS}"`。 |
| `NTSTATUS >= 0`（成功） | 记录请求的分辨率（毫秒，4 位小数）和先前（"elder"）分辨率。 |

分辨率值通过除以 `10000.0` 转换为毫秒显示（因为单位是 100 纳秒）。

### 分辨率值示例

| `time_resolution` 值 | 等效间隔 |
|----------------------|----------|
| `156250` | 15.6250 ms（Windows 默认值） |
| `10000` | 1.0000 ms |
| `5000` | 0.5000 ms（500 µs） |

### 重要副作用

- **系统范围影响。** `NtSetTimerResolution` 影响整个操作系统的全局定时器分辨率，而不仅仅是调用进程。当分辨率被提高时，所有进程都受益于更高精度的计时，但功耗可能会增加。
- **持续生效直到恢复。** 提高的分辨率在调用进程运行期间持续有效，直到调用 `NtSetTimerResolution` 并将 `set_resolution` 设为 `false`。当进程退出时，Windows 自动将分辨率恢复为任何剩余进程请求的次高分辨率。
- **最小分辨率。** 操作系统强制执行依赖于硬件的最小定时器间隔（通常为 0.5 ms）。低于此下限的请求会成功，但会被钳制到支持的最小值。

### 平台说明

- **仅限 Windows。** `NtSetTimerResolution` 是由 `ntdll.dll` 导出的未公开 NT 原生 API，通过 `winapi.rs` 顶部的 `#[link(name = "ntdll")]` extern 块链接。
- 此 API 与 `winmm.dll` 中的 `timeBeginPeriod` / `timeEndPeriod` 使用相同的机制，但无需多媒体库。

### 不安全代码

函数体包裹在 `unsafe` 块中，因为 `NtSetTimerResolution` 是通过原始 `ntdll` FFI 绑定的外部函数调用。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `winapi.rs` |
| **调用者** | `main.rs` — 在配置了 `cli.time_resolution` 时于启动期间调用。 |
| **被调用者** | `NtSetTimerResolution` (ntdll)、`log!` 宏 → [`log_message`](../logging.rs/log_message.md) |
| **API** | NT 原生 API — `NtSetTimerResolution` |
| **权限** | 无明确要求，但在安全策略严格的系统上可能受组策略限制。 |
| **平台** | Windows |

## 另请参阅

| 主题 | 链接 |
|------|------|
| logging 模块 | [logging.rs](../logging.rs/README.md) |
| enable_debug_privilege | [enable_debug_privilege](enable_debug_privilege.md) |
| winapi 模块概述 | [README](README.md) |

---
*提交：[37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
