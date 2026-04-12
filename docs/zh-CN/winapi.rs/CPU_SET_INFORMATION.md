# CPU_SET_INFORMATION 静态变量 (winapi.rs)

延迟初始化的、互斥锁保护的向量，包含系统上所有逻辑处理器的 CPU 集信息。这是 CPU 集 ID 与逻辑处理器索引之间映射的权威数据源。

## 语法

```rust
static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>> = Lazy::new(|| {
    // 调用 GetSystemCpuSetInformation 进行填充
    // ...
});
```

## 成员

该静态变量在 `Mutex` 后持有一个 `Vec<CpuSetData>`。每个 [`CpuSetData`](CpuSetData.md) 条目包含：

- `id` — 系统分配的 CPU 集 ID
- `logical_processor_index` — 逻辑处理器的从零开始的索引

## 备注

该向量在首次访问时通过调用 Windows API `GetSystemCpuSetInformation` 填充一次。此 API 枚举系统上的所有 CPU 集，每个 CPU 集对应一个逻辑处理器。初始化是延迟的（通过 `once_cell::sync::Lazy`），以避免在静态初始化时调用 Windows API。

所有对该向量的访问都通过 `Mutex` 同步，确保线程安全。实际上数据在初始化后是只读的，但由于带有内部可变性的 `Lazy<T>` 需要 `Sync`，互斥锁仍然是必要的。

此静态变量是 [`get_cpu_set_information`](get_cpu_set_information.md) 的后备存储，并被所有 CPU 集转换函数间接使用：

- [`cpusetids_from_indices`](cpusetids_from_indices.md)
- [`cpusetids_from_mask`](cpusetids_from_mask.md)
- [`indices_from_cpusetids`](indices_from_cpusetids.md)
- [`mask_from_cpusetids`](mask_from_cpusetids.md)

该向量按逻辑处理器索引排序，与 `GetSystemCpuSetInformation` 返回的顺序一致。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L312 |
| **访问方式** | [`get_cpu_set_information`](get_cpu_set_information.md) |
| **Windows API** | [GetSystemCpuSetInformation](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getsystemcpusetinformation) |

## 另请参阅

- [CpuSetData 结构体](CpuSetData.md)
- [get_cpu_set_information](get_cpu_set_information.md)
- [cpusetids_from_indices](cpusetids_from_indices.md)
- [cpusetids_from_mask](cpusetids_from_mask.md)