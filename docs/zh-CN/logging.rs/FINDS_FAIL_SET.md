# FINDS_FAIL_SET 静态变量 (logging.rs)

跟踪在 `-find` 模式中因 `ACCESS_DENIED` 而无法查询亲和性的进程名称。[`is_affinity_unset`](../winapi.rs/is_affinity_unset.md) 使用此集合跳过对已知失败进程的重试。

## 语法

```rust
static FINDS_FAIL_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
```

## 成员

该静态变量在 `Mutex` 后持有一个 `HashSet<String>`。每个条目是一个在 `-find` 模式中因访问被拒绝而查询失败的进程名称（例如 `"csrss.exe"`）。

## 备注

在 `-find` 模式下，应用程序会枚举所有正在运行的进程并尝试查询其亲和性设置。某些系统进程或受保护进程会始终返回 `ACCESS_DENIED` 错误。`FINDS_FAIL_SET` 记录这些进程名称，以便后续循环迭代中 [`is_affinity_unset`](../winapi.rs/is_affinity_unset.md) 可以直接跳过它们，而无需再次尝试打开进程句柄。

这种去重策略具有以下优势：

- **减少系统调用** — 避免对已知不可访问的进程反复执行 `OpenProcess` 调用。
- **减少日志噪音** — 防止每次循环迭代都记录相同的访问拒绝错误。
- **提高性能** — 跳过注定失败的操作，加快每次循环迭代的处理速度。

### 与 FINDS_SET 的区别

[`FINDS_SET`](FINDS_SET.md) 跟踪**成功发现**的进程名称，用于发现日志的去重；而 `FINDS_FAIL_SET` 跟踪**查询失败**的进程名称，用于跳过后续重试。两者共同服务于 `-find` 模式，但分别处理成功和失败的情况。

### 线程安全

所有对 `HashSet` 的访问都通过 `Mutex` 同步。锁在检查和插入操作期间被持有，然后立即释放。

### 生命周期

该集合在应用程序的整个生命周期内持续增长且不会被清除。一旦某个进程名称被标记为失败，即使该进程后续重新启动或权限发生变化，在同一会话中也不会再次尝试查询。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/logging.rs |
| **源码行** | L69 |
| **使用方** | [`is_affinity_unset`](../winapi.rs/is_affinity_unset.md) |

## 另请参阅

- [FINDS_SET 静态变量](FINDS_SET.md)
- [is_affinity_unset](../winapi.rs/is_affinity_unset.md)
- [logging.rs 模块概述](README.md)