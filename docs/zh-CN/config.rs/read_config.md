# read_config 函数 (config.rs)

读取并解析 AffinityServiceRust 配置文件，返回包含所有已解析进程规则、常量、别名以及解析过程中遇到的错误和警告的完整 [ConfigResult](ConfigResult.md)。

## 语法

```rust
pub fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult
```

## 参数

`path`

配置文件的文件系统路径。接受任何实现 `AsRef<Path>` 的类型，如 `&str`、`String` 或 `PathBuf`。

## 返回值

返回 [ConfigResult](ConfigResult.md) 结构体，包含：

- **configs** — `HashMap<u32, HashMap<String, ProcessConfig>>`，将等级层级映射到进程名 → [ProcessConfig](ProcessConfig.md) 条目。
- **constants** — 已解析的 [ConfigConstants](ConfigConstants.md)，若未指定则使用默认值。
- **errors** — 致命解析错误列表。若非空，配置不应被使用。
- **warnings** — 非致命警告列表（冗余规则、未知值被视为默认值等）。
- **counts** — 常量、别名、组、组成员、进程规则和冗余规则的统计计数。

## 备注

该函数逐行处理配置文件。空行和以 `#` 开头的行视为注释并跳过。解析器识别三类指令，可以任意顺序出现：

### 常量段

以 `@` 开头的行定义调度器行为常量。格式为 `@NAME = value`。已识别的常量通过 [parse_constant](parse_constant.md) 解析：

- `@MIN_ACTIVE_STREAK = 2`
- `@KEEP_THRESHOLD = 0.69`
- `@ENTRY_THRESHOLD = 0.42`

### 别名段

以 `*` 开头的行定义 CPU 别名。格式为 `*name = cpu_spec`。别名通过 [parse_alias](parse_alias.md) 解析，可在后续规则字段中使用 `*name` 语法引用。CPU 规格由 [parse_cpu_spec](parse_cpu_spec.md) 解析。

- `*p = 0-7`
- `*e = 8-19`

### 进程规则

其余行定义单个进程规则或进程组。

**单个规则** 遵循以下格式：

```
name:priority:affinity:cpuset:prime:io_priority:memory_priority:ideal:grade
```

`affinity` 之后的字段均为可选。进程名和规则字段按 `:` 分割，然后委托给 [parse_and_insert_rules](parse_and_insert_rules.md)。

**组规则** 使用花括号定义共享相同规则的多个进程：

```
browsers {
  chrome.exe:
  firefox.exe:
  msedge.exe
}:high:*p:0:0:none:none
```

也支持单行组：`browsers { chrome.exe: firefox.exe }:high:*p:0:0:none:none`

组成员通过 [collect_members](collect_members.md) 提取。多行组通过 [collect_group_block](collect_group_block.md) 收集。关闭 `}:` 后的规则后缀随后由 [parse_and_insert_rules](parse_and_insert_rules.md) 解析。

### CPU Set 重置理想处理器

如果 cpuset 字段以 `@` 为前缀（例如 `@*p`），则生成的 [ProcessConfig](ProcessConfig.md) 上的 `cpu_set_reset_ideal` 标志会被设置为 `true`，这将在应用 CPU Set 后触发线程理想处理器的重新分配。参见 [apply_priority](../apply.rs/apply_priority.md)。

### 错误处理

解析器具有容错性——在单行遇到错误后会继续处理，将所有错误和警告收集到结果中。调用者应在使用已解析配置前检查 `ConfigResult::is_valid()`。

### 配置格式

```
process.exe:priority:affinity:cpuset:prime:io:memory:ideal:grade
```

各字段直接映射到 [ProcessConfig](ProcessConfig.md) 的字段。`affinity` 之后的字段均为可选，默认为无操作值。

### 示例

```
# 常量
@MIN_ACTIVE_STREAK = 3
@KEEP_THRESHOLD = 0.75

# 别名
*p = 0-7
*e = 8-19

# 单个规则
game.exe:high:*p:*p:?8x*p@engine.dll:none:none:*p

# 组规则
browsers { chrome.exe: firefox.exe }:normal:*e:0:0:none:none
```

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/config.rs |
| **签名** | `pub fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult` |
| **调用方** | [main](../main.rs/main.md)、[sort_and_group_config](sort_and_group_config.md) |
| **调用** | [parse_constant](parse_constant.md)、[parse_alias](parse_alias.md)、[collect_members](collect_members.md)、[collect_group_block](collect_group_block.md)、[parse_and_insert_rules](parse_and_insert_rules.md) |

## 另请参阅

- [ConfigResult](ConfigResult.md) — 解析结果容器
- [ProcessConfig](ProcessConfig.md) — 单个进程的配置结构体
- [ConfigConstants](ConfigConstants.md) — 调度器行为常量
- [parse_and_insert_rules](parse_and_insert_rules.md) — 规则字段解析与插入
- [sort_and_group_config](sort_and_group_config.md) — 自动分组相同规则