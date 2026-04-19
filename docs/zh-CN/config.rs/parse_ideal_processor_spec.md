# parse_ideal_processor_spec 函数 (config.rs)

解析理想处理器规格字符串，生成 [IdealProcessorRule](IdealProcessorRule.md) 条目列表。每个规则将一组 CPU 索引（从别名解析）映射到线程起始模块前缀列表，实现在单个进程内按模块分配理想处理器。

## 语法

```rust
fn parse_ideal_processor_spec(
    spec: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    errors: &mut Vec<String>,
) -> Vec<IdealProcessorRule>
```

## 参数

`spec: &str`

配置规则行字段 7 中的理想处理器规格字符串。格式由一个或多个段组成，每个段以 `*` 开头，后跟 CPU 别名名称，以及可选的 `@` 分隔符和分号分隔的模块前缀。示例：

- `"0"` — 无理想处理器规则（返回空向量）。
- `"*pcore"` — 将所有线程分配给别名 `pcore` 的 CPU。
- `"*pcore@engine.dll;render.dll"` — 仅将来自 `engine.dll` 或 `render.dll` 的线程分配给 `pcore` CPU。
- `"*pcore@engine.dll*ecore@audio.dll"` — 两条规则：引擎线程到 P 核，音频线程到 E 核。

`line_number: usize`

规格出现的配置文件中的基于 1 的行号。包含在错误消息中，用于用户诊断。

`cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>`

当前定义的 CPU 别名映射，由早期的 [parse_alias](parse_alias.md) 调用填充。键是小写别名名称（不带 `*` 前缀）。

`errors: &mut Vec<String>`

**\[out\]** 错误消息累加器。当规格不以 `*` 开头、别名名称为空或别名未在 `cpu_aliases` 中找到时，追加错误。

## 返回值

`Vec<IdealProcessorRule>` — [IdealProcessorRule](IdealProcessorRule.md) 条目列表，每个条目包含 CPU 索引列表和前缀过滤器列表。如果规格为 `"0"`、空或完全无效，则返回空向量。

## 备注

### 解析算法

1. **修剪和提前退出：** 修剪规格。如果为空或等于 `"0"`，则立即返回空向量。
2. **前缀验证：** 如果规格不以 `*` 开头，则推送错误并返回空向量。
3. **段分割：** 在 `*` 处分割规格（第一个空段来自前导 `*` 被跳过）。每个非空段代表一条规则。
4. **别名和前缀提取：** 在每个段内：
   - 如果存在 `@`，则 `@` 之前部分是别名名称，`@` 之后部分是分号分隔的模块前缀列表。
   - 如果不存在 `@`，则整个段是别名名称，前缀列表为空（匹配所有线程）。
5. **别名解析：** 别名名称转为小写并在 `cpu_aliases` 中查找。如果别名不存在，推送错误并跳过该段。
6. **空 CPU 检查：** 如果解析的 CPU 列表为空（别名映射到无 CPU），则完全跳过该段——不生成任何规则。
7. **前缀规范化：** 每个前缀字符串被修剪、转为小写，空字符串被过滤掉。
8. **规则构建：** 创建 [IdealProcessorRule](IdealProcessorRule.md) 条目，带有解析后的 CPU 和前缀列表，然后推送到结果向量。

### 与 Prime 线程前缀的关系

理想处理器规格语法与 [parse_and_insert_rules](parse_and_insert_rules.md) 中字段 4 解析的 Prime 线程前缀语法相似——但独立。两者都使用 `*alias@prefix` 模式，但它们服务于不同的目的：

| 功能 | Prime 线程（字段 4） | 理想处理器（字段 7） |
|---------|------------------------|---------------------------|
| **目的** | 通过 CPU 集合将高活动性线程固定到专用 CPU | 为所有匹配线程设置理想处理器提示 |
| **强制执行** | 硬（CPU 集合限制） | 软（调度器提示） |
| **需要跟踪** | 是 (`track_top_x_threads`) | 否 |
| **每前缀优先级提升** | 是 (`!priority` 后缀) | 否 |
| **生成的结构** | [PrimePrefix](PrimePrefix.md) | [IdealProcessorRule](IdealProcessorRule.md) |

### 字段位置歧义

规则格式中的字段 7 可以包含理想处理器规格（以 `*` 开头）或等级数字。调用方 [parse_and_insert_rules](parse_and_insert_rules.md) 进行消歧：如果字段以 `*` 开头或等于 `"0"`，则视为理想处理器规格，等级从字段 8 读取（默认为 1）。如果字段解析为普通整数，则视为等级，不创建理想处理器规则。

### 错误报告

此函数的错误被附加到 `errors` 向量，最终出现在 [ConfigResult](ConfigResult.md) 的错误列表中。以下条件会产生错误：

- **缺少 `*` 前缀：** `"Line {N}: Ideal processor spec must start with '*', got '{spec}'"` — 整个规格被拒绝。
- **空别名名称：** `"Line {N}: Empty alias in ideal processor rule '*{segment}'"` — 单个段被跳过；其他段仍可能成功。
- **未知别名：** `"Line {N}: Unknown CPU alias '*{alias}' in ideal processor specification"` — 段被跳过。

### 配置语法示例

```
*pcore = 0-7
*ecore = 8-19

# 所有线程在 P 核上获得理想处理器提示
game.exe:normal:0:0:0:none:none:*pcore:1

# 按模块的理想处理器提示
game2.exe:normal:0:0:0:none:none:*pcore@engine.dll;render.dll*ecore@audio.dll:1
```

在第二个示例中，创建两个 [IdealProcessorRule](IdealProcessorRule.md) 条目：

1. `cpus: [0, 1, 2, 3, 4, 5, 6, 7], prefixes: ["engine.dll", "render.dll"]`
2. `cpus: [8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19], prefixes: ["audio.dll"]`

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **可见性** | 私有 (`fn`) — config 模块内部 |
| **调用方** | [parse_and_insert_rules](parse_and_insert_rules.md)（规则行的字段 7） |
| **被调用方** | CPU 别名映射查找（无函数调用；内联别名解析） |
| **依赖项** | [IdealProcessorRule](IdealProcessorRule.md), [HashMap](../collections.rs/README.md), [List](../collections.rs/README.md) |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [config.rs](README.md) |
| 理想处理器规则结构体 | [IdealProcessorRule](IdealProcessorRule.md) |
| 规则插入函数 | [parse_and_insert_rules](parse_and_insert_rules.md) |
| CPU 别名定义 | [parse_alias](parse_alias.md) |
| 运行时理想处理器应用 | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| Prime 线程前缀（相关概念） | [PrimePrefix](PrimePrefix.md) |
| 其他字段的别名解析 | [resolve_cpu_spec](resolve_cpu_spec.md) |

*文档针对 Commit：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*