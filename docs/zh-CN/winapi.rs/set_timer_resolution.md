# set_timer_resolution 函数 (winapi.rs)

通过 `NtSetTimerResolution` 设置系统计时器分辨率。

## 语法

```rust
pub fn set_timer_resolution(cli: &CliArgs)
```

## 参数

`cli` — 指向 [`CliArgs`](../cli.rs/CliArgs.md) 的引用，包含 `time_resolution` 值（以 100ns 为单位）。

## 备注

使用 CLI 参数中的分辨率值调用 `NtSetTimerResolution`。成功时记录新的和之前的分辨率值，失败时记录 NTSTATUS 代码。之前内联在 `main()` 中，现已提取为独立函数。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L823–L837 |
| **调用方** | [`main`](../main.rs/main.md) |

## 另请参阅

- [winapi.rs 模块概述](README.md)