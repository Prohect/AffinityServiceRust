# Affinity Service Rust

<!-- languages -->
- 🇺🇸 [English](https://github.com/Prohect/AffinityServiceRust/blob/master/README.md)
- 🇨🇳 [中文 (简体)](https://github.com/Prohect/AffinityServiceRust/blob/master/README.zh-CN.md)

一个用 Rust 编写的 Windows 进程管理工具，根据配置文件自动为进程应用 CPU 亲和性、优先级、I/O 优先级和内存优先级规则。

## 功能

| 功能 | 说明 |
|------|------|
| **进程优先级** | 设置优先级类别（Idle → Real-time） |
| **CPU 亲和性** | 限制进程到指定核心（硬性限制，仅 ≤64 核心,子进程继承） |
| **CPU 集** | 通过 Windows CPU 集分配首选核心（软性偏好，支持 >64 核心） |
| **Prime Core 调度** | 将最活跃的线程分配到指定核心（软性偏好） |
| **I/O 优先级** | 控制 I/O 优先级（Very Low → High，High 需要管理员） |
| **内存优先级** | 控制内存页面优先级（Very Low → Normal） |
| **计时器分辨率** | 调整 Windows 计时器分辨率 |

> **关于 >64 核心系统：** CPU 亲和性（硬性限制）仅在单个处理器组内有效（≤64 核心）。对于 >64 核心的系统，请使用 CPU 集，它可以跨所有处理器组工作（软性偏好）。

### Prime Core 调度

针对多线程应用（如游戏），此功能识别 CPU 密集型线程并通过 Windows CPU 集将其分配到指定核心（软性偏好，非硬性固定）：

- 监控线程 CPU 周期消耗
- 过滤低活跃线程（入场阈值：最大值的 42%）
- 保护已提升线程不被过早降级（保持阈值：最大值的 69%）
- 要求持续活跃（可通过 `@MIN_ACTIVE_STREAK` 配置，默认：2 个间隔）才能提升
- 可选择通过前缀模式按起始模块名称过滤线程（语法：`prime_cpus@prefix1;prefix2`，默认：空匹配所有模块）
- 以管理员运行时记录线程起始地址及模块解析（如 `ntdll.dll+0x3C320`）

适用于游戏中主线程/渲染线程需要优先运行在 P 核，同时避开核心 0/1（硬件中断处理器）的场景。

> **注意：** 线程起始地址解析需要管理员权限和 SeDebugPrivilege。无提权时，起始地址显示为 `0x0`。

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

| 选项 | 说明 |
|------|------|
| `-help` | 显示基本帮助 |
| `-helpall` | 显示详细帮助和示例 |
| `-console` | 输出到控制台而非日志文件 |
| `-config <file>` | 使用自定义配置文件（默认：`config.ini`） |
| `-noUAC` | 不请求管理员权限 |
| `-interval <ms>` | 检查间隔，毫秒（默认：`5000`） |
| `-resolution <0.0001ms>` | 设置计时器分辨率 |
| `-find` | 记录未管理的进程 |
| `-convert` | 转换 Process Lasso 配置 |
| `-validate` | 验证配置文件语法（不运行） |
| `-processlogs` | 处理日志以查找新进程和搜索路径 |
| `-dryrun` | 显示将会更改的内容（不实际应用） |

## 配置

### 格式

```
process_name:priority:affinity:cpu_set:prime_cpus[@prefixes]:io_priority:memory_priority
```

### CPU 规格

| 格式 | 示例 | 说明 |
|------|------|------|
| 范围 | `0-7` | 核心 0 到 7 |
| 多范围 | `0-7;64-71` | 用于 >64 核心系统 |
| 单独指定 | `0;2;4;6` | 指定核心 |
| 单个 | `7` | 单个核心（不是掩码） |
| 十六进制掩码 | `0xFF` | 旧格式（≤64 核心） |
| 别名 | `*pcore` | 预定义别名 |
| 不更改 | `0` | 不修改 |

> **重要：** 纯数字如 `7` 表示核心 7，不是位掩码。使用 `0x7` 或 `0-2` 表示核心 0-2。

### 优先级等级

| 类型 | 等级 |
|------|------|
| 进程 | `none`, `idle`, `below normal`, `normal`, `above normal`, `high`, `real time` |
| I/O | `none`, `very low`, `low`, `normal`, `high`（需要管理员） |
| 内存 | `none`, `very low`, `low`, `medium`, `below normal`, `normal` |

### 进程组

使用 `{ }` 语法将多个进程组合使用相同规则。组名是可选的（仅用于文档/调试）：

```ini
# 命名组（单行）
browsers { chrome.exe: firefox.exe: msedge.exe }:normal:*e:0:0:low:below normal

# 命名组（多行）
asus_services {
    asuscertservice.exe
    armourycrate.exe
    # 内部允许注释
    armourycrate.service.exe
}:none:*e:0:0:low:none

# 匿名组（无需名称）
{
    textinputhost.exe: ctfmon.exe
    dllhost.exe: sihost.exe
}:none:*e:0:0:low:none

# 匿名单行组
{ taskmgr.exe: perfmon.exe }:none:*a:0:0:none:none
```

### Prime 线程调度

`prime_cpus` 字段支持可选的基于模块的过滤和每模块 CPU 分配：

- `prime_cpus` - Prime 线程的基础 CPU 集
- `prime_cpus@prefix1;prefix2` - 仅提升来自以指定前缀开头的模块的线程
- `prime_cpus@prefix1*alias1;prefix2*alias2` - 为每个模块前缀分配特定的 CPU 集

示例：
- `*pN01` - 所有 prime 线程使用除 0-1 外的 P 核
- `*pN01@cs2.exe;nvwgf2umx.dll` - 仅 CS2 和 NVIDIA 线程，使用 *pN01 CPU
- `*pN01@cs2.exe*p;nvwgf2umx.dll*e` - CS2 线程使用 P 核 (*p)，NVIDIA 线程使用 E 核 (*e)

### 示例

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

# === 规则 ===
# 进程:优先级:亲和性:cpuset:prime[@prefixes]:io:memory

# 单进程规则
cs2.exe:normal:*a:*p:*pN01:normal:normal

# Prime 带模块过滤 - 仅来自以 UnityPlayer.dll 或 GameModule.dll 开头的模块的线程
game.exe:normal:*a:*p:*pN01@UnityPlayer.dll;GameModule.dll:normal:normal

# 每模块 CPU 分配 - CS2 主线程在 P 核，NVIDIA 在 E 核
cs2.exe:normal:*a:*p:*pN01@cs2.exe*p;nvwgf2umx.dll*e:normal:normal

# 命名组 - 浏览器运行在 E 核
browsers { chrome.exe: firefox.exe: msedge.exe }:normal:*e:0:0:low:below normal

# 匿名组 - 后台应用
{
    discord.exe: telegram.exe: slack.exe
}:below normal:*e:0:0:low:low

# 系统（高 I/O 需要管理员）
dwm.exe:high:*p:0:0:high:normal
```

## 工具

### 进程发现

使用 `-processlogs` 模式从日志中发现尚未在配置或黑名单中的新进程。

**要求：**
- Everything 搜索工具，`es.exe` 在 PATH 中
- 日志文件位于 `logs/` 目录（默认，可通过 `-in` 指定），通常通过运行 `-find` 模式生成

**工作流程：**
1. 使用 `-find` 运行应用程序以扫描并记录未管理的进程到 `.find.log` 文件
2. 运行 `-processlogs` 以处理这些日志，过滤掉已配置/黑名单中的进程，并搜索文件路径

**用法：**
```bash
# 首先，扫描未管理的进程（每天运行或根据需要）
AffinityServiceRust.exe -find -console

# 然后，处理日志以查找新进程
AffinityServiceRust.exe -processlogs

# 指定配置和黑名单文件（配置默认为 config.ini，黑名单无默认值）
AffinityServiceRust.exe -processlogs -config my_config.ini -blacklist my_blacklist.ini

# 指定日志目录和输出文件
AffinityServiceRust.exe -processlogs -in mylogs -out results.txt
```

这会扫描 `logs/` 目录中的 `.find.log` 文件，提取进程名称，过滤掉已配置或黑名单中的进程，并使用 `es.exe` 搜索其余进程。结果保存到 `new_processes_results.txt`，将每个进程与文件路径配对，便于审查和添加到配置中。

适用于保持配置与新应用程序的同步更新。



### 配置转换

使用 `-convert` 模式将 Process Lasso 配置文件转换为 AffinityServiceRust 格式。

**用法：**
```bash
AffinityServiceRust.exe -convert -in prolasso.ini -out my_config.ini
```

这将 Process Lasso 规则转换为 AffinityServiceRust 配置格式，便于从 Process Lasso 迁移到 AffinityServiceRust。

## 调试

```bash
# 验证配置文件语法
AffinityServiceRust.exe -validate -config config.ini

# 试运行 - 查看将会更改的内容（不实际应用）
AffinityServiceRust.exe -dryrun -noUAC -config config.ini

# 非管理员，带控制台
AffinityServiceRust.exe -console -noUAC -logloop -loop 3 -interval 2000

# 管理员（运行后查看 logs/YYYYMMDD.log）
AffinityServiceRust.exe -logloop -loop 3 -interval 2000
```

> 以管理员运行时避免使用 `-console`，因为 UAC 会启动新进程，窗口会立即关闭。

详见 [DEBUG.md](DEBUG.md)。

使用 AI 代理（Zed、Cursor 等）的贡献者，请参阅 [AGENT.md](AGENT.md) 了解 CLI 工具和工作流程技巧。

## 编译

```bash
# 通过 rustup 安装 Rust（选择 MSVC 构建工具）
cargo build --release
```

如需 rust-analyzer 支持，还需安装 MSBuild 和 Windows 11 SDK。

## 参与贡献

欢迎提交 issue 和 pull request。

使用 AI 代理的开发者，请参阅 [AGENT.md](AGENT.md) 了解实用 CLI 工具和批量编辑工作流程。
