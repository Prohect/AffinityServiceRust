# get_cpu_set_information 函数 (winapi.rs)

返回全局 [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) 静态变量的引用，确保在首次访问时完成初始化。

## 语法

```rust
pub fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>>
```

## 参数

此函数不接受参数。

## 返回值

返回 `&'static Mutex<Vec<CpuSetData>>` — 指向延迟初始化的、互斥锁保护的 [`CpuSetData`](CpuSetData.md) 条目向量的引用，表示系统上的所有 CPU 集。

## 备注

此函数是一个轻量级访问器，返回 [`CPU_SET_INFORMATION`](CPU_SET_INFORMATION.md) 静态变量的引用。首次调用时，该静态变量通过查询 `GetSystemCpuSetInformation` 进行初始化，为系统上的每个逻辑处理器填充一个 [`CpuSetData`](CpuSetData.md) 条目。

调用方必须在读取向量内容之前锁定返回的互斥锁。应尽量缩短持锁时间以避免竞争。

此函数在代码库中所有需要 CPU 集 ID 查找或转换的地方使用，包括 [`cpusetids_from_indices`](cpusetids_from_indices.md)、[`cpusetids_from_mask`](cpusetids_from_mask.md)、[`indices_from_cpusetids`](indices_from_cpusetids.md) 和 [`mask_from_cpusetids`](mask_from_cpusetids.md)。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L353–L355 |
| **调用方** | [`cpusetids_from_indices`](cpusetids_from_indices.md)、[`cpusetids_from_mask`](cpusetids_from_mask.md)、[`indices_from_cpusetids`](indices_from_cpusetids.md)、[`mask_from_cpusetids`](mask_from_cpusetids.md) |
| **Windows API** | [GetSystemCpuSetInformation](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getsystemcpusetinformation)（通过延迟初始化） |

## 另请参阅

- [CPU_SET_INFORMATION 静态变量](CPU_SET_INFORMATION.md)
- [CpuSetData 结构体](CpuSetData.md)
- [cpusetids_from_indices](cpusetids_from_indices.md)
- [cpusetids_from_mask](cpusetids_from_mask.md)