# get_cpu_set_information 函数 (winapi.rs)

返回对延迟初始化、互斥锁保护的系统 CPU 集合信息缓存的引用。首次访问时，缓存通过调用 `GetSystemCpuSetInformation` 枚举系统上的所有 CPU 集合来填充。

## 语法

```rust
pub fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>>
```

## 参数

此函数不接受参数。

## 返回值

返回 `&'static Mutex<Vec<CpuSetData>>` —— 一个指向全局延迟初始化互斥锁的引用，该互斥锁包含一个 [CpuSetData](CpuSetData.md) 条目的向量。每个条目将一个 CPU Set ID 映射到其逻辑处理器索引。

返回的引用具有 `'static` 生命周期，因为它指向 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 全局静态变量，该变量通过 `Lazy` 初始化一次，并在程序运行期间一直存在。

## 备注

- 此函数是 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 静态变量的薄访问器。在首次访问之后，它不会执行任何分配或系统调用。
- 底层 `Lazy` 初始化器调用 `GetSystemCpuSetInformation` 两次：第一次确定所需的缓冲区大小，第二次检索数据。结果被解析为 `Vec<CpuSetData>`，其中包含 `(id, logical_processor_index)` 对。
- 如果初始的 `GetSystemCpuSetInformation` 调用失败，将通过 `log_to_find` 写入诊断消息，且返回的向量将为空。
- 调用者在读取数据之前必须锁定互斥锁。由于 CPU 集合拓扑在操作系统启动后的整个生命周期内是固定的，初始化后内部数据不会再改变。
- 此函数由 [cpusetids_from_indices](cpusetids_from_indices.md)、[cpusetids_from_mask](cpusetids_from_mask.md)、[indices_from_cpusetids](indices_from_cpusetids.md) 和 [mask_from_cpusetids](mask_from_cpusetids.md) 在内部调用。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `winapi.rs` |
| **调用者** | [cpusetids_from_indices](cpusetids_from_indices.md)、[cpusetids_from_mask](cpusetids_from_mask.md)、[indices_from_cpusetids](indices_from_cpusetids.md)、[mask_from_cpusetids](mask_from_cpusetids.md) |
| **被调用者** | 无（仅为访问器；初始化时调用 Windows API 的 `GetSystemCpuSetInformation`） |
| **Win32 API** | [GetSystemCpuSetInformation](https://learn.microsoft.com/en-us/windows/win32/api/systeminfomationapi/nf-systeminfomationapi-getsystemcpusetinformation)（在 `Lazy` 初始化期间） |
| **权限** | 无需特殊权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| CpuSetData 结构体 | [CpuSetData](CpuSetData.md) |
| CPU_SET_INFORMATION 静态变量 | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| cpusetids_from_indices | [cpusetids_from_indices](cpusetids_from_indices.md) |
| cpusetids_from_mask | [cpusetids_from_mask](cpusetids_from_mask.md) |
| indices_from_cpusetids | [indices_from_cpusetids](indices_from_cpusetids.md) |
| mask_from_cpusetids | [mask_from_cpusetids](mask_from_cpusetids.md) |

---

*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
