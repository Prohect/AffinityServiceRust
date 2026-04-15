# parse_cpu_spec 函数 (config.rs)

将 CPU 规格字符串解析为已排序的 CPU 索引列表。此函数是解释 AffinityServiceRust 配置文件所支持的各种 CPU 规格格式的主要入口点，包括数字范围、分号分隔的单独索引和旧式十六进制位掩码。

## 语法

```AffinityServiceRust/src/config.rs#L78-113
pub fn parse_cpu_spec(s: &str) -> List<[u32; CONSUMER_CPUS]>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `s` | `&str` | 以下述支持格式之一表示的 CPU 规格字符串。解析前会去除前导和尾随空白。 |

## 返回值

类型：`List<[u32; CONSUMER_CPUS]>`

已排序、已去重的 `u32` CPU 索引列表。当输入为空、`"0"` 或无法解析时，返回空列表。

## 备注

### 支持的格式

| 格式 | 示例 | 结果 | 说明 |
|------|------|------|------|
| 空或 `"0"` | `""`、`"0"` | `[]`（空） | 表示"不更改"——当前 CPU 分配保持不变。 |
| 十六进制位掩码 | `"0xFF"`、`"0X0F"` | `[0,1,2,3,4,5,6,7]`、`[0,1,2,3]` | 适用于 ≤ 64 个逻辑处理器的旧式格式。必须以 `0x` 或 `0X` 开头。通过 `u64::from_str_radix` 解析。 |
| CPU 范围 | `"0-7"` | `[0,1,2,3,4,5,6,7]` | 包含范围。推荐的现代配置格式。 |
| 单独 CPU | `"0;4;8"` | `[0,4,8]` | 以分号分隔的单独 CPU 索引列表。 |
| 混合范围 | `"0-3;8-11"` | `[0,1,2,3,8,9,10,11]` | 以分号分隔的多个范围和/或单独索引。支持超过 64 核的系统。 |
| 单个 CPU | `"7"` | `[7]` | 解释为 CPU 索引 7，**不是**位掩码。使用 `"0x7"` 或 `"0-2"` 表示核心 0–2。 |

### 算法

1. 对输入字符串去除空白。
2. 如果结果为空或正好是 `"0"`，立即返回空列表。
3. 如果字符串以 `"0x"` 或 `"0X"` 开头，将其视为十六进制位掩码：
   - 将十六进制部分解析为 `u64`。
   - 通过 [`mask_to_cpu_indices`](mask_to_cpu_indices.md) 将被设置的位位置（`1`）转换为 CPU 索引。
   - 如果十六进制解析失败，返回空列表。
4. 否则，按 `';'` 拆分字符串，逐段处理：
   - 跳过空段（来自尾随或连续分号）。
   - 如果段包含 `'-'`，将其解析为包含范围 `start-end`。解析失败时两个边界都默认为 `0`；起始值失败时终止值默认为起始值。
   - 如果段是纯整数，作为单个 CPU 索引添加。
   - 在插入过程中抑制重复的 CPU 索引。
5. 在返回之前将结果列表按升序排序。

### 重要的歧义消除

值 `"7"` 表示 **CPU 索引 7**（单个逻辑处理器），而不是 CPU 0–2 的位掩码。这是一个有意的设计选择，以避免位掩码和索引解释之间的歧义。要指定 CPU 0、1 和 2，请使用十六进制表示法 `"0x7"` 或范围表示法 `"0-2"`。

### 边界情况

- `0x`/`0X` 前缀之后的无效十六进制字符串返回空列表，不产生错误。
- 非数字的范围边界（例如 `"a-z"`）通过 `unwrap_or(0)` 回退到 `0`。
- 规格中的重复索引会被静默去重。
- 该函数不验证 CPU 索引是否对应系统上物理存在的处理器；该验证在应用时进行。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 可见性 | `pub` |
| 调用方 | [`resolve_cpu_spec`](resolve_cpu_spec.md)、[`parse_alias`](parse_alias.md)、[`parse_mask`](parse_mask.md)、[`read_config`](read_config.md)（间接） |
| 被调用方 | [`mask_to_cpu_indices`](mask_to_cpu_indices.md)（用于十六进制位掩码输入） |
| API | [`collections.rs`](../collections.rs/README.md) 中的 `List` 和 `CONSUMER_CPUS` |
| 权限 | 无 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| mask_to_cpu_indices | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| cpu_indices_to_mask | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| format_cpu_indices | [format_cpu_indices](format_cpu_indices.md) |
| resolve_cpu_spec | [resolve_cpu_spec](resolve_cpu_spec.md) |
| parse_mask | [parse_mask](parse_mask.md) |
| config 模块概述 | [README](README.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*