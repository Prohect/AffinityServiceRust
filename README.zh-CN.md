# AffinityServiceRust

<!-- languages -->
- 🇺🇸 [English](https://github.com/Prohect/AffinityServiceRust/blob/master/README.md)
- 🇨🇳 [中文 (简体)](https://github.com/Prohect/AffinityServiceRust/blob/master/README.zh-CN.md)

基于 Rust 编写的高性能 Windows 进程管理服务，根据配置文件自动为进程应用 CPU 亲和性、优先级、I/O 优先级和内存优先级规则。

## 概述

AffinityServiceRust 持续监控运行中的进程，并根据配置文件中定义的规则应用定制化的调度策略。它支持：

- **进程优先级管理**：设置进程优先级类（从空闲到实时）—— 参见 [优先级级别](#优先级级别)
- **CPU 亲和性**：将进程硬性绑定到特定的逻辑处理器（旧版 ≤64 核系统）—— 参见 [`apply_affinity()`](docs/zh-CN/apply.rs/apply_affinity.md)
- **CPU 集**：跨所有处理器组的软性 CPU 偏好（现代 >64 核系统）—— 参见 [`apply_process_default_cpuset()`](docs/zh-CN/apply.rs/apply_process_default_cpuset.md)
- **Prime 线程调度**：动态识别并分配 CPU 密集型线程到指定的"prime"核心 —— 参见下方的 [Prime 线程调度](#prime-线程调度) 章节
- **理想处理器分配**：为最繁忙的 N 个线程静态分配线程到 CPU —— 参见下方的 [理想处理器分配](#理想处理器分配) 章节
- **I/O 优先级控制**：控制磁盘 I/O 调度优先级 —— 参见 [`apply_io_priority()`](docs/zh-CN/apply.rs/apply_io_priority.md)
- **内存优先级控制**：调整进程工作集的内存页面优先级 —— 参见 [`apply_memory_priority()`](docs/zh-CN/apply.rs/apply_memory_priority.md)
- **热重载**：自动检测并应用配置文件更改
- **规则等级**：控制每个进程规则的应用频率 —— 参见 [规则等级](#规则等级)

## 文档

| 主题 | 文档 |
|------|------|
| **架构** | [docs/zh-CN/main.rs/README.md](docs/zh-CN/main.rs/README.md) - 主循环和入口点 |
| **配置** | [docs/zh-CN/config.rs/README.md](docs/zh-CN/config.rs/README.md) - 配置解析和 CPU 规格 |
| **应用逻辑** | [docs/zh-CN/apply.rs/README.md](docs/zh-CN/apply.rs/README.md) - 如何将设置应用到进程 |
| **调度器** | [docs/zh-CN/scheduler.rs/README.md](docs/zh-CN/scheduler.rs/README.md) - Prime 线程调度器实现 |
| **CLI 选项** | [docs/zh-CN/cli.rs/README.md](docs/zh-CN/cli.rs/README.md) - 命令行参数 |
| **优先级级别** | [docs/zh-CN/priority.rs/README.md](docs/zh-CN/priority.rs/README.md) - 优先级枚举定义 |
| **Windows API** | [docs/zh-CN/winapi.rs/README.md](docs/zh-CN/winapi.rs/README.md) - Windows API 包装器 |
| **日志** | [docs/zh-CN/logging.rs/README.md](docs/zh-CN/logging.rs/README.md) - 错误跟踪和日志 |
| **ETW 监控** | [docs/zh-CN/event_trace.rs/README.md](docs/zh-CN/event_trace.rs/README.md) - 基于 ETW 的响应式进程监控 |

## 快速开始

1. 编译或下载发行版二进制文件
2. 下载 `config.ini` 和 `blacklist.ini` 到工作目录
3. 编辑 `config.ini` 以匹配你的 CPU 拓扑（参见[配置](#配置)部分）
4. 使用适当的权限运行：

```bash
# 基本用法，带控制台输出
AffinityServiceRust.exe -config my_config.ini -console

# 以管理员身份运行（推荐用于完整功能）
powershell -Command "Start-Process -FilePath './AffinityServiceRust.exe' -Verb RunAs -Wait"

# 显示所有可用选项
AffinityServiceRust.exe -helpall
```

## 功能特性

| 功能 | 说明 |
|------|------|
| **进程优先级** | 设置优先级类（空闲、低于标准、标准、高于标准、高、实时） |
| **CPU 亲和性** | 旧版基于掩码的亲和性（≤64 核，[`SetProcessAffinityMask`](docs/zh-CN/apply.rs/apply_affinity.md)） |
| **CPU 集** | 通过 Windows CPU Sets 软性偏好核心（支持 >64 核，[`SetProcessDefaultCpuSets`](docs/zh-CN/apply.rs/apply_process_default_cpuset.md)） |
| **Prime 线程调度** | 动态分配 CPU 密集型线程到性能核心 |
| **理想处理器（首核）分配** | 基于滞后（hysteresis）算法的理想处理器分配，使用与 Prime 线程调度相同的算法和常量（[`MIN_ACTIVE_STREAK`](docs/zh-CN/config.rs/ConfigConstants.md)、[`ENTRY_THRESHOLD`](docs/zh-CN/config.rs/ConfigConstants.md)、[`KEEP_THRESHOLD`](docs/zh-CN/config.rs/ConfigConstants.md)） |
| **I/O 优先级** | 控制 I/O 优先级（极低、低、标准、高 - 需要管理员） |
| **内存优先级** | 极低、低、中、低于标准、标准 |
| **计时器分辨率** | 调整 Windows 系统计时器分辨率 |
| **热重载** | 配置文件变更时自动重新加载 |
| **ETW 进程监控** | 通过 Windows 事件跟踪实时检测进程启动/停止 |
| **规则等级** | 控制规则应用频率（每 N 次循环） |

> **关于 >64 核系统的说明：** CPU 亲和性（[`SetProcessAffinityMask`](docs/zh-CN/apply.rs/apply_affinity.md)）只能在单个处理器组内工作（≤64 核）。对于 >64 核系统，请使用 CPU 集，它可以跨所有处理器组工作，但为软性偏好。

### Prime 线程调度

Prime 线程调度器使用基于滞后（hysteresis）的算法，动态识别最 CPU 密集的线程并将其分配到指定的"prime"核心：

**算法：**
- 在可配置的时间间隔通过 [`prefetch_all_thread_cycles()`](docs/zh-CN/apply.rs/prefetch_all_thread_cycles.md) 监控线程 CPU 周期消耗
- 应用滞后机制防止频繁切换：
  - **入场阈值**：线程必须超过最大周期的配置百分比（默认 42%）才能成为候选
  - **保持阈值**：一旦提升，线程保持在配置百分比以上（默认 69%）则保持 prime 状态
  - **活跃连击**：提升前需要连续活跃间隔（默认 2 个）
- 自动过滤低活跃线程
- 支持多段式 CPU 分配：不同模块可以使用不同的核心集
- 按模块线程优先级控制（显式或自动提升）
- 线程跟踪模式：进程退出时记录详细统计信息

详情请参见 [`apply_prime_threads()`](docs/zh-CN/apply.rs/apply_prime_threads.md) 和 [scheduler 模块](docs/zh-CN/scheduler.rs/README.md)。

**线程跟踪输出：**
当跟踪的进程退出时，为前 N 个线程记录详细统计信息：
- 线程 ID 和总 CPU 周期消耗
- 起始地址解析为 `module.dll+offset` 格式
- 内核时间和用户时间
- 线程优先级、基础优先级、上下文切换
- 线程状态和等待原因

### 理想处理器分配

可选的 `ideal` 规范可为最活跃的线程分配首选处理器，使用与 Prime 线程调度**相同的滞后过滤器**。

**算法：**
- 每次迭代的周期数据由共享的 [`prefetch_all_thread_cycles()`](docs/zh-CN/apply.rs/prefetch_all_thread_cycles.md) 预取通道提供（每线程一次 `QueryThreadCycleTime` 调用，数量上限为逻辑 CPU 数）
- 应用 [`MIN_ACTIVE_STREAK`](docs/zh-CN/config.rs/ConfigConstants.md)、[`ENTRY_THRESHOLD`](docs/zh-CN/config.rs/ConfigConstants.md)、[`KEEP_THRESHOLD`](docs/zh-CN/config.rs/ConfigConstants.md) 常量，与 Prime 线程调度完全相同：
  - **第 1 轮（保留）**：已分配且仍高于 `KEEP_THRESHOLD` 的线程保留其槽位，**无需写系统调用**
  - **第 2 轮（晋升）**：高于 `ENTRY_THRESHOLD` 且连击计数满足 `MIN_ACTIVE_STREAK` 的新候选线程获得槽位
- 延迟设置：若新选线程的当前理想处理器已在空闲 CPU 池中，则跳过 `SetThreadIdealProcessorEx` 调用——就地认领该槽位
- 降级：不再被选中的线程恢复其原始理想处理器
- 每条分配日志包含 `start=模块+偏移`（如 `start=cs2.exe+0xEA60`）
- 多规则语法允许不同模块前缀使用不同的 CPU 集

详情请参见 [`apply_ideal_processors()`](docs/zh-CN/apply.rs/apply_ideal_processors.md)。

### 理想处理器重置

当进程的 CPU 亲和性（affinity）更改后，AffinityServiceRust 会自动重设该进程内各线程的理想处理器分配，以避免 Windows 内核在亲和性更新后将过多线程截断到相同物理 CPU 核心的问题。

也可以通过在 cpuset 字段前添加 `@` 前缀来为 CPU 集更改启用此功能：

```ini
# 设置 CPU 集为 0-3 后，将线程理想处理器重新分配到 CPU 0-3
game.exe:normal:*a:@0-3:*p:normal:normal:1
```

**工作原理：**
- 收集线程的总 CPU 时间并按降序排序
- 以轮询方式将理想处理器分配到配置的 CPU
- 应用小的随机偏移以避免聚集
- 在亲和性更改后自动运行，或在 cpuset 使用 `@` 前缀后在 CPU 集更改后运行

详情请参见 [`reset_thread_ideal_processors()`](docs/zh-CN/apply.rs/reset_thread_ideal_processors.md)。



## 命令行选项

### 基本选项

| 选项 | 说明 |
|------|------|
| `-help` | 显示基本帮助 |
| `-helpall` | 显示详细帮助和示例 |
| `-console` | 输出到控制台而非日志文件 |
| `-config <file>` | 使用自定义配置文件（默认：`config.ini`） |
| `-blacklist <file>` | `-find` 模式的黑名单文件 |
| `-noUAC` | 不请求管理员权限 |
| `-interval <ms>` | 检查间隔，毫秒（默认：`5000`，最小：`16`） |
| `-resolution <0.0001ms>` | 设置计时器分辨率（如 `5210` = 0.5210ms，`0` = 不设置） |

### 运行模式

| 模式 | 说明 |
|------|------|
| `-find` | 记录具有默认亲和性的未管理进程 |
| `-convert` | 转换 Process Lasso 配置（`-in <file> -out <file>`） |
| `-autogroup` | 自动将相同规则的进程合并为具名分组（`-in <file> -out <file>`） |
| `-validate` | 验证配置文件语法（不运行） |
| `-processlogs` | 处理日志以查找新进程和搜索路径 |
| `-dryrun` | 显示将会更改的内容（不实际应用） |

### 调试选项

| 选项 | 说明 |
|------|------|
| `-loop <count>` | 运行循环次数（默认：无限） |
| `-logloop` | 每次循环开始时记录消息 |
| `-noDebugPriv` | 不请求 SeDebugPrivilege |
| `-noIncBasePriority` | 不请求 SeIncreaseBasePriorityPrivilege |
| `-noETW` | 不启动 ETW 进程监控 |
| `continuous_process_level_apply` | 每个循环重新应用进程级设置，而不是每个 PID 仅应用一次 |

完整 CLI 文档请参见 [cli.md](docs/zh-CN/cli.rs/README.md)。

## 配置

### 格式

```
process_name:priority:affinity:cpuset:prime_cpus[@prefixes]:io_priority:memory_priority:grade
```

解析后的表示请参见 [`ProcessConfig`](docs/zh-CN/config.rs/ProcessConfig.md) 结构体。

### CPU 规格

| 格式 | 示例 | 说明 |
|------|------|------|
| 范围 | `0-7` | 核心 0 到 7 |
| 多范围 | `0-7;64-71` | 用于 >64 核系统 |
| 单独核心 | `0;2;4;6` | 指定核心 |
| 单核 | `7` | 单个核心（不是掩码） |
| 十六进制掩码 | `0xFF` | 旧格式（≤64 核） |
| 别名 | `*pcore` | 预定义别名 |
| 不修改 | `0` | 不修改 |

### CPU 规格注意事项

**重要：** 普通数字如 `7` 表示核心 7，不是位掩码。使用 `0-7` 表示核心 0-7，而不是 `7`。

解析实现请参见 [`parse_cpu_spec()`](docs/zh-CN/config.rs/parse_cpu_spec.md)。

### 规则等级

`grade` 字段控制规则的应用频率（默认值：1 = 每次循环）：

| 等级 | 频率 | 使用场景 |
|------|------|----------|
| `1` | 每次循环 | 关键进程（游戏、实时应用） |
| `2` | 每第 2 次循环 | 半关键进程 |
| `5` | 每第 5 次循环 | 后台工具 |
| `10` | 每第 10 次循环 | 极少变化的进程（更新程序） |

```ini
# 每次循环应用（默认）
game.exe:high:*pcore:0:*pcore:normal:normal:0:1

# 每第 3 次循环应用（用于较不关键的进程）
background.exe:normal:*ecore:0:0:low:none:0:3

# 每第 10 次循环应用（最小监控频率）
updater.exe:normal:0:0:0:normal:none:0:10
```

### 优先级级别

| 类型 | 级别 |
|------|------|
| 进程 | `none`、`idle`、`below normal`、`normal`、`above normal`、`high`、`real time` |
| 线程 | `none`、`idle`、`lowest`、`below normal`、`normal`、`above normal`、`highest`、`time critical` |
| I/O | `none`、`very low`、`low`、`normal`、`high`（需要管理员） |
| 内存 | `none`、`very low`、`low`、`medium`、`below normal`、`normal` |

所有优先级级别定义请参见 [priority.md](docs/zh-CN/priority.rs/README.md)。

### CPU 别名

使用 `*name = spec` 定义可重用的 CPU 规格：

```ini
# === 别名 ===
*a = 0-19           # 所有核心（8P+12E）
*p = 0-7            # P 核
*e = 8-19           # E 核
*pN01 = 2-7         # P 核除 0-1
```

别名支持所有 CPU 规格格式，包括 >64 核系统的多范围。

### 进程组

使用 `{ }` 语法将多个进程组合使用相同规则。组名是可选的（仅用于文档）：

```ini
# 命名组（多行）
browsers {
    chrome.exe: firefox.exe: msedge.exe
    # 组内允许注释
}:normal:*e:0:0:low:below normal

# 命名组（单行）
sys_utils { notepad.exe: calc.exe }:none:*e:0:0:low:none

# 匿名组（无需名称）
{
    textinputhost.exe: ctfmon.exe: chsime.exe
    dllhost.exe: sihost.exe: ShellHost.exe
}:none:*e:0:0:low:none

# 匿名单行组
{ taskmgr.exe: perfmon.exe }:none:*a:0:0:none:none
```

### Prime 线程调度

`prime_cpus` 字段支持多段式 CPU 分配，包括按模块过滤和线程优先级控制。

**语法：**
```
[?[?]x]*alias1[@module1[!priority1];module2[!priority2]*alias2@module3[!priority3];module4...]
```

**解析规则：**
1. 可选跟踪前缀：`?x`（跟踪并应用）或 `??x`（仅跟踪，不应用）
2. 按 `*` 分割得到段（每段 = CPU 别名 + 其模块列表）
3. 在每段的 `@` 之后，按 `;` 分割得到模块前缀
4. 每个模块前缀可以有可选的 `!priority` 后缀

**组成部分：**

| 组成部分 | 说明 |
|----------|------|
| `prime_cpus` | Prime 线程的基础 CPU 集（所有模块） |
| `?x*prime_cpus` | 跟踪前 x 个线程，应用规则，退出时记录 |
| `??x*prime_cpus` | 仅监控：跟踪前 x 个线程，退出时记录，不应用 CPU 集 |
| `*alias@module1;module2` | 仅提升来自指定模块前缀的线程，使用 alias CPU |
| `*alias1@mod1*alias2@mod2` | 多段式：mod1 线程在 alias1 CPU，mod2 线程在 alias2 CPU |
| `module!priority` | 设置显式线程优先级（idle 到 time critical） |
| `module` | 使用自动提升（当前优先级 + 1 级，上限为 highest） |

**示例：**

```ini
# 简单 - 所有 prime 线程在除 0-1 外的 P 核
game.exe:normal:*a:*p:*pN01:normal:normal:0:1

# 跟踪前 10 个线程，应用规则，退出时记录
game.exe:normal:*a:*p:?10*pN01:normal:normal:0:1

# 仅监控 - 跟踪前 20 个线程，退出时记录，不应用 CPU 集
game.exe:normal:*a:*p:??20*pN01:normal:normal:0:1

# 模块过滤 - 仅 CS2 和 NVIDIA 线程
cs2.exe:normal:*a:*p:*pN01@cs2.exe;nvwgf2umx.dll:normal:normal:0:1

# 多段式 - CS2 在 P 核，NVIDIA 在 E 核
cs2.exe:normal:*a:*p:*p@cs2.exe*e@nvwgf2umx.dll:normal:normal:0:1

# 按模块线程优先级 - CS2 为 time critical，NVIDIA 为 above normal
cs2.exe:normal:*a:*p:*pN01@cs2.exe!time critical;nvwgf2umx.dll!above normal:normal:normal:0:1

# 三段式，不同 CPU 和优先级
game.exe:normal:*a:*p:*p@engine.dll!time critical*pN01@render.dll!highest*e@background.dll!normal:normal:normal:0:1

# 混合 - 部分显式优先级，其他自动提升
game.exe:normal:*a:*p:*pN01@UnityPlayer.dll!time critical;GameModule.dll:normal:normal:0:1

# 跟踪和多段式 - 跟踪前 10，CS2 在 P 核，NVIDIA 在 E 核
cs2.exe:normal:*a:*p:?10*p@cs2.exe*e@nvwgf2umx.dll:normal:normal:0:1
```

### 理想处理器（首核）分配（Ideal Processor Assignment）

可选的 `ideal` 字段可以插入到规则的最后 `grade` 字段之前，用于请求对进程中最繁忙线程的静态理想处理器分配。该配置使用与 ALIASES 中相同的 `*name` CPU 别名，并支持按模块前缀进行可选过滤。

- 位置：`ideal` 字段位于规则中的最后 `grade` 字段之前。
- 语法：
  - `*alias` — 将该别名表示的 CPUs 用作匹配线程的候选理想处理器（匹配所有线程）。
  - `*alias@prefix1;prefix2` — 仅对其起始模块名以某个前缀开始的线程应用该别名的 CPUs（多个前缀以 `;` 分隔）。
  - 支持链式多规则：`*alias1@mod1*alias2@mod2`
- 语义：
  - 对于每个 `*alias` 规则，程序会按照线程的总 CPU 使用（内核 + 用户时间）对匹配线程进行排序。对于该别名所包含的 CPU 数量 N，选取排名前 N 的线程，将它们分别按排名映射到别名内的 CPU 索引并设置为理想处理器（ideal processor）。
  - 当某线程不再位列前 N 时，会尝试将其之前的理想处理器值恢复回去。
  - 如果别名不包含模块过滤（没有 `@...`），则匹配该进程的所有线程。
  - 当前实现将理想处理器应用到处理器组 0（对于 >64 逻辑处理器且存在多个处理器组的系统暂不支持。
- 示例：
```ini
# 将 *pN01 的 CPUs 作为 UnityPlayer.dll 相关线程的理想处理器
game.exe:normal:*a:*p:*pN01@UnityPlayer.dll:normal:normal:*pN01@UnityPlayer.dll:1

# 多规则：engine 线程 -> p 核，render 线程 -> pN01 子集
game.exe:normal:*a:*p:*p@engine.dll*pN01@render.dll:normal:normal:*p@engine.dll*pN01@render.dll:1

# 别名无过滤：对所有线程应用（对最繁忙的 N 个线程分配理想 CPU）
background.exe:normal:*a:*p:*p:normal:normal:*p:5
```

**当跟踪的进程退出时**，为每个线程记录详细统计信息：
- 线程 ID 和总 CPU 周期
- 起始地址解析为 `module.dll+offset` 格式
- 内核时间和用户时间
- 线程优先级和状态
- 上下文切换和等待原因

### 调度器常量

配置 prime 线程调度行为：

```ini
@MIN_ACTIVE_STREAK = 2   # 提升前需要的连续活跃间隔数
@ENTRY_THRESHOLD = 0.42  # 成为候选的最大周期比例
@KEEP_THRESHOLD = 0.69   # 保持 prime 状态的最大周期比例
```

上述常量同时适用于 **Prime 线程调度**和**理想处理器分配**——两者共用同一个滞后过滤器。

结构体定义请参见 [`ConfigConstants`](docs/zh-CN/config.rs/ConfigConstants.md)。

### 完整示例

```ini
# === 常量 ===
@MIN_ACTIVE_STREAK = 2   # 提升前需要的连续活跃间隔数
@ENTRY_THRESHOLD = 0.42  # 成为候选的最大周期比例
@KEEP_THRESHOLD = 0.69   # 保持 prime 状态的最大周期比例

# === 别名 ===
*a = 0-19           # 所有核心（8P+12E）
*p = 0-7            # P 核
*e = 8-19           # E 核
*pN01 = 2-7         # P 核除 0-1
*pN0 = 1-7          # P 核除 0

# === 规则 ===
# 格式：process:priority:affinity:cpuset:prime[@prefixes]:io:memory:ideal[@prefixes]:grade

# 单进程 - 简单
cs2.exe:normal:*a:*p:*pN01:normal:normal:0:1

# Prime 带模块过滤 - 仅特定模块
game.exe:normal:*a:*p:*pN01@UnityPlayer.dll;GameModule.dll:normal:normal:0:1

# 多段式 - 不同模块不同核心
cs2.exe:normal:*a:*p:*p@cs2.exe*e@nvwgf2umx.dll:normal:normal:0:1

# 按模块线程优先级
cs2.exe:normal:*a:*p:*pN01@cs2.exe!time critical;nvwgf2umx.dll!above normal:normal:normal:0:1

# 三段式，不同 CPU 和优先级
game.exe:normal:*a:*p:*p@engine.dll!time critical*pN01@render.dll!highest*e@background.dll!normal:normal:normal:0:1

# 跟踪前 10 个线程 - 退出时记录
game.exe:normal:*a:*p:?10*pN01@UnityPlayer.dll:normal:normal:0:1

# 仅监控 - 跟踪但不应用
game.exe:normal:*a:*p:??20*pN01:normal:normal:0:1

# 命名组 - 浏览器在 E 核
browsers { chrome.exe: firefox.exe: msedge.exe }:normal:*e:0:0:low:below normal:1

# 匿名组 - 后台应用
{
    discord.exe: telegram.exe: slack.exe
}:below normal:*e:0:0:low:low:2

# 系统进程（高 I/O 需要管理员）
dwm.exe:high:*p:0:0:high:normal:1

# Process Lasso（E 核低优先级）
process_mgmt {
    bitsumsessionagent.exe: processgovernor.exe: processlasso.exe
    affinityservicerust.exe: affinityserverc.exe
}:none:*e:0:0:low:none:1
```

## 特权和能力

### 需要了解的内容

| 目标进程 | 无管理员 | 管理员 | 说明 |
|----------|---------|-------|------|
| 同用户进程 | ✅ 完全 | ✅ 完全 | 无需提权即可工作 |
| 已提升进程 | ❌ | ✅ 完全 | 需要管理员 |
| SYSTEM 进程 | ❌ | ✅ 完全 | 需要管理员 |
| 受保护进程 | ❌ | ❌ | 即使管理员也无法修改 |

| 规则 | 无管理员 | 管理员 | 说明 |
|------|---------|-------|------|
| 进程优先级 | ✅ | ✅ | 所有级别均可用 |
| CPU 亲和性 | ✅ | ✅ | 仅限 ≤64 核 |
| CPU 集 | ✅ | ✅ | 适用于 >64 核 |
| Prime 调度 | ✅ | ✅ | 线程级 CPU 集 |
| I/O 优先级 - 高 | ❌ | ✅ | 需要管理员（SeIncreaseBasePriorityPrivilege） |
| 内存优先级 | ✅ | ✅ | 所有级别均可用 |

**建议：** 以管理员权限运行以获得完整功能，特别是 I/O 优先级`high`和管理 SYSTEM 进程。

## 工具

### 进程发现

使用 `-processlogs` 模式从日志中发现配置和黑名单中尚未包含的新进程。

**要求：**
- Everything 搜索工具，`es.exe` 在 PATH 中
- `-find` 模式生成的日志文件

**工作流：**
```bash
# 1. 扫描未管理的进程（按需或每日运行）
AffinityServiceRust.exe -find -console

# 2. 处理日志以查找新进程
AffinityServiceRust.exe -processlogs

# 3. 使用自定义配置和黑名单
AffinityServiceRust.exe -processlogs -config my_config.ini -blacklist my_blacklist.ini

# 4. 指定日志目录和输出文件
AffinityServiceRust.exe -processlogs -in mylogs -out results.txt
```

这会扫描 `logs/` 目录（或用 `-in` 指定）中的 `.find.log` 文件，提取进程名称，过滤掉已配置/黑名单中的进程，并使用 `es.exe` 搜索其余进程。结果保存到 `new_processes_results.txt`（或用 `-out` 指定），将每个进程与文件路径配对以便审查。

### 配置转换

转换 Process Lasso 配置文件到 AffinityServiceRust 格式：

```bash
AffinityServiceRust.exe -convert -in prolasso.ini -out my_config.ini
```

这将 Process Lasso 规则转换为 AffinityServiceRust 配置格式，便于迁移。

### 配置自动分组

将规则字符串完全相同的进程自动合并为具名分组块：

```bash
AffinityServiceRust.exe -autogroup -in config.ini -out config_grouped.ini
```

规则字符串完全一致的条目将合并到 `grp_N { }` 块中，成员按字母顺序排列。若整行长度小于 128 个字符，则输出为单行格式；否则以 `: ` 分隔成员并换行，每行不超过 128 个字符。

**输入：**
```ini
explorer.exe:none:*a:*e:0:none:none:0:4
cmd.exe:none:*a:*e:0:none:none:0:4
notepad.exe:none:*a:*e:0:none:none:0:4
```

**输出：**
```ini
grp_0 { cmd.exe: explorer.exe: notepad.exe }:none:*a:*e:0:none:none:0:4
```

前导部分（`@常量`、`*别名` 及注释行）原样保留。规则间的单行注释在重新分组时会被丢弃。

实现请参见 [`sort_and_group_config()`](docs/zh-CN/config.rs/sort_and_group_config.md)。

### 配置验证

运行前验证配置文件语法：

```bash
AffinityServiceRust.exe -validate -config config.ini
```

检查：
- 语法错误
- 未定义的 CPU 别名
- 无效的优先级值
- 格式错误的进程组

## 调试

```bash
# 验证配置语法
AffinityServiceRust.exe -validate -config config.ini

# 试运行 - 查看将会更改的内容（不实际应用）
AffinityServiceRust.exe -dryrun -config config.ini

# 非管理员，带控制台（用于测试）
AffinityServiceRust.exe -console -noUAC -logloop -loop 3 -interval 2000

# 管理员模式（运行后查看 logs/YYYYMMDD.log）
AffinityServiceRust.exe -logloop -loop 3 -interval 2000
```

> **注意：** 使用 UAC 提权运行时，避免使用 `-console`，因为 UAC 会生成新进程且窗口立即关闭。请检查日志文件。

详见 [DEBUG.md](DEBUG.md)。

AI 代理贡献者（Zed、Cursor 等）请参阅 project_specific_agent.md 了解 CLI 工具和工作流技巧。

## 构建

```bash
# 通过 rustup 安装 Rust（选择 MSVC 构建工具）
cargo build --release
```

二进制文件位于 `target/release/AffinityServiceRust.exe`。

对于 rust-analyzer 支持，还需安装 MSBuild 和 Windows 11 SDK。

## 工作原理

1. **初始化**
   - 解析命令行参数
   - 通过 [`read_config()`](docs/zh-CN/config.rs/read_config.md) 加载并验证配置文件
   - 请求管理员提权（除非 `-noUAC`）
   - 启用 [`SeDebugPrivilege`](docs/zh-CN/winapi.rs/enable_debug_privilege.md) 和 [`SeIncreaseBasePriorityPrivilege`](docs/zh-CN/winapi.rs/enable_inc_base_priority_privilege.md)
   - 设置计时器分辨率（如果指定）
   - 终止从启动器继承的任何子进程（如计划任务运行器附加的 `conhost.exe`），在进入主循环前执行清理
   - 启动 [`EtwProcessMonitor`](docs/zh-CN/event_trace.rs/EtwProcessMonitor.md) 用于响应式进程检测——新进程无需等待下一个轮询间隔即可触发规则应用

2. **主循环**（每个间隔，默认 5000ms）
   - 通过 [`NtQuerySystemInformation`](docs/zh-CN/process.rs/ProcessSnapshot.md) 获取所有运行进程的快照
   - 对于每个匹配配置规则的进程：
     - 通过 [`apply_config_process_level()`](docs/zh-CN/main.rs/apply_config_process_level.md) 应用进程级设置（默认一次性，启用 `continuous_process_level_apply` 后则每个循环都应用：优先级、亲和性、CPU 集、I/O、内存）
     - 通过 [`apply_config_thread_level()`](docs/zh-CN/main.rs/apply_config_thread_level.md) 应用线程级设置（每次迭代：prime 线程调度、理想处理器分配）
   - 记录所有更改
   - 清理已死进程/线程句柄
   - 休眠直到下一个间隔

3. **ETW 响应式检测**：来自 [`EtwProcessMonitor`](docs/zh-CN/event_trace.rs/EtwProcessMonitor.md) 的进程启动事件会立即触发进程级规则应用；进程停止事件清理调度器状态和错误跟踪

4. **Prime 线程调度**（每个进程，每个间隔）
   - 选择候选线程（按 CPU 时间排序，过滤已死线程）
   - 查询候选线程的 CPU 周期（通过 `QueryThreadCycleTime`）
   - 计算自上次检查以来的增量周期
   - 更新活跃连击（连续高活跃间隔）
   - 提升超过入场阈值且连击充足的线程
   - 降级低于保持阈值的线程
   - 通过 [`SetThreadSelectedCpuSets`](docs/zh-CN/scheduler.rs/PrimeThreadScheduler.md) 应用 CPU 集
   - 可选提升线程优先级（自动或显式）

5. **共享周期预取**（每进程，prime 与 ideal 调度前）
   - 利用 `NtQuerySystemInformation` 缓冲区中的 CPU 时间增量对所有线程排序（无额外系统调用）
   - 仅保留前 N 个线程（N = 逻辑 CPU 数）——排名以外的线程不可能赢得任何分配槽位
   - 仅对前 N 个线程打开句柄并调用 `QueryThreadCycleTime`；结果缓存于 [`ThreadStats::cached_cycles`](docs/zh-CN/scheduler.rs/ThreadStats.md)

6. **理想处理器分配**（每进程，每间隔）
   - 应用与 Prime 线程调度相同的滞后过滤器（连击 + 保留/入场阈值）
   - 第 1 轮：已分配且高于 `keep_threshold` 的线程保留槽位，无写系统调用
   - 第 2 轮：晋升满足连击条件的新线程；若线程已在空闲 CPU 池中则跳过 `SetThreadIdealProcessorEx`（延迟设置）
   - 降级不再被选中的线程，恢复原始理想处理器

7. **热重载**
   - 监控配置文件修改时间
   - 变更时，重新加载并验证
   - 如果有效，立即应用新配置
   - 如果无效，保持先前配置并记录错误

8. **进程退出跟踪**
   - 当跟踪的进程退出时，记录 CPU 周期消耗最高的前 N 个线程
   - 通过 `psapi GetMappedFileName` 解析线程起始地址为 `module.dll+offset` 格式
   - 清理模块缓存

## 已知行为

7. **自身子进程上出现 `[SET_AFFINITY][ACCESS_DENIED]`**：当本服务派生了子进程（例如由计划任务运行器附加的 `conhost.exe`，或 UAC 重启动时产生的进程），而该子进程的名称恰好匹配某条配置规则时，服务会尝试对其应用亲和性，并向 `.find.log` 写入如下一行：

   ```
   apply_config: [SET_AFFINITY][ACCESS_DENIED]  6976-conhost.exe
   ```

   这是**预期行为** —— 服务会对快照中所有名称匹配的进程应用规则，包括自身短暂存活的子进程。该子进程会在主循环启动前被终止（见启动清理逻辑），因此此条目每次运行最多出现一次，可安全忽略。

8. **`[OPEN][ACCESS_DENIED]` 按线程去重**：当 [`apply_config_process_level()`](docs/zh-CN/main.rs/apply_config_process_level.md)/[`apply_config_thread_level()`](docs/zh-CN/main.rs/apply_config_thread_level.md) 因 `ACCESS_DENIED`（或其他错误）无法打开某进程或线程时，该错误仅对每个唯一的 `(pid, tid, 进程名, 操作)` 组合写入一次 `.find.log`。每次获取快照后，去重映射表会与当前快照对账：PID 已退出或被其他可执行文件复用的条目将被清除，因此若同一进程名在新 PID 下再次出现，错误将重新触发一次。同名可执行文件的多个并发实例（如具有不同 PID 的多个 `svchost.exe`）被独立跟踪——某个实例被拒绝访问，不会压制其他同名但不同 PID 实例的错误输出。

错误去重实现请参见 [`is_new_error()`](docs/zh-CN/logging.rs/is_new_error.md)。

## 已知限制

1. **CPU 亲和性 ≤64 核**：旧版 SetProcessAffinityMask API 只能在单个处理器组内工作。对于 >64 核系统，请使用 CPU 集。

2. **多组/NUMA 系统**：本项目尚未在多个处理器组或 NUMA 系统上测试。`ideal` 处理器分配当前仅在处理器组 0 内分配处理器。具有 >64 逻辑处理器或多个 CPU 组的系统可能会遇到意外行为。

3. **受保护进程**：如 `csrss.exe` 和 `smss.exe` 之类的进程无法修改，即使有管理员权限。

4. **提权时的控制台输出**：使用 UAC 提权时运行 `-console`， Elevated 进程会在新窗口中启动并立即关闭。请使用日志文件输出。

5. **线程起始地址解析**：需要管理员权限和 SeDebugPrivilege。无提权时，起始地址显示为 `0x0`。

6. **计时器分辨率**：系统计时器分辨率影响循环精度。非常低的值（<1ms）可能会影响系统稳定性。

## 贡献

欢迎提交问题和拉取请求。

请尝试更新此 README 时更新提交 SHA：**920d8fafb3d9e22e6078f62bbb7d8d97e7d21c4b**。这让开发者能够对比最新源码以理解变更。

## 许可证

详见 [LICENSE](LICENSE) 文件。
