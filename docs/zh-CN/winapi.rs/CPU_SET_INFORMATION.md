# CPU_SET_INFORMATION 静态变量 (winapi.rs)

延迟初始化、互斥锁保护的全局 [CpuSetData](CpuSetData.md) 条目向量，将系统上每个逻辑处理器映射到其 Windows CPU 集合 ID。此静态变量是 AffinityServiceRust 中 CPU 集合拓扑的唯一数据源，模块中所有 CPU 集合转换函数均会查询它。

## 语法

```rust
static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>> = Lazy::new(|| {
    Mutex::new({
        let mut cpu_set_data: Vec<CpuSetData> = Vec::new();
        let mut required_size: u32 = 0;

        let current_process = unsafe { GetCurrentProcess() };

        let _ = unsafe {
            GetSystemCpuSetInformation(None, 0, &mut required_size, Some(current_process), Some(0))
        };

        let mut buffer: Vec<u8> = vec![0u8; required_size as usize];

        let success = unsafe {
            GetSystemCpuSetInformation(
                Some(buffer.as_mut_ptr() as *mut SYSTEM_CPU_SET_INFORMATION),
                required_size,
                &mut required_size,
                Some(current_process),
                Some(0),
            )
            .as_bool()
        };

        if !success {
            log_to_find("GetSystemCpuSetInformation failed");
        } else {
            let mut offset = 0;
            while offset < required_size as usize {
                let entry = unsafe {
                    let entry_ptr = buffer.as_ptr().add(offset) as *const SYSTEM_CPU_SET_INFORMATION;
                    &*entry_ptr
                };

                let data = unsafe { extract_cpu_set_data(entry) };
                cpu_set_data.push(data);
                offset += entry.Size as usize;
            }
        }
        cpu_set_data
    })
});
```

## 成员

| 成员 | 类型 | 描述 |
|--------|------|-------------|
| 内部值 | `Vec<CpuSetData>` | 一个向量，包含系统上每个逻辑处理器对应的一个 [CpuSetData](CpuSetData.md) 条目。每个条目将处理器的不透明 Windows CPU 集合 ID 与其从零开始的逻辑处理器索引配对。向量长度等于当前进程可见的逻辑处理器数量。 |

## 备注

### 初始化

`CPU_SET_INFORMATION` 包装在 `once_cell::sync::Lazy` 中，因此初始化闭包仅在首次访问时执行一次。初始化过程对 `GetSystemCpuSetInformation` 执行两次调用：

1. **大小查询** — 第一次调用传递零长度缓冲区，以在 `required_size` 中获取所需的字节大小。
2. **数据获取** — 分配所需大小的缓冲区，第二次调用将其填充为可变长度 `SYSTEM_CPU_SET_INFORMATION` 结构数组。

然后使用每个条目的 `Size` 字段逐条目遍历原始缓冲区以推进偏移量。每个条目被传递给模块私有辅助函数 `extract_cpu_set_data`，该函数从联合体中读取 `CpuSet.Id` 和 `CpuSet.LogicalProcessorIndex` 字段并构造一个 [CpuSetData](CpuSetData.md) 值。

### 失败处理

如果第二次调用 `GetSystemCpuSetInformation` 失败，错误将记录到 find 日志中，结果向量为空。所有下游转换函数将返回空结果，实际上是在不使服务崩溃的情况下禁用了 CPU 集合操作。

### 线程安全

该向量受 `std::sync::Mutex` 保护。所有访问器在迭代前都会锁定互斥锁。由于数据在初始化后是只读的，因此争用极小——锁的存在主要是为了满足 Rust 对全局可变静态变量的 `Send`/`Sync` 要求。

### 拓扑假设

- CPU 集合数据在首次访问时捕获一次，之后不会刷新。如果系统拓扑在运行时发生变化（例如 CPU 热添加），缓存将变得过时。这是可以接受的，因为在 AffinityServiceRust 运行的桌面 Windows 系统上，CPU 热插拔极为罕见。
- `LogicalProcessorIndex` 字段存储为 `u8`，最多支持 256 个逻辑处理器。具有跨多个处理器组超过 256 个处理器的系统可能需要进行结构性更改。

### 访问器

公共函数 [get_cpu_set_information](get_cpu_set_information.md) 返回 `&'static Mutex<Vec<CpuSetData>>`，提供对缓存的共享访问而不暴露 `Lazy` 包装器。

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **Crate 依赖** | `once_cell::sync::Lazy`, `std::sync::Mutex` |
| **填充方** | `GetSystemCpuSetInformation` (Win32), `extract_cpu_set_data` (模块私有辅助函数) |
| **访问方式** | [get_cpu_set_information](get_cpu_set_information.md) |
| **使用方** | [cpusetids_from_indices](cpusetids_from_indices.md), [cpusetids_from_mask](cpusetids_from_mask.md), [indices_from_cpusetids](indices_from_cpusetids.md), [mask_from_cpusetids](mask_from_cpusetids.md) |
| **API** | [`GetSystemCpuSetInformation`](https://learn.microsoft.com/en-us/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) |
| **特权** | 无 — `GetSystemCpuSetInformation` 不需要提权 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 元素类型 | [CpuSetData](CpuSetData.md) |
| 公共访问器函数 | [get_cpu_set_information](get_cpu_set_information.md) |
| CPU 索引 → CPU 集合 ID | [cpusetids_from_indices](cpusetids_from_indices.md) |
| 亲和性掩码 → CPU 集合 ID | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU 集合 ID → CPU 索引 | [indices_from_cpusetids](indices_from_cpusetids.md) |
| CPU 集合 ID → 亲和性掩码 | [mask_from_cpusetids](mask_from_cpusetids.md) |
| 对进程应用 CPU 集合 | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md) |
| GetSystemCpuSetInformation (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) |