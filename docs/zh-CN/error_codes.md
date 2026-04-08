# 错误代码模块文档

Windows 错误代码转换。

## 概述

本模块将 Windows 错误代码映射为人类可读的字符串以便记录。

## 调用者

- [apply.rs](apply.md) - 转换 API 错误
- [winapi.rs](winapi.md) - 转换句柄操作错误
- [logging.rs](logging.md) - 错误消息格式化

## 函数

### error_from_code_win32

将 Win32 错误代码转换为字符串。

```rust
pub fn error_from_code_win32(code: u32) -> String
```

**参数：**
- `code` - 来自 `GetLastError()` 的 Win32 错误代码

**返回：**
- 命名常量字符串（例如 `"ACCESS_DENIED"`）
- 或格式化的十六进制代码（例如 `"WIN32_ERROR_CODE_0x00000005"`）

**映射的代码：**

| 代码 | 名称 | 描述 |
|------|------|------|
| 0 | `SUCCESS` | 操作成功 |
| 2 | `FILE_NOT_FOUND` | 文件未找到 |
| 5 | `ACCESS_DENIED` | 访问被拒绝 |
| 6 | `INVALID_HANDLE` | 无效句柄 |
| 8 | `NOT_ENOUGH_MEMORY` | 内存不足 |
| 31 | `ERROR_GEN_FAILURE` | 一般故障 |
| 87 | `INVALID_PARAMETER` | 无效参数 |
| 122 | `INSUFFICIENT_BUFFER` | 缓冲区太小 |
| 126 | `MOD_NOT_FOUND` | 模块未找到 |
| 127 | `PROC_NOT_FOUND` | 过程未找到 |
| 193 | `BAD_EXE_FORMAT` | 可执行格式错误 |
| 565 | `TOO_MANY_THREADS` | 线程太多 |
| 566 | `THREAD_NOT_IN_PROCESS` | 线程不在进程中 |
| 567 | `PAGEFILE_QUOTA_EXCEEDED` | 页面文件配额超限 |
| 571 | `IO_PRIVILEGE_FAILED` | I/O 特权失败 |
| 577 | `INVALID_IMAGE_HASH` | 无效映像哈希 |
| 633 | `DRIVER_FAILED_SLEEP` | 驱动程序休眠转换失败 |
| 998 | `NOACCESS` | 内存访问无效 |
| 1003 | `CALLER_CANNOT_MAP_VIEW` | 无法映射视图 |
| 1006 | `VOLUME_CHANGED` | 卷已更改 |
| 1007 | `FULLSCREEN_MODE` | 全屏模式 |
| 1008 | `INVALID_HANDLE_STATE` | 无效句柄状态 |
| 1058 | `SERVICE_DISABLED` | 服务已禁用 |
| 1060 | `SERVICE_DOES_NOT_EXIST` | 服务不存在 |
| 1062 | `SERVICE_NOT_STARTED` | 服务未启动 |
| 1073 | `ALREADY_RUNNING` | 服务已在运行 |
| 1314 | `PRIVILEGE_NOT_HELD` | 未持有所需特权 |
| 1330 | `INVALID_ACCOUNT_NAME` | 无效账户名 |
| 1331 | `LOGON_FAILURE` | 登录失败 |
| 1332 | `ACCOUNT_RESTRICTION` | 账户限制 |
| 1344 | `NO_LOGON_SERVERS` | 无登录服务器 |
| 1346 | `RPC_AUTH_LEVEL_MISMATCH` | RPC 身份验证级别不匹配 |
| 1444 | `INVALID_THREAD_ID` | 无效线程 ID |
| 1445 | `NON_MDICHILD_WINDOW` | 不是 MDI 子窗口 |
| 1450 | `NO_SYSTEM_RESOURCES` | 系统资源不足 |
| 1460 | `TIMEOUT` | 操作超时 |
| 1453 | `QUOTA_EXCEEDED` | 配额超限 |
| 1455 | `PAGEFILE_TOO_SMALL` | 页面文件太小 |
| 1500 | `EVT_INVALID_CHANNEL` | 无效事件通道 |
| 1503 | `EVT_CHANNEL_ALREADY_EXISTS` | 事件通道已存在 |

**示例：**
```rust
let code = unsafe { GetLastError().0 };
let msg = error_from_code_win32(code);
// code=5 → msg="ACCESS_DENIED"
// code=9999 → msg="WIN32_ERROR_CODE_0x0000270F"
```

### error_from_ntstatus

将 NTSTATUS 代码转换为字符串。

```rust
pub fn error_from_ntstatus(status: i32) -> String
```

**参数：**
- `status` - 来自 NT API 函数的 NTSTATUS

**返回：**
- 命名常量字符串（例如 `"STATUS_ACCESS_DENIED"`）
- 或格式化的十六进制代码（例如 `"NTSTATUS_0xC0000005"`）

**映射的代码：**

| 代码 | 名称 | 描述 |
|------|------|------|
| 0x00000000 | `STATUS_SUCCESS` | 成功 |
| 0x00000001 | `STATUS_WAIT_1` | 等待 1 |
| 0xC0000001 | `STATUS_UNSUCCESSFUL` | 未指定的错误 |
| 0xC0000002 | `STATUS_NOT_IMPLEMENTED` | 未实现 |
| 0xC0000003 | `STATUS_INVALID_INFO_CLASS` | 无效信息类 |
| 0xC0000004 | `STATUS_INFO_LENGTH_MISMATCH` | 信息长度不匹配 |
| 0xC0000008 | `STATUS_INVALID_HANDLE` | 无效句柄 |
| 0xC000000D | `STATUS_INVALID_PARAMETER` | 无效参数 |
| 0xC0000017 | `STATUS_NO_MEMORY` | 内存不足 |
| 0xC0000018 | `STATUS_CONFLICTING_ADDRESSES` | 地址冲突 |
| 0xC0000022 | `STATUS_ACCESS_DENIED` | 访问被拒绝 |
| 0xC0000023 | `STATUS_BUFFER_TOO_SMALL` | 缓冲区太小 |
| 0xC0000034 | `STATUS_OBJECT_NAME_NOT_FOUND` | 对象名未找到 |
| 0xC000004B | `STATUS_THREAD_IS_TERMINATING` | 线程正在终止 |
| 0xC0000061 | `STATUS_PRIVILEGE_NOT_HELD` | 未持有特权 |
| 0xC00000BB | `STATUS_NOT_SUPPORTED` | 不支持 |
| 0xC000010A | `STATUS_PROCESS_IS_TERMINATING` | 进程正在终止 |

**常见 NTSTATUS 值：**

| 值 | 含义 |
|-------|---------|
| `0` (0x00000000) | 成功 |
| 负数 | 错误（高位设置） |
| `0xC0000005` | 访问冲突 |
| `0xC0000022` | 访问被拒绝 |
| `0xC0000034` | 对象未找到 |

**示例：**
```rust
let status = NtSetInformationProcess(...).0;
let msg = error_from_ntstatus(status);
// status=-1073741790 (0xC0000022) → msg="STATUS_ACCESS_DENIED"
```

## 设计说明

### 覆盖范围

这些函数提供了进程/线程管理期间遇到的最常见错误代码的映射。未知代码格式化为十六进制以便调试。

### Win32 vs NTSTATUS

**Win32 错误代码：**
- 大多数 Win32 API 通过 `GetLastError()` 返回
- 范围：0 到 65535（通常）
- 格式：`ERROR_*` 常量

**NTSTATUS 代码：**
- 由 NT 原生 API 返回
- 范围：32 位有符号（0 表示成功，负数表示错误）
- 格式：`STATUS_*` 常量
- 通常比 Win32 错误更具体

### 转换

记录时使用 `i32::cast_unsigned()` 将 NTSTATUS 转换为 `u32`：

```rust
let status: NTSTATUS = ...;
let status_u32 = i32::cast_unsigned(status.0);
```

## 使用示例

### Win32 API 错误

```rust
match unsafe { SetProcessAffinityMask(handle, mask) } {
    Ok(_) => {},
    Err(_) => {
        let code = unsafe { GetLastError().0 };
        log!("Failed: {}", error_from_code_win32(code));
        // "Failed: ACCESS_DENIED"
    }
}
```

### NT API 错误

```rust
let status = unsafe {
    NtQueryInformationProcess(handle, class, ptr, len, &mut ret_len)
}.0;

if status < 0 {
    log!("Failed: {}", error_from_ntstatus(status));
    // "Failed: STATUS_ACCESS_DENIED"
}
```

### 错误代码日志记录

```rust
// 在 apply.rs 错误处理中
log_error_if_new(pid, name, operation, error_code, result, || {
    format!("{}: [{}] {}-{}", 
        fn_name, 
        error_from_code_win32(error_code),
        pid, 
        name
    )
});
// 输出："apply_affinity: [ACCESS_DENIED] 1234-notepad.exe"
```

## 依赖

- 无外部 crate 依赖
- 纯 Rust 字符串格式化

## 扩展

添加新的错误代码：

1. 向适当的函数添加匹配分支
2. 使用一致的命名（UPPER_SNAKE_CASE）
3. 优先使用标准 Windows 常量名
4. 按数字顺序放置

示例：
```rust
1234 => "NEW_ERROR_CODE".to_string(),
```
