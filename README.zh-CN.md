# Affinity Service Rust

一个用 Rust 为 Windows 编写的简单应用程序，自动为特定进程管理进程优先级、CPU 亲和性、Windows CPU 集、线程级 CPU 调度和 I/O 优先级。它从一个简单的配置文件读取规则并持续应用。

## 功能

- 进程优先级管理：自动设置优先级类别（Idle、Below Normal、Normal、Above Normal、High、Real-time）
- CPU 亲和性管理：使用亲和性掩码将进程限制到指定 CPU 核心（硬性限制）
- CPU 集管理：为进程分配 Windows 的首选 CPU 集（软性偏好）
- 线程级 CPU集 调度：动态识别最活跃的线程并将其调度到指定的核心（软性偏好）
- I/O 优先级管理：控制 I/O 优先级（Very Low、Low、Normal）
- 计时器分辨率管理：调整 Windows 计时器分辨率
- 简单配置：可编辑的 INI 文件以定义进程规则
- 查找未管理的进程：发现哪些进程可以受益于自定义设置并记录系统上曾运行的程序
- 与 Process Lasso 兼容：将现有的 Process Lasso 配置转换为 Affinity Service Rust 格式
- 灵活运行：可在有或无管理员权限下运行；支持控制台或后台模式

关于亲和性与 CPU 集：CPU 亲和性是对进程可运行核心的硬性限制（子进程会继承亲和性），而 Windows 的 CPU 集是调度器的偏好设置，表示首选核心但不严格限制。

### 线程级 Prime Core 调度

对于拥有大量线程的应用程序（如使用线程池的游戏），Prime Core 调度功能可以识别 CPU 使用最密集的线程，并使它们优先偏好在指定的核心上。

工作原理：
1. 监控线程随时间的 CPU 周期消耗
2. 使用入场阈值（默认为最大值的 42%）过滤低活跃度线程
3. 使用保持阈值（默认为最大值的 69%）保护已提升的线程不被降级
4. 要求线程持续活跃（连续 2+ 个间隔）才能被提升
5. 减少不必要的提升/降级操作以降低系统调用开销

适用场景：
- 使用线程池的游戏，其中主线程和渲染线程应优先访问同一簇 P 核
- 软性避免经常处理硬件中断的 CPU 核心 0/1
- 减少关键线程的 L2 缓存抖动和上下文切换

## 快速开始

1.  下载或编译 `AffinityServiceRust. exe`
2.  从本仓库获取配置文件（或自行创建）：
   - 使用预配置的 [`config.ini`](https://github.com/Prohect/AffinityServiceRust/blob/master/config.ini) 作为起点（覆盖 200+ 常见进程）
   - 使用包含的 [`blacklist. ini`](https://github.com/Prohect/AffinityServiceRust/blob/master/blacklist.ini) 作为进程发现模式的黑名单
   - 根据你的 CPU 拓扑和偏好编辑这些文件
3. 运行应用程序 ─ 建议使用命令行；也可以双击 . exe 使用默认选项运行

注意：默认情况下，应用静默在后台运行，并将活动记录到 `logs\YYYYmmDD.log` 和 `logs\YYYYmmDD.find.log`。使用 `-console` 参数可查看实时输出。

建议以管理员身份运行以允许修改系统/全局设置；在需要避免提权时可使用 `-noUAC` 参数。

### 基本用法

```bash
# 使用自定义配置文件（若未以管理员运行，某些更改可能受限；参见 '-noUAC'）
AffinityServiceRust.exe -config my_config.ini -console
```

### 获取帮助

```bash
# 显示基本帮助
AffinityServiceRust. exe -help

# 显示包含所有选项与示例的详细帮助
AffinityServiceRust.exe -helpall
```

### 高级用法

转换 Process Lasso 配置：
```bash
AffinityServiceRust.exe -convert -in prolasso.ini -out my_config.ini
```

查找未被管理的进程：
```bash
# 任何具有默认亲和性且不在 config 或 blacklist 中的进程会被记录到 logs\YYYYmmDD. find.log
AffinityServiceRust.exe -find
```

## 常用选项

| 选项 | 说明 |
|------|------|
| `-help` | 显示基本帮助信息 |
| `-helpall` | 显示包含所有选项与示例的详细帮助 |
| `-console` | 输出到控制台而不是日志文件 |
| `-config <file>` | 使用自定义配置文件（默认：`config.ini`） |
| `-noUAC` | 运行时不请求管理员权限 |
| `-interval <ms>` | 检查间隔，毫秒（默认：`5000`） |
| `-resolution <0.0001ms>` | 要设置的计时器分辨率（默认：不更改） |

使用 `-helpall` 查看包括转换与调试功能在内的全部可用选项。

---

## 配置

### 配置文件格式

格式：`process_name,priority,affinity_mask,cpu_set_mask,prime_cpu_mask,io_priority`

字段说明：
- process_name：可执行文件名（例如 `chrome.exe`）
- priority：`none`、`idle`、`below normal`、`normal`、`above normal`、`high`、`real time`
- affinity_mask：亲和性掩码，十六进制（如 `0xFF`）或别名（如 `*pcore`），或 `0` 表示不更改
- cpu_set_mask：CPU 集掩码（格式同上），或 `0` 表示不更改
- prime_cpu_mask：线程级 Prime Core 调度的 CPU 掩码，或 `0` 表示禁用
- io_priority：`none`、`very low`、`low`、`normal`

### 调度常量

可以在配置中调整线程调度行为的常量：

```ini
@ENTRY_THRESHOLD = 0.42    # 被考虑提升的最小周期占比
@KEEP_THRESHOLD = 0.69     # 保护不被降级的最小周期占比
```

示例 `config. ini`：
```ini
# === 调度常量 ===
@ENTRY_THRESHOLD = 0.42
@KEEP_THRESHOLD = 0. 69

# === 亲和性别名 ===
# 定义别名一次并在规则中复用；若 CPU 拓扑变化只需修改这里
*a = 0xFFFFF        # 所有核心
*p = 0xFF           # 性能核心 0-7
*e = 0xFFF00        # 效率核心 8-19
*pN0 = 0xFE         # 除核心 0 外的 P 核
*pN01 = 0xFC        # 除核心 0-1 外的 P 核

# === 进程配置 ===
# 列顺序：process_name,priority,affinity_mask,cpu_set_mask,prime_cpu_mask,io_priority

# 游戏 - 使用 prime core 调度将主线程/渲染线程固定到 P 核（避开核心 0/1）
cs2. exe,normal,*a,*p,*pN01,normal
game.exe,high,*a,*p,*pN01,normal

# 后台应用 - 效率核心，低 I/O 优先级
discord.exe,below normal,*e,0,0,low
chrome.exe,normal,*e,0,0,low

# 工作应用
code.exe,above normal,*a,*e,0,normal
```

### 设置说明

| 字段 | 可选值 | 说明 |
|------|--------|------|
| Priority（优先级） | `none`、`idle`、`below normal`、`normal`、`above normal`、`high`、`real time` | 进程优先级类别 |
| Affinity（亲和性） | `0`、`0xFF`、`*alias_name` | CPU 核心掩码（十六进制、十进制或别名），`0` 表示不更改 |
| CPU set（CPU 集） | `0`、`0xFF`、`*alias_name` | 进程的 Windows CPU 集掩码（格式同亲和性） |
| Prime CPU | `0`、`0xFF`、`*alias_name` | 线程级调度的 CPU 掩码（`0` 表示禁用） |
| I/O Priority（I/O 优先级） | `none`、`very low`、`low`、`normal` | I/O 优先级等级 |

亲和性选项：
- 直接数值：例如 `0xFF`（核心 0-7）、`0xF000`（核心 12-15），也支持十进制如 `255`
- 别名：用 `*name = 0xFF` 定义后在规则中以 `*name` 引用
- `0`：表示不更改当前值

提示：
- 最佳实践：使用别名使配置更简洁且易维护
- 快速设置：从仓库下载预配置的 [`config.ini`](https://github.com/Prohect/AffinityServiceRust/blob/master/config.ini) 和 [`blacklist.ini`](https://github. com/Prohect/AffinityServiceRust/blob/master/blacklist.ini)
- 升级 CPU 后只需修改别名定义即可一次性更新所有规则
- 使用 `none` 或 `0` 跳过对某一项的更改
- 对于游戏，考虑使用 prime_cpu_mask 避开处理中断的核心 0/1
- 运行 `AffinityServiceRust.exe -helpall` 获取详细配置说明与别名示例

### 使用仓库中的配置文件

快速设置步骤：
1. 从仓库下载 [`config.ini`](https://github.com/Prohect/AffinityServiceRust/blob/master/config.ini) 和 [`blacklist.ini`](https://github. com/Prohect/AffinityServiceRust/blob/master/blacklist.ini)
2. 在 `config.ini` 中编辑亲和性别名以匹配你的 CPU 拓扑：
```ini
# Intel 8P+12E（例如 14700KF）
*p = 0xFF          # 核心 0-7
*e = 0xFFF00       # 核心 8-19

# Intel 6P+8E
*p = 0x3F          # 核心 0-5
*e = 0x3FC0        # 核心 6-13
```
3. 将这些文件放在与 `AffinityServiceRust.exe` 相同的文件夹中
4. 运行程序

好处：
- 为许多常见应用立即提供优化
- 经测试的配置对大多数系统适用
- 通过编辑别名轻松自定义
- 可维护的配置 ─ 在一个地方修改 CPU 拓扑即可应用到所有规则
- 社区维护的规则将随着时间改进

---

## 使用注意与说明

- 建议使用管理员权限以便管理系统进程；对于受限场景可使用 `-noUAC`
- 性能影响：程序本身占用极少的 CPU 与内存；默认扫描间隔为 5 秒（可配置）
- 日志：在 `logs` 文件夹中生成带时间戳的日志；使用 `-console` 可查看实时输出
- Process Lasso 用户：使用 `-convert` 导入现有设置
- 对于使用线程池的游戏，prime core 调度可以帮助稳定帧时间，让关键线程运行在快速核心上

## 编译

- 可以使用 rustup 安装 Rust 和 cargo
- 安装过程中会提示安装 Visual Studio 构建工具
- 默认只选择一个组件：MSVC
- 这对 cargo 构建应用程序已经足够
- 但如果需要 rust-analyzer，还需要以下组件：
    - MSBuild
    - Windows 11 SDK
- 运行 `cargo build --release` 编译应用程序

## 参与贡献

如果你发现问题或有改进建议，欢迎打开 issue 或提交 pull request。

---
