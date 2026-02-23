# AffinityServiceRust

<!-- languages -->
- 🇺🇸 [English](https://github.com/Prohect/AffinityServiceRust/blob/master/README.md)
- 🇨🇳 [中文 (简体)](https://github.com/Prohect/AffinityServiceRust/blob/master/README.zh-CN.md)

基于 Rust 编写的高性能 Windows 进程管理服务，根据配置文件自动为进程应用 CPU 亲和性、优先级、I/O 优先级和内存优先级规则。

## 功能特性

| 功能 | 说明 |
|------|------|
| **进程优先级** | 设置优先级类（空闲、低于标准、标准、高于标准、高、实时） |
| **CPU 亲和性** | 硬性绑定进程到特定核心（≤64 核，子进程继承） |
| **CPU 集** | 通过 Windows CPU Sets 软性偏好核心（支持 >64 核） |
| **Prime 线程调度** | 动态分配 CPU 密集型线程到性能核心 |
| **I/O 优先级** | 控制 I/O 优先级（极低、低、标准、高 - 需要管理员） |
| **内存优先级** | 控制内存页优先级（极低到标准） |
| **计时器分辨率** | 调整 Windows 系统计时器分辨率 |
| **热重载** | 配置文件变更时自动重新加载 |

> **关于 >64 核系统的说明：** CPU 亲和性（SetProcessAffinityMask）只能在单个处理器组内工作（≤64 核）。对于 >64 核系统，请使用 CPU 集，它可以跨所有处理器组工作，但为软性偏好。

### Prime 线程调度

针对多线程应用（如游戏），此功能动态识别 CPU 密集型线程并使用 Windows CPU Sets 将其分配到指定的"prime"核心：

**算法：**
- 实时监控线程 CPU 周期消耗
- 应用滞后机制防止频繁切换：
  - **入场阈值**：线程必须超过最大周期的 42%（可通过 `@ENTRY_THRESHOLD` 配置）
  - **保持阈值**：提升后，线程高于最大周期的 69% 保持 prime 状态（可通过 `@KEEP_THRESHOLD` 配置）
  - **活跃连击**：提升前需要连续 2+ 个间隔的持续活跃（可通过 `@MIN_ACTIVE_STREAK` 配置）
- 自动过滤低活跃线程
- 可选的基于模块的过滤：仅提升来自特定 DLL/模块的线程
- 可选的线程跟踪：进程退出时记录 CPU 周期消耗最高的前 N 个线程
- 记录线程起始地址及模块解析（如 `ntdll.dll+0x3C320`）

**多段式 CPU 分配：**
- 支持按模块覆盖 CPU：不同模块可运行在不同的核心集上
- 语法：`*alias1@module1.dll;module2.dll*alias2@module3.dll`
- 示例：CS2 游戏线程在 P 核，NVIDIA 驱动线程在 E 核

**线程优先级控制：**
- 按模块线程优先级：`module.dll!time critical` 设置显式优先级
- 自动提升模式：省略优先级时，自动提升 1 级（上限为 Highest）

**跟踪模式：**
- `?x*cpus` - 跟踪前 x 个线程，进程退出时记录详细统计信息
- `??x*cpus` - 仅监控：跟踪并记录线程但不应用 CPU 集
- 日志包含：TID、CPU 周期、内核/用户时间、上下文切换、起始地址（模块+偏移）

> **注意：** 线程起始地址解析（模块+偏移格式）需要管理员权限和 SeDebugPrivilege。无提权时，起始地址显示为 `0x0`。调试符号自动从微软符号服务器下载（企业网络可通过 `-proxy` 配置代理）。

## 快速开始

1. 下载或编译 `AffinityServiceRust.exe`
2. 下载 [`config.ini`](https://github.com/Prohect/AffinityServiceRust/blob/master/config.ini) 和 [`blacklist.ini`](https://github.com/Prohect/AffinityServiceRust/blob/master/blacklist.ini)
3. 编辑 `config.ini` 以匹配你的 CPU 拓扑
4. 运行程序（建议以管理员身份运行以获得完整功能）

```bash
# 基本用法，带控制台输出
AffinityServiceRust.exe -config my_config.ini -console

# 显示所有选项
AffinityServiceRust.exe -helpall

# 转换 Process Lasso 配置
AffinityServiceRust.exe -convert -in prolasso.ini -out my_config.ini

# 查找未管理的进程
AffinityServiceRust.exe -find
```

> **注意：** 默认静默在后台运行，日志保存在 `logs\YYYYmmDD.log`。使用 `-console` 查看实时输出。管理员权限可启用高 I/O 优先级和系统进程管理。

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
| `-proxy <url>` | 下载调试符号的 HTTP 代理（如 `http://proxy:8080`） |

### 运行模式

| 模式 | 说明 |
|------|------|
| `-find` | 记录具有默认亲和性的未管理进程 |
| `-convert` | 转换 Process Lasso 配置（`-in <file> -out <file>`） |
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

## 配置

### 格式

```
process_name:priority:affinity:cpuset:prime_cpus[@prefixes]:io_priority:memory_priority
```

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

> **重要：** 普通数字如 `7` 表示核心 7，不是位掩码。使用 `0x7` 或 `0-2` 表示核心 0-2。

### 优先级级别

| 类型 | 级别 |
|------|------|
| 进程 | `none`、`idle`、`below normal`、`normal`、`above normal`、`high`、`real time` |
| 线程 | `none`、`idle`、`lowest`、`below normal`、`normal`、`above normal`、`highest`、`time critical` |
| I/O | `none`、`very low`、`low`、`normal`、`high`（需要管理员） |
| 内存 | `none`、`very low`、`low`、`medium`、`below normal`、`normal` |

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
game.exe:normal:*a:*p:*pN01:normal:normal

# 跟踪前 10 个线程，应用规则，退出时记录
game.exe:normal:*a:*p:?10*pN01:normal:normal

# 仅监控 - 跟踪前 20 个线程，退出时记录，不应用 CPU 集
game.exe:normal:*a:*p:??20*pN01:normal:normal

# 模块过滤 - 仅 CS2 和 NVIDIA 线程
cs2.exe:normal:*a:*p:*pN01@cs2.exe;nvwgf2umx.dll:normal:normal

# 多段式 - CS2 在 P 核，NVIDIA 在 E 核
cs2.exe:normal:*a:*p:*p@cs2.exe*e@nvwgf2umx.dll:normal:normal

# 按模块线程优先级 - CS2 为 time critical，NVIDIA 为 above normal
cs2.exe:normal:*a:*p:*pN01@cs2.exe!time critical;nvwgf2umx.dll!above normal:normal:normal

# 三段式，不同 CPU 和优先级
game.exe:normal:*a:*p:*p@engine.dll!time critical*pN01@render.dll!highest*e@background.dll!normal:normal:normal

# 混合 - 部分显式优先级，其他自动提升
game.exe:normal:*a:*p:*pN01@UnityPlayer.dll!time critical;GameModule.dll:normal:normal

# 跟踪和多段式 - 跟踪前 10，CS2 在 P 核，NVIDIA 在 E 核
cs2.exe:normal:*a:*p:?10*p@cs2.exe*e@nvwgf2umx.dll:normal:normal
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
# 格式：process:priority:affinity:cpuset:prime[@prefixes]:io:memory

# 单进程 - 简单
cs2.exe:normal:*a:*p:*pN01:normal:normal

# Prime 带模块过滤 - 仅特定模块
game.exe:normal:*a:*p:*pN01@UnityPlayer.dll;GameModule.dll:normal:normal

# 多段式 - 不同模块不同核心
cs2.exe:normal:*a:*p:*p@cs2.exe*e@nvwgf2umx.dll:normal:normal

# 按模块线程优先级
cs2.exe:normal:*a:*p:*pN01@cs2.exe!time critical;nvwgf2umx.dll!above normal:normal:normal

# 三段式，不同 CPU 和优先级
game.exe:normal:*a:*p:*p@engine.dll!time critical*pN01@render.dll!highest*e@background.dll!normal:normal:normal

# 跟踪前 10 个线程 - 退出时记录
game.exe:normal:*a:*p:?10*pN01@UnityPlayer.dll:normal:normal

# 仅监控 - 跟踪但不应用
game.exe:normal:*a:*p:??20*pN01:normal:normal

# 命名组 - 浏览器在 E 核
browsers { chrome.exe: firefox.exe: msedge.exe }:normal:*e:0:0:low:below normal

# 匿名组 - 后台应用
{
    discord.exe: telegram.exe: slack.exe
}:below normal:*e:0:0:low:low

# 系统进程（高 I/O 需要管理员）
dwm.exe:high:*p:0:0:high:normal

# Process Lasso（E 核低优先级）
process_mgmt {
    bitsumsessionagent.exe: processgovernor.exe: processlasso.exe
    affinityservicerust.exe: affinityserverc.exe
}:none:*e:0:0:low:none
```

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
   - 加载并验证配置文件
   - 请求管理员提权（除非 `-noUAC`）
   - 启用 SeDebugPrivilege 和 SeIncreaseBasePriorityPrivilege
   - 设置计时器分辨率（如果指定）

2. **主循环**（每个间隔，默认 5000ms）
   - 通过 `NtQuerySystemInformation` 获取所有运行进程的快照
   - 对于每个匹配配置规则的进程：
     - 应用进程优先级
     - 应用 CPU 亲和性（通过 SetProcessAffinityMask 硬性限制）
     - 应用 CPU 集（通过 SetProcessDefaultCpuSets 软性偏好）
     - 应用 prime 线程调度（动态线程到核心分配）
     - 应用 I/O 优先级（通过 NtSetInformationProcess）
     - 应用内存优先级（通过 SetProcessInformation）
   - 记录所有更改
   - 清理已死进程/线程句柄
   - 休眠直到下一个间隔

3. **Prime 线程调度**（每个进程，每个间隔）
   - 选择候选线程（按 CPU 时间排序，过滤已死线程）
   - 查询候选线程的 CPU 周期（通过 QueryThreadCycleTime）
   - 计算自上次检查以来的增量周期
   - 更新活跃连击（连续高活跃间隔）
   - 提升超过入场阈值且连击充足的线程
   - 降级低于保持阈值的线程
   - 通过 SetThreadSelectedCpuSets 应用 CPU 集
   - 可选提升线程优先级（自动或显式）

4. **热重载**
   - 监控配置文件修改时间
   - 变更时，重新加载并验证
   - 如果有效，立即应用新配置
   - 如果无效，保持先前配置并记录错误

5. **进程退出跟踪**
   - 当跟踪的进程退出时，记录 CPU 周期消耗最高的前 N 个线程
   - 解析线程起始地址为模块+偏移格式
   - 清理符号句柄和模块缓存

## 架构

```
src/
├── main.rs         - 主循环、进程快照、应用配置
├── cli.rs          - 命令行解析、帮助消息
├── config.rs       - 配置文件解析、CPU 规格解析、别名、组
├── scheduler.rs    - Prime 线程调度器（滞后、连击跟踪）
├── priority.rs     - 优先级枚举（进程、线程、I/O、内存）
├── process.rs      - 通过 NtQuerySystemInformation 获取进程快照
├── winapi.rs       - Windows API 包装器（CPU 集、权限、符号）
└── logging.rs      - 记录到控制台或文件
```

## 限制

- **CPU 亲和性**（SetProcessAffinityMask）只能在一个处理器组内工作（≤64 核）
  - 对于 >64 核系统使用 CPU 集
- **I/O 优先级** "critical" 仅限内核，用户模式不可用
- **高 I/O 优先级**需要管理员提权
- **线程起始地址解析**需要管理员和 SeDebugPrivilege
  - 无管理员权限时，起始地址显示为 `0x0`
- **符号下载**需要互联网访问微软符号服务器
  - 企业代理网络使用 `-proxy`

## 贡献

欢迎提交问题和拉取请求。

AI 代理开发者请参阅 project_specific_agent.md 了解有用的 CLI 工具和批量编辑工作流。

## 许可证

详见 [LICENSE](LICENSE) 文件。