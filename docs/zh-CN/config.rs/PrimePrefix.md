# PrimePrefix 结构体 (config.rs)

表示一个模块名前缀过滤器，结合可选的 CPU 集合覆盖和线程优先级提升，用于 prime 线程调度。当 prime 线程调度器识别出一个热线程时，会将该线程的起始地址模块名与 `prefix` 字段进行匹配。如果前缀匹配（或为空，表示"匹配所有"），则该线程将被固定到 `cpus` 中指定的 CPU 上，并可选地将优先级提升至 `thread_priority`。

## 语法

```rust
#[derive(Debug, Clone)]
pub struct PrimePrefix {
    pub prefix: String,
    pub cpus: Option<Vec<u32>>,
    pub thread_priority: ThreadPriority,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `prefix` | `String` | 用于按线程起始地址过滤线程的模块名前缀。空字符串匹配所有线程。匹配不区分大小写，与线程起始地址解析出的模块名进行比较（例如 `"engine.dll"`）。 |
| `cpus` | `Option<Vec<u32>>` | 可选的按前缀 CPU 集合覆盖。当为 `Some` 时，匹配的线程将被固定到这些特定的 CPU 索引上，而非父规则的 `prime_threads_cpus`。当为 `None` 时，使用父规则的 CPU 列表。 |
| `thread_priority` | `ThreadPriority` | 应用于匹配的 prime 线程的线程优先级。`ThreadPriority::None` 表示不更改优先级（保留操作系统自动提升行为）。非 `None` 值（例如 `AboveNormal`、`Highest`）通过 `SetThreadPriority` 显式设置线程的调度优先级。 |

## 备注

`PrimePrefix` 条目在 [parse_and_insert_rules](parse_and_insert_rules.md) 的规则解析过程中构造，当 prime 线程字段包含 `@` 分隔的前缀规格时触发。

### 配置语法

prime 线程字段（规则行中的第 4 个字段）支持以下形式的前缀限定规格：

```
*alias@prefix1;prefix2!priority*alias2@prefix3
```

其中：

- `*alias` 引用通过 `*alias = cpu_spec` 定义的 CPU 别名。
- `@prefix1;prefix2` 列出以分号分隔的模块名前缀。
- `!priority` 可选地为特定前缀附加线程优先级（例如 `engine.dll!highest`）。

例如，规格 `*p@engine.dll;render.dll!above normal*e@audio.dll` 将产生三个 `PrimePrefix` 条目：

1. `{ prefix: "engine.dll", cpus: Some(<p cpus>), thread_priority: None }`
2. `{ prefix: "render.dll", cpus: Some(<p cpus>), thread_priority: AboveNormal }`
3. `{ prefix: "audio.dll", cpus: Some(<e cpus>), thread_priority: None }`

当 prime 线程字段中没有 `@` 时，会创建一个具有空前缀和 `cpus: None` 的单个 `PrimePrefix`，表示所有线程都符合条件，并使用父规则的 CPU 列表。

### 线程匹配

在运行时，apply 模块将每个线程的起始地址解析为模块名，并使用不区分大小写的前缀匹配与 `prefix` 字段进行比较。空 `prefix` 充当通配符。

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **构造者** | [parse_and_insert_rules](parse_and_insert_rules.md) |
| **消费者** | [apply_prime_threads](../apply.rs/apply_prime_threads.md)、[apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |
| **父结构体** | [ProcessConfig](ProcessConfig.md)（字段 `prime_threads_prefixes`） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 每进程配置记录 | [ProcessConfig](ProcessConfig.md) |
| 规则解析和前缀提取 | [parse_and_insert_rules](parse_and_insert_rules.md) |
| Prime 线程应用逻辑 | [apply_prime_threads](../apply.rs/apply_prime_threads.md) |
| 线程优先级枚举 | [ThreadPriority](../priority.rs/ThreadPriority.md) |
| CPU 别名解析 | [parse_alias](parse_alias.md) |