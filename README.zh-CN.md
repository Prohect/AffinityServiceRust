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

配置文件使用类似 INI 的格式，包含常量、别名和规则部分。

进程规则遵循以下格式：

```
process_name:priority:affinity:cpuset:prime_cpus[@prefixes]:io_priority:memory_priority:ideal[@prefixes]:grade
```

解析后的表示请参见 [`ProcessConfig`](docs/zh-CN/config.rs/ProcessConfig.md) 结构体。

### CPU 规格格式

| 格式 | 示例 | 说明 |
|------|------|------|
| 范围 | `0-7` | 核心 0 到 7 |
| 多范围 | `0-7;64-71` | 用于 >64 核系统 |
| 单独核心 | `0;2;4;6` | 指定核心 |
| 单核 | `5` | 单个核心（NOT a bitmask） |
| 十六进制掩码 | `0xFF` | 旧格式（≤64 核） |
| 别名 | `*pcore` | 预定义别名 |

**重要：** 普通数字表示核心索引，不是位掩码。使用 `0-7` 表示核心 0-7，而不是 `7`。

### 规则等级

`grade` 字段控制规则的应用频率（默认：1）：

| 等级 | 频率 | 使用场景 |
|------|------|----------|
| `1` | 每次循环 | 关键进程（游戏、实时应用） |
| `2` | 每第 2 次循环 | 半关键进程 |
| `5` | 每第 5 次循环 | 后台工具 |
| `10` | 每第 10 次循环 | 极少变化的进程 |

### 优先级级别

| 类型 | 级别 |
|------|------|
| 进程 | `none`、`idle`、`below normal`、`normal`、`above normal`、`high`、`real time` |
| 线程 | `none`、`idle`、`lowest`、`below normal`、`normal`、`above normal`、`highest`、`time critical` |
| I/O | `none`、`very low`、`low`、`normal`、`high`（需要管理员） |
| 内存 | `none`、`very low`、`low`、`medium`、`below normal`、`normal` |

详细配置语法，包括别名、组、prime 调度、理想分配、常量和示例，请参见 [parse_and_insert_rules](docs/zh-CN/config.rs/parse_and_insert_rules.md)。

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

详见 [cli.md](docs/zh-CN/cli.rs/README.md)。

AI 代理贡献者（Zed、Cursor 等）请参阅 project_specific_agent.md 了解 CLI 工具和工作流技巧。

## 构建

```bash
# 通过 rustup 安装 Rust（选择 MSVC 构建工具）
cargo build --release
```

二进制文件位于 `target/release/AffinityServiceRust.exe`。

对于 rust-analyzer 支持，还需安装 MSBuild 和 Windows 11 SDK。

## 工作原理

AffinityServiceRust 持续监控运行中的进程，并应用配置的规则，包括进程优先级、CPU 亲和性/集、I/O/内存优先级、Prime 线程调度和理想处理器分配。它使用 ETW 进行响应式进程检测，并支持配置文件的热重载。

详细架构和实现请参见 [docs/main.md](docs/zh-CN/main.rs/README.md)。

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
