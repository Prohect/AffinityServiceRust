# CLI 模块文档

命令行参数解析和帮助文本生成。

## 概述

本模块处理：
- CLI 参数解析 ([`parse_args()`](#parse_args))
- `-help` 和 `-helpall` 标志的帮助文本生成
- 用于嵌入输出文件的配置格式模板

## 调用者

- [`main()`](main.md) - CLI 解析的入口点
- 用户通过 `-help`, `-helpall` 标志

## 数据结构

### CliArgs

解析的命令行参数。

```rust
pub struct CliArgs {
    pub interval_ms: u64,                    // 检查间隔（默认：5000）
    pub help_mode: bool,                     // -help 标志
    pub help_all_mode: bool,                 // -helpall 标志
    pub convert_mode: bool,                  // -convert 标志
    pub autogroup_mode: bool,                // -autogroup 标志
    pub find_mode: bool,                     // -find 标志
    pub validate_mode: bool,                 // -validate 标志
    pub process_logs_mode: bool,             // -processlogs 标志
    pub dry_run: bool,                       // -dryrun 标志
    pub config_file_name: String,            // -config <file>
    pub blacklist_file_name: Option<String>, // -blacklist <file>
    pub in_file_name: Option<String>,        // -in <file>
    pub out_file_name: Option<String>,       // -out <file>
    pub no_uac: bool,                        // -noUAC 标志
    pub loop_count: Option<u32>,             // -loop <count>
    pub time_resolution: u32,                // -resolution <t>
    pub log_loop: bool,                      // -logloop 标志
    pub skip_log_before_elevation: bool,     // -skip_log_before_elevation
    pub no_debug_priv: bool,                 // -noDebugPriv 标志
    pub no_inc_base_priority: bool,          // -noIncBasePriority 标志
}
```

## 函数

### parse_args

将命令行参数解析为 [`CliArgs`](#cliargs)。

```rust
pub fn parse_args(args: &[String], cli: &mut CliArgs) -> Result<()>
```

**行为：**
- 未知参数被静默忽略
- 缺少值的参数使用默认值
- 验证最小间隔（16ms）

**被调用者：**[`main()`](main.md) 在启动时

### print_help

打印基本帮助消息。

```rust
pub fn print_help()
```

**显示：**常用选项（help, config, interval, find, console, noUAC, resolution）

### print_cli_help

打印详细的 CLI 帮助，包含调试选项。

```rust
pub fn print_cli_help()
```

**显示：**所有选项，包括操作模式、I/O 参数和调试/测试选项。

### get_config_help_lines

返回用于嵌入的配置文件模板。

```rust
pub fn get_config_help_lines() -> Vec<&'static str>
```

**返回：**描述配置格式的模板行，插入到转换后的配置顶部。

**使用者：**[`convert()`](config.md#process-lasso-conversion) 在生成输出时

### print_config_help

打印配置格式帮助。

```rust
pub fn print_config_help()
```

**被调用者：**[`print_help_all()`](#print_help_all)

### print_help_all

打印完整帮助（CLI + 配置格式）。

```rust
pub fn print_help_all()
```

**被调用者：**[`main()`](main.md) 中的 `-helpall` 标志处理

## CLI 参数摘要

### 分类

| 分类 | 标志 |
|------|------|
| **帮助** | `-help`, `-helpall` |
| **输出** | `-console` |
| **配置** | `-config <file>`, `-blacklist <file>` |
| **定时** | `-interval <ms>`, `-resolution <t>` |
| **模式** | `-convert`, `-autogroup`, `-find`, `-validate`, `-processlogs`, `-dryrun` |
| **I/O** | `-in <file/dir>`, `-out <file>` |
| **调试** | `-loop <n>`, `-logloop`, `-noDebugPriv`, `-noIncBasePriority`, `-skip_log_before_elevation` |
| **特权** | `-noUAC` |

### 常见用例

**基本运行：**
```bash
AffinityServiceRust.exe -config myapp.ini
```

**带控制台输出的调试：**
```bash
AffinityServiceRust.exe -console -noUAC -logloop -loop 3 -interval 2000 -config test.ini
```

**验证配置：**
```bash
AffinityServiceRust.exe -validate -config test.ini
```

**试运行（预览更改）：**
```bash
AffinityServiceRust.exe -dryrun -noUAC -config test.ini
```

**转换 Process Lasso 配置：**
```bash
AffinityServiceRust.exe -convert -in prolasso.ini -out myconfig.ini
```

## 配置格式模板

`get_config_help_lines()` 函数提供了一个描述配置格式的最小模板。完整的配置文档请参见 [config.md](config.md)。

**模板包括：**
- 字段描述（process, priority, affinity, cpuset, prime, io, memory, ideal, grade）
- CPU 别名示例
- 分组语法

## 依赖

- `crate::log` - 通过日志宏输出
- `crate::logging` - 控制台输出标志
- `windows::core::Result` - 错误处理

## 注意

- 使用 UAC 提升运行时，`-console` 输出会转到无法在当前会话中显示的新会话。请改用日志文件。
- `-skip_log_before_elevation` 标志在 UAC 提升期间自动添加，以防止重复日志记录。
