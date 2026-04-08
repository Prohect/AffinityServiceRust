# Config 模块文档

配置文件解析、CPU 规格处理和配置操作工具。

## 概述

本模块提供：
- INI 风格的配置文件解析
- CPU 规格解析（范围、掩码、别名）
- 进程分组处理
- 从 Process Lasso 格式转换配置
- 相同规则的自动分组

## 调用者

- [main.rs](main.md) - 通过 `read_config()` 主配置加载
- CLI `-convert` 模式 - Process Lasso 转换
- CLI `-autogroup` 模式 - 规则分组优化

## 数据结构

### ProcessConfig

单个进程的完整配置。

```rust
pub struct ProcessConfig {
    pub name: String,                           // 进程可执行文件名
    pub priority: ProcessPriority,              // 进程优先级类
    pub affinity_cpus: Vec<u32>,                // 硬亲和性 CPU 列表
    pub cpu_set_cpus: Vec<u32>,                 // CPU 集 CPU 列表
    pub cpu_set_reset_ideal: bool,              // CPU 集更改后重置理想处理器
    pub prime_threads_cpus: Vec<u32>,           // Prime 调度 CPU
    pub prime_threads_prefixes: Vec<PrimePrefix>, // 模块特定规则
    pub track_top_x_threads: i32,               // 跟踪前 N 线程（0=关, >0=跟踪, <0=仅跟踪）
    pub io_priority: IOPriority,                // I/O 优先级
    pub memory_priority: MemoryPriority,        // 内存优先级
    pub ideal_processor_rules: Vec<IdealProcessorRule>, // 理想处理器分配
}
```

**类型引用：**
- `priority`: [`ProcessPriority`](priority.md#processpriority)
- `io_priority`: [`IOPriority`](priority.md#iopriority)
- `memory_priority`: [`MemoryPriority`](priority.md#memorypriority)
- `prime_threads_prefixes`: [`PrimePrefix`](#primeprefix)
- `ideal_processor_rules`: [`IdealProcessorRule`](#idealprocessorrule)

### PrimePrefix

Prime 线程调度的模块特定前缀规则。

```rust
pub struct PrimePrefix {
    pub prefix: String,              // 要匹配的模块名前缀
    pub cpus: Option<Vec<u32>>,      // 此前缀的特定 CPU（None = 使用 prime_threads_cpus）
    pub thread_priority: ThreadPriority, // 要应用的线程优先级
}
```

用于 [`ProcessConfig::prime_threads_prefixes`](#processconfig) 进行每模块 CPU 和优先级分配。

**类型引用：**
- `thread_priority`: [`ThreadPriority`](priority.md#threadpriority)

### IdealProcessorRule

基于模块前缀为线程分配理想处理器的规则。

```rust
pub struct IdealProcessorRule {
    pub cpus: Vec<u32>,              // 理想处理器分配的 CPU 索引
    pub prefixes: Vec<String>,       // 要匹配的模块名前缀（空 = 所有线程）
}
```

用于 [`ProcessConfig::ideal_processor_rules`](#processconfig)。当 `prefixes` 为空时，规则适用于所有线程。

### IdealProcessorPrefix

带特定 CPU 的模块前缀（解析的内部辅助）。

```rust
pub struct IdealProcessorPrefix {
    pub prefix: String,              // 模块名前缀
    pub cpus: Vec<u32>,              // 此前缀的 CPU 索引
}
```

### ConfigConstants

调度器行为常量。

```rust
pub struct ConfigConstants {
    pub min_active_streak: u8,      // 提升前连续间隔（默认：2）
    pub keep_threshold: f64,        // 保持 prime 的分数（默认：0.69）
    pub entry_threshold: f64,       // 成为候选的分数（默认：0.42）
}
```

### ConfigResult

解析结果包含统计信息和错误。

```rust
pub struct ConfigResult {
    pub configs: HashMap<u32, HashMap<String, ProcessConfig>>, // grade -> (name -> config)
    pub constants: ConfigConstants,
    pub constants_count: usize,
    pub aliases_count: usize,
    pub groups_count: usize,
    pub group_members_count: usize,
    pub process_rules_count: usize,
    pub redundant_rules_count: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}
```

## CPU 规格解析

### parse_cpu_spec

将 CPU 规格字符串解析为排序的 CPU 索引向量。

**支持的格式：**
- `"0"` 或空 → 空向量（无更改）
- `"0xFF"` → 十六进制位掩码（旧版，≤64 核）
- `"0-7"` → CPU 范围（含）
- `"0;4;8"` → 分号分隔的单个 CPU
- `"0-7;64-71"` → >64 核系统的多范围

**示例：**
```rust
parse_cpu_spec("0-3")     // → [0, 1, 2, 3]
parse_cpu_spec("0;2;4")   // → [0, 2, 4]
parse_cpu_spec("0x0F")    // → [0, 1, 2, 3]
parse_cpu_spec("")        // → []
parse_cpu_spec("0")       // → []
```

**被调用者：**
- [`read_config()`](config.md#config-file-format) - 主配置加载函数
- `resolve_cpu_spec()`（内部）- 别名解析
- [`convert()`](config.md#process-lasso-conversion) - Process Lasso 转换

### cpu_indices_to_mask

将 CPU 索引转换为位掩码（用于 ≤64 核系统）。

```rust
pub fn cpu_indices_to_mask(cpus: &[u32]) -> usize
```

**示例：**
```rust
cpu_indices_to_mask(&[0, 1, 2, 3])  // → 0x0F
```

### format_cpu_indices

将 CPU 索引格式化为紧凑字符串（尽可能使用范围）。

```rust
pub fn format_cpu_indices(cpus: &[u32]) -> String
```

**示例：**
```rust
format_cpu_indices(&[0, 1, 2, 3])   // → "0-3"
format_cpu_indices(&[0, 2, 4])      // → "0,2,4"
format_cpu_indices(&[])             // → "0"
```

## 理想处理器解析

### parse_ideal_processor_spec

解析带模块前缀过滤的理想处理器规格。

**格式：**`*alias[@prefix1;prefix2]`

- `*` 是每条规则段的必需前缀标记
- `alias` 是 CPU 别名（必须在 ALIAS 部分定义）
- `@prefix` 可选地按线程起始模块名过滤

**多段：**`*p@engine.dll*e@helper.dll`

**示例：**
```rust
// 简单
parse_ideal_processor_spec("*pN01", line_num, &aliases, &mut errors)
// → [IdealProcessorRule { cpus: [2,3,4,5,6,7], prefixes: [] }]

// 带模块过滤
parse_ideal_processor_spec("*pN01@cs2.exe;nvwgf2umx.dll", ...)
// → [IdealProcessorRule { cpus: [2,3,4,5,6,7], prefixes: ["cs2.exe", "nvwgf2umx.dll"] }]
```

**被调用者：**`parse_and_insert_rules()`（内部）- 解析规则字段 6 或 7 时

## Prime 线程规格解析

`prime_cpus` 字段支持高级语法：

```
[?[?]x]*alias[@module1[!priority];module2[!priority]*alias2@module3...]
```

**组成部分：**
- `?x*cpus` - 跟踪前 x 线程，应用规则，退出时记录
- `??x*cpus` - 仅监控：跟踪并退出时记录，不应用 CPU 集
- `*alias@module1;module2` - 仅影响来自指定模块的线程
- `*alias1@mod1*alias2@mod2` - 多段：每模块不同 CPU
- `module!priority` - 设置显式线程优先级
- `module` - 自动提升（当前优先级 + 1 级）

**示例：**
```ini
# 在 P 核（除 0-1）上跟踪前 10 线程
game.exe:normal:*a:*p:?10*pN01:normal:normal:1

# 多段：CS2 在 P 核，NVIDIA 在 E 核
cs2.exe:normal:*a:*p:*p@cs2.exe*e@nvwgf2umx.dll:normal:normal:1

# 每模块线程优先级
cs2.exe:normal:*a:*p:*pN01@cs2.exe!time critical;nvwgf2umx.dll!above normal:normal:normal:1

# 仅监控（不应用）：跟踪前 20 线程
game.exe:normal:*a:*p:??20*pN01:normal:normal:1
```

## 配置文件格式

### 常量部分

```ini
@MIN_ACTIVE_STREAK = 2    // 提升前连续间隔
@ENTRY_THRESHOLD = 0.42   // 成为候选的最大周期分数
@KEEP_THRESHOLD = 0.69    // 保持 prime 的最大周期分数
```

### 别名部分

```ini
*a = 0-19           // 所有核心
*p = 0-7            // P 核
*e = 8-19           // E 核
*pN01 = 2-7         // 除 0-1 外的 P 核
```

别名支持所有 CPU 规格格式，包括 >64 核系统的多范围。

### 规则部分

**基本规则：**
```ini
process.exe:priority:affinity:cpuset:prime:io:memory:ideal:grade
```

**CPU 集重置理想处理器：**

在 cpuset 字段前加 `@` 前缀以启用应用 CPU 集后重置线程理想处理器：
```ini
# 将 CPU 集设置为 0-3 后，在 CPU 0-3 上重新分配线程理想处理器
game.exe:normal:*a:@0-3:*p:normal:normal:1
```

这可以防止 Windows 在 CPU 集更改后将线程限制到狭窄的 CPU 范围。

**示例：**
```ini
# 简单规则（grade 默认为 1）
cs2.exe:normal:*a:*p:*pN01:normal:normal

# 带理想处理器和显式 grade
background.exe:normal:*a:*p:*p:normal:normal:*p:5

# CPU 集带理想处理器重置
game.exe:normal:*a:@0-3:*p:normal:normal:1

# 进程组
browsers { chrome.exe: firefox.exe: msedge.exe }:normal:*e:0:0:low:below normal:1
```

## Process Lasso 转换

`convert()` 函数将 Process Lasso INI 格式转换为 AffinityServiceRust 格式。

**解析的字段：**
- `NamedAffinities=alias,cpus,alias,cpus...` → `*alias = cpus`
- `DefaultPriorities=process,priority,process,priority...` → priority 字段
- `DefaultAffinitiesEx=process,mask,cpuset,process,mask,cpuset...` → affinity/cpuset 字段

**被调用者：**CLI `-convert` 模式

**示例：**
```bash
AffinityServiceRust.exe -convert -in prolasso.ini -out my_config.ini
```

## 自动分组

`sort_and_group_config()` 函数将设置相同的规则合并到命名组中。

**算法：**
1. 保留前言（注释、常量、别名）解析现有配置
2. 收集所有规则并按相同规则字符串分组
3. 为多进程规则创建命名组（`grp_0`, `grp_1` 等）
4. 如果少于 128 字符则单行格式化组，否则多行

**输出格式：**
```ini
# 输入：
explorer.exe:none:*a:*e:0:none:none:0:4
cmd.exe:none:*a:*e:0:none:none:0:4
notepad.exe:none:*a:*e:0:none:none:0:4

# 输出：
grp_0 { cmd.exe: explorer.exe: notepad.exe }:none:*a:*e:0:none:none:0:4
```

**被调用者：**CLI `-autogroup` 模式

**示例：**
```bash
AffinityServiceRust.exe -autogroup -in config.ini -out config_grouped.ini
```

## 错误处理

配置解析收集错误和警告而不立即失败：

- **错误：**无效语法、未定义别名、未闭合的组
- **警告：**未知优先级、空别名、冗余规则

使用 `ConfigResult::print_report()` 显示解析结果。

## 辅助函数

### read_list

读取简单列表文件（黑名单等）。

```rust
pub fn read_list<P: AsRef<Path>>(path: P) -> Result<Vec<String>>
```

过滤：空行、注释（以 `#` 开头的行）

### read_utf16le_file

读取 UTF-16 LE 编码文件（Process Lasso 配置）。

```rust
pub fn read_utf16le_file(path: &str) -> Result<String>
```

## 依赖

- `crate::cli::get_config_help_lines` - 用于转换输出模板
- `crate::log` - 日志宏
- `crate::logging` - 错误日志工具
- `crate::priority` - 优先级枚举定义
- `std::collections` - HashMap 用于配置和别名
- `std::fs` - 文件 I/O
