# ProcessHandle struct (winapi.rs)

The `ProcessHandle` struct is an RAII wrapper that holds a set of Windows process handles with varying access levels. It guarantees that all contained handles are automatically closed when the struct is dropped. The struct maintains up to four handles per process — limited and full variants for both read (query) and write (set) operations — allowing callers to use the appropriate access level for each operation.

## Syntax

```rust
pub struct ProcessHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: Option<HANDLE>,
    pub w_limited_handle: HANDLE,
    pub w_handle: Option<HANDLE>,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `r_limited_handle` | `HANDLE` | Process handle opened with `PROCESS_QUERY_LIMITED_INFORMATION`. Always valid when the struct is constructed. |
| `r_handle` | `Option<HANDLE>` | Process handle opened with `PROCESS_QUERY_INFORMATION`. May be `None` if the higher-privilege open failed (e.g., protected processes). |
| `w_limited_handle` | `HANDLE` | Process handle opened with `PROCESS_SET_LIMITED_INFORMATION`. Always valid when the struct is constructed. |
| `w_handle` | `Option<HANDLE>` | Process handle opened with `PROCESS_SET_INFORMATION`. May be `None` if the higher-privilege open failed. |

## Remarks

- **RAII handle lifecycle.** The `Drop` implementation closes all valid handles automatically. `Option<HANDLE>` variants are only closed if they are `Some`. The limited handles (`r_limited_handle`, `w_limited_handle`) are always closed because they are guaranteed valid at construction time.

- **Access level fallback pattern.** The struct supports a two-tier access model. Limited handles (`PROCESS_QUERY_LIMITED_INFORMATION`, `PROCESS_SET_LIMITED_INFORMATION`) succeed for most processes, while full handles (`PROCESS_QUERY_INFORMATION`, `PROCESS_SET_INFORMATION`) may fail for protected or system processes. Callers should check whether `r_handle` or `w_handle` is `Some` before using them and fall back to the limited variants.

- **Construction.** Instances are created exclusively by [get_process_handle](get_process_handle.md), which opens all four handles and returns `None` if either of the required limited handles cannot be obtained. The full handles are best-effort.

- **Thread safety.** The struct does not implement `Send` or `Sync`. Handles should not be shared across threads; obtain a new `ProcessHandle` on each thread that needs access.

- **Platform.** Windows only. Relies on `windows::Win32::Foundation::HANDLE` and `CloseHandle`.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `winapi.rs` |
| Created by | [get_process_handle](get_process_handle.md) |
| Depends on | `windows::Win32::Foundation::{CloseHandle, HANDLE}` |
| Privileges | `SeDebugPrivilege` recommended for full-access handles on protected processes |
| Platform | Windows |

## See Also

| Topic | Link |
|-------|------|
| get_process_handle | [get_process_handle](get_process_handle.md) |
| ThreadHandle struct | [ThreadHandle](ThreadHandle.md) |
| process module | [process.rs](../process.rs/README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
