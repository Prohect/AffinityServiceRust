# CPU_SET_INFORMATION 静态变量 (winapi.rs)

延迟初始化、互斥锁保护的系统 CPU 集合数据缓存。首次访问时，此静态变量通过调用 Windows `GetSystemCpuSetInformation` API 枚举系统上的所有 CPU 集合，并将结果存储为 `Vec<CpuSetData>`。后续访问将返回缓存数据，无需重新查询操作系统。

## 语法

```rust
static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>> = Lazy::new(|| { ... });
```

## 类型

`once_cell::sync::Lazy<std::sync::Mutex<Vec<CpuSetData>>>`

## 备注

### 初始化

`Lazy` 初始化器执行以下步骤：

1. 使用零长度缓冲区调用 `GetSystemCpuSetInformation`，以确定所需的缓冲区大小（`required_size`）。
2. 分配一个所需大小的 `Vec<u8>`。
3. 使用已分配的缓冲区再次调用 `GetSystemCpuSetInformation`，以检索所有 CPU 集合条目。
4. 遍历缓冲区，通过 unsafe 辅助函数 `extract_cpu_set_data` 解析每个 `SYSTEM_CPU_SET_INFORMATION` 结构体，该函数读取 `CpuSet.Id` 和 `CpuSet.LogicalProcessorIndex` 联合体字段。
5. 将每个提取的 [`CpuSetData`](CpuSetData.md) 条目推入结果向量。

如果第二次调用 `GetSystemCpuSetInformation` 失败，将通过 `log_to_find` 写入诊断消息（`"GetSystemCpuSetInformation failed"`），并返回空向量。

### 线程安全

此静态变量包装在 `Mutex` 中，以允许多线程安全并发访问。实际上，初始填充后内部数据不会再改变——CPU 集合拓扑在操作系统启动后的整个生命周期内是固定的——但 `Mutex` 是必需的，因为 `Lazy` 初始化必须同步，且调用者可能从不同线程访问。

### 生命周期

该静态变量具有 `'static` 生命周期，永远不会被释放。其包含的数据在进程的整个运行期间保持有效。运行时 CPU 拓扑的变化（例如热添加处理器）**不会**被反映。

### 访问模式

不应在 `winapi` 模块外部直接访问此静态变量。请使用 [`get_cpu_set_information`](get_cpu_set_information.md) 访问器函数，它返回一个 `&'static Mutex<Vec<CpuSetData>>` 引用。以下函数在内部锁定并读取此静态变量：

- [`cpusetids_from_indices`](cpusetids_from_indices.md)
- [`cpusetids_from_mask`](cpusetids_from_mask.md)
- [`indices_from_cpusetids`](indices_from_cpusetids.md)
- [`mask_from_cpusetids`](mask_from_cpusetids.md)

### 缓冲区解析

`GetSystemCpuSetInformation` 返回的原始缓冲区包含可变大小的条目。初始化器使用每个条目的 `Size` 字段来推进偏移量，确保无论操作系统版本或未来结构体扩展如何，都能正确解析。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `winapi.rs` |
| **可见性** | 私有（模块内部） |
| **访问器** | [`get_cpu_set_information`](get_cpu_set_information.md) |
| **Win32 API** | [`GetSystemCpuSetInformation`](https://learn.microsoft.com/en-us/windows/win32/api/systeminfomationapi/nf-systeminfomationapi-getsystemcpusetinformation) |
| **依赖** | `once_cell::sync::Lazy`、`std::sync::Mutex`、[`CpuSetData`](CpuSetData.md) |
| **平台** | 仅限 Windows |
| **权限** | 无需特殊权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| CpuSetData 结构体 | [CpuSetData](CpuSetData.md) |
| get_cpu_set_information 访问器 | [get_cpu_set_information](get_cpu_set_information.md) |
| cpusetids_from_indices | [cpusetids_from_indices](cpusetids_from_indices.md) |
| cpusetids_from_mask | [cpusetids_from_mask](cpusetids_from_mask.md) |
| indices_from_cpusetids | [indices_from_cpusetids](indices_from_cpusetids.md) |
| mask_from_cpusetids | [mask_from_cpusetids](mask_from_cpusetids.md) |
| MODULE_CACHE 静态变量 | [MODULE_CACHE](MODULE_CACHE.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
