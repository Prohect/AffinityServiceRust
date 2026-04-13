# get_cpu_set_information 函数 (winapi.rs)

返回对延迟初始化的、受互斥锁保护的全局 [CpuSetData](CpuSetData.md) 条目向量的引用，该向量描述了系统的 CPU 集合拓扑。首次调用时，底层的 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 静态变量通过查询 `GetSystemCpuSetInformation` 完成初始化；后续调用返回相同的缓存引用。

## 语法

```rust
pub fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>>
```

## 参数

无。

## 返回值

一个 `&'static Mutex<Vec<CpuSetData>>` 引用，指向全局 [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) 静态变量。调用方必须通过 `.lock().unwrap()` 获取互斥锁才能访问内部向量。返回的引用在进程的整个生命周期内有效。

## 备注

此函数是一个简单的访问器，用于解引用 `Lazy<Mutex<Vec<CpuSetData>>>` 静态变量。它的存在是为了封装全局变量并提供简洁的公共 API——调用方无需直接引用 `CPU_SET_INFORMATION`。

### 线程安全

`Mutex` 保证了对内部 `Vec<CpuSetData>` 的互斥访问。由于数据在初始化时一次性填充，之后不再修改，因此竞争仅限于简短的加锁/解锁序列。所有 CPU 集合转换函数（[cpusetids_from_indices](cpusetids_from_indices.md)、[cpusetids_from_mask](cpusetids_from_mask.md)、[indices_from_cpusetids](indices_from_cpusetids.md)、[mask_from_cpusetids](mask_from_cpusetids.md)）在遍历向量期间均持有互斥锁。

### 初始化触发

首次调用 `get_cpu_set_information`（或首次解引用 `CPU_SET_INFORMATION`）时会触发 `Lazy` 初始化器，其过程如下：

1. 使用零长度缓冲区调用 `GetSystemCpuSetInformation` 以确定所需的大小。
2. 分配所需大小的字节缓冲区。
3. 再次调用 `GetSystemCpuSetInformation` 填充缓冲区。
4. 遍历变长的 `SYSTEM_CPU_SET_INFORMATION` 条目，将每个 `(id, logical_processor_index)` 对提取为 [CpuSetData](CpuSetData.md)。

如果 `GetSystemCpuSetInformation` 失败，向量为空，并通过 `log_to_find` 记录一条消息。

### 典型用法

```rust
let guard = get_cpu_set_information().lock().unwrap();
for entry in guard.iter() {
    // 访问 entry.id 和 entry.logical_processor_index
}
```

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **可见性** | `pub` |
| **调用方** | [cpusetids_from_indices](cpusetids_from_indices.md)、[cpusetids_from_mask](cpusetids_from_mask.md)、[indices_from_cpusetids](indices_from_cpusetids.md)、[mask_from_cpusetids](mask_from_cpusetids.md) |
| **底层静态变量** | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| **API** | [`GetSystemCpuSetInformation`](https://learn.microsoft.com/en-us/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation)（仅在初始化期间） |
| **特权** | 无 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 全局 CPU 集合缓存 | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| CPU 集合条目类型 | [CpuSetData](CpuSetData.md) |
| CPU 索引 → CPU 集合 ID | [cpusetids_from_indices](cpusetids_from_indices.md) |
| 亲和性掩码 → CPU 集合 ID | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU 集合 ID → CPU 索引 | [indices_from_cpusetids](indices_from_cpusetids.md) |
| CPU 集合 ID → 亲和性掩码 | [mask_from_cpusetids](mask_from_cpusetids.md) |
| GetSystemCpuSetInformation (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) |