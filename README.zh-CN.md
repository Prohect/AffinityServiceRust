# Affinity Service Rust

<!-- languages -->
- 🇺🇸 [English](https://github.com/Prohect/AffinityServiceRust/blob/master/README.md)
- 🇨🇳 [中文 (简体)](https://github.com/Prohect/AffinityServiceRust/blob/master/README.zh-CN.md)

一个用 Rust 编写的 Windows 进程管理工具，根据配置文件自动为进程应用 CPU 亲和性、优先级、I/O 优先级和内存优先级规则。

## 功能

| 功能 | 说明 |
|------|------|
| **进程优先级** | 设置优先级类别（Idle → Real-time） |
| **CPU 亲和性** | 限制进程到指定核心（硬性限制，仅 ≤64 核心，子进程继承） |
| **CPU 集** | 通过 Windows CPU 集分配首选核心（软性偏好，支持 >64 核心） |
| **Prime Core 调度** | 将最活跃的线程分配到指定核心（软性偏好） |
| **自动重载** | 当检测到 `config` 或 `blacklist` 更改时自动重新加载 |
| **后置报告** | 在受监控进程退出时记录前 X 个高负载线程的详细统计信息 |
| **I/O 优先级** | 控制 I/O 优先级（Very Low → High，High 需要管理员） |
| **内存优先级** | 控制内存页面优先级（Very Low → Normal） |
| **计时器分辨率** | 调整 Windows 计时器分辨率 |

> **关于 >64 核心系统：** CPU 亲和性（硬性限制）仅在单个处理器组内有效（≤64 核心）。对于 >64 核心的系统，请使用 CPU 集，它可以跨所有处理器组工作（软性偏好）。

### Prime Core 调度

针对多线程应用（如游戏），此功能识别 CPU 密集型线程并通过 Windows CPU 集将其分配到指定核心（软性偏好，非硬性固定）：

- 监控线程 CPU 周期消耗。
- 过滤低活跃线程（入场阈值：最大值的 42%）。
- 保护已提升线程不被过早降级（保持阈值：最大值的 69%）。
- 要求持续活跃（可通过 `@MIN_ACTIVE_STREAK` 配置，默认：2 个间隔）才能提升。
- 可选择通过前缀模式按起始模块名称过滤线程（语法：`prime_cpus@prefix1;prefix2`）。
- 以管理员运行时记录线程起始地址及模块解析（如 `ntdll.dll+0x3C320`）。

#### 监控与后置报告
通过在 `prime_cpus` 字段中使用 `?` 或 `??` 前缀，您可以跟踪线程性能并在进程退出时查看详细报告：

- `?*alias`: **监控 + 应用**。正常应用规则，但在退出时记录“前 X”线程报告。
- `??*alias`: **仅监控**。跳过所有核心/优先级更改，仅跟踪线程并在退出时记录报告。
- `?10*alias`: 自定义前 X 统计数量（例如，显示前 10 个线程，默认为 `2 * 核心数`）。

报告包含：**总周期 (Total Cycles)**、**上下文切换 (Context Switches)**、**线程状态**、**优先级**以及**起始模块地址**。

**注意：** 线程起始地址解析需要管理员权限和 SeDebugPrivilege。无提权时，起始地址显示为 `0x0`。

### 配置自动重载
服务会监控配置文件和黑名单文件的修改时间戳。当您保存 `config.ini` 时，服务将：
1. 在下一个检查间隔（默认 5s）内检测到更改。
2. 验证新配置。
3. 如果有效，立即应用新规则。
4. 如果新配置存在语法错误，则保留旧配置（安全重载）。

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

## 命令行选项

| 选项 | 说明 |
|------|------|
| `-help` | 显示基本帮助 |
| `-helpall` | 显示详细帮助和示例 |
| `-console` | 输出到控制台而非日志文件 |
| `-config <file>` | 使用自定义配置文件 |
| `-noUAC` | 不请求管理员权限 |
| `-interval <ms>` | 检查间隔（默认：`5000`） |
| `-resolution <0.0001ms>` | 设置计时器分辨率 |
| `-find` | 记录未管理的进程 |
| `-convert` | 转换 Process Lasso 配置 |
| `-validate` | 验证配置文件语法 |
| `-processlogs` | 处理日志以查找新进程 |
| `-dryrun` | 显示将会更改的内容（不实际应用） |

## 配置

### 格式

```
process_name:priority:affinity:cpu_set:prime_cpus[@prefixes]:io_priority:memory_priority
```

### Prime 线程调度语法

`prime_cpus` 字段支持监控模式和每前缀优先级控制：

- `?*alias` - 监控所有线程并在退出时报告前 X 名（应用 + 监控）
- `??*alias` - 监控所有线程并在退出时报告前 X 名（仅监控）
- `*p@engine.dll!time critical` - 将引擎线程设为最高优先级并分配至 P 核

#### 示例
- `??*pN01@cs2.exe;nvwgf2umx.dll` - 监控 CS2/NVIDIA 线程而不应用任何更改
- `?10*p@cs2.exe!highest` - 对 CS2 线程应用最高优先级/P 核，并在退出时报告前 10 名

### 示例配置

```ini
# === 常量 ===
@MIN_ACTIVE_STREAK = 2   # 提升前需要的连续活跃间隔数

# === 别名 ===
*p = 0-7            # P 核
*e = 8-19           # E 核
*pN01 = 2-7         # P 核除 0-1

# === 规则 ===
# 监控模式 - 仅跟踪 CS2 线程而不应用核心绑定
cs2.exe:normal:*a:*p:??*pN01:normal:normal

# 应用规则 + 退出时监控前 10 个高负载线程
game.exe:normal:*a:*p:?10*pN01@UnityPlayer.dll;GameModule.dll:normal:normal

# 命名组 - 浏览器运行在 E 核
browsers { chrome.exe: firefox.exe: msedge.exe }:normal:*e:0:0:low:below normal
```

## 工具

### 进程发现
使用 `-find` 扫描系统并使用 `-processlogs` 分析日志，可以帮助您发现新的游戏或后台程序并获取它们的文件路径。

### 配置转换
使用 `-convert` 模式可以快速将 Process Lasso 的配置文件转换为此工具支持的格式。

## 调试

```bash
# 验证语法
AffinityServiceRust.exe -validate -config config.ini

# 试运行
AffinityServiceRust.exe -dryrun -config config.ini
```

详见 [DEBUG.md](DEBUG.md)。

## 编译

```bash
# 安装 Rust 后运行
cargo build --release
```

## 参与贡献

欢迎提交 issue 和 pull request。
