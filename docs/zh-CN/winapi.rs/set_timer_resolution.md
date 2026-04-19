# set_timer_resolution 函数 (winapi.rs)

通过未公开的 `NtSetTimerResolution` NT API 将 Windows 全局计时器分辨率设置为用户指定的间隔。这允许服务请求更高频率的系统计时器 tick，从而减少时间敏感工作负载的调度延迟。

## 语法

```rust
pub fn set_timer_resolution(cli: &CliArgs)
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `cli` | `&CliArgs` | 解析后的命令行参数引用。`time_resolution` 字段指定期望的计时器分辨率，单位为 100 纳秒。值为 `0` 时禁用该功能（无操作）。 |

## 返回值

此函数不返回值。成功或失败通过日志输出传达。

## 说明

### 分辨率单位

`cli.time_resolution` 值以 100 纳秒 (100ns) 为单位表示，与 `NtSetTimerResolution` 的原生单位匹配。常见值：

| 期望分辨率 | 值 (100ns 单位) | 等效值 |
|--------------------|---------------------|------------|
| 0.5 毫秒 | `5000` | 0.5000 毫秒 |
| 1.0 毫秒 | `10000` | 1.0000 毫秒 |
| 15.6 毫秒 (默认) | `156250` | 15.6250 毫秒 |

### 行为

1. 如果 `cli.time_resolution == 0`，函数立即返回，不调用任何 API。
2. 否则，使用 `set_resolution = true` 调用 `NtSetTimerResolution` 请求指定的间隔。
3. API 在其 `p_current_resolution` 输出参数中返回*先前*（"elder"）计时器分辨率。
4. 成功时（`NTSTATUS >= 0`），请求的分辨率和先前的分辨率都以毫秒为单位记录（值 ÷ 10,000）。
5. 失败时（`NTSTATUS < 0`），NTSTATUS 代码以十六进制格式记录。

### 系统级影响

`NtSetTimerResolution` 影响全局 Windows 计时器分辨率，而不仅仅是调用进程。系统使用任何进程请求的*最小*（最精确）分辨率。当请求进程退出时，其分辨率请求会自动释放，系统可能会恢复到更粗糙的间隔。

### 日志输出示例

**成功:**
```
Succeed to set timer resolution: 0.5000ms
elder timer resolution: 156250
```

**失败:**
```
Failed to set timer resolution: 0xC0000022
```

### NtSetTimerResolution 签名

```c
NTSTATUS NtSetTimerResolution(
    ULONG   DesiredResolution,  // 单位为 100ns
    BOOLEAN SetResolution,      // TRUE 设置，FALSE 重置
    PULONG  CurrentResolution   // 接收先前的分辨率
);
```

这是从 `ntdll.dll` 直接导入的未公开 NT API，通过 winapi.rs 中的 `#[link(name = "ntdll")]` FFI 块。

### 与多媒体定时器的关系

这实现了与 `winmm.dll` 的已文档化 `timeBeginPeriod`/`timeEndPeriod` API 相同的效果，但直接使用更底层的 NT 接口。NT API 提供更高的粒度（100ns 单位 vs `timeBeginPeriod` 的 1ms 单位）。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用方** | `src/main.rs` 中的主启动逻辑 |
| **被调用方** | `NtSetTimerResolution` (ntdll.dll FFI) |
| **NT API** | `NtSetTimerResolution`（未公开，通过 `ntdll.dll` 链接） |
| **特权** | 无需任何特权；任何进程都可以请求更高的计时器分辨率 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [winapi.rs](README.md) |
| CLI 参数解析 | [cli.rs](../cli.rs/README.md) |
| NT FFI 声明 | [winapi.rs](README.md)（外部 FFI 部分） |

*文档版本：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*