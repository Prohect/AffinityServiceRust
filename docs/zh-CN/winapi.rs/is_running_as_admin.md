# is_running_as_admin 函数 (winapi.rs)

检查当前进程是否以管理员（提升的）权限运行。

## 语法

```rust
pub fn is_running_as_admin() -> bool
```

## 参数

此函数不接受参数。

## 返回值

如果当前进程以提升的管理员令牌运行，则返回 `true`。如果进程以标准用户权限运行，或检查本身失败，则返回 `false`。

## 备注

此函数通过打开当前进程令牌并检查其提升状态来确定是否具有管理员权限。检查通过以 `TOKEN_QUERY` 访问权限打开进程令牌并查询令牌的提升状态来执行。

该结果由 [`main`](../main.rs/main.md) 使用，以决定是否需要 UAC 提升。如果进程未以管理员身份运行且未设置 `--no-uac` 标志，则调用 [`request_uac_elevation`](request_uac_elevation.md) 以提升权限重新启动进程。

如果令牌查询因任何原因失败（例如对自身进程令牌的访问不足），函数保守地返回 `false`，这将触发 UAC 提升流程。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L437–L466 |
| **调用方** | [`main`](../main.rs/main.md)（main.rs） |
| **Windows API** | [OpenProcessToken](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken)、[GetTokenInformation](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation) |

## 另请参阅

- [request_uac_elevation](request_uac_elevation.md)
- [enable_debug_privilege](enable_debug_privilege.md)
- [winapi.rs 模块概述](README.md)