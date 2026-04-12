# PrimePrefix 结构体 (config.rs)

模块特定前缀规则，用于主线程调度。将线程模块名前缀与可选的专用 CPU 核心及可选的线程优先级覆盖相关联。

## 语法

```rust
#[derive(Debug, Clone)]
pub struct PrimePrefix {
    pub prefix: String,
    pub cpus: Option<Vec<u32>>,
    pub thread_priority: ThreadPriority,
}
```

## 参数

`prefix`

用于匹配线程起始模块名的字符串。当为空字符串时，该规则适用于所有线程，不区分其起始模块。当非空时，仅当线程解析后的起始地址模块名包含此前缀时才会被匹配。

`cpus`

可选的 CPU 索引列表，指定匹配此前缀的线程应调度到的处理器。当值为 `Some` 时，这些 CPU 将覆盖进程级别的 `prime_threads_cpus` 设置。当值为 `None` 时，使用进程级别的主线程 CPU 列表。

`thread_priority`

[ThreadPriority](../priority.rs/ThreadPriority.md) 值，当匹配此前缀的线程被主线程调度器提升时应用此优先级。当设置为 `ThreadPriority::None` 时，不应用优先级提升（自动提升行为）。

## 返回值

不适用（结构体定义）。

## 备注

`PrimePrefix` 在 [ProcessConfig](ProcessConfig.md)`::`[prime_threads_prefixes](ProcessConfig.md) 中使用，可对哪些线程接受主线程调度以及被分配到哪些处理器进行细粒度控制。

### 解析格式

主线程前缀规则在配置规则的 prime 字段中指定，使用如下语法：

`*alias@module1[!priority];module2[!priority]`

其中：
- `*alias` 引用一个定义了目标 CPU 的 CPU 别名
- `@module1` 是模块前缀过滤器
- `!priority` 是可选的线程优先级后缀（例如 `!highest`、`!above normal`）
- 多个前缀之间用 `;` 分隔

多个别名-前缀组可以串联：

`*p@engine.dll!highest;render.dll*e@helper.dll`

### 示例

规则 `*p@engine.dll!highest;audio.dll` 会生成两个 `PrimePrefix` 条目：
1. `PrimePrefix { prefix: "engine.dll", cpus: Some([0,1,2,3]), thread_priority: Highest }`
2. `PrimePrefix { prefix: "audio.dll", cpus: Some([0,1,2,3]), thread_priority: None }`

当 `prefix` 为空（无 `@` 过滤）时，会创建一个匹配所有线程的默认条目：

`PrimePrefix { prefix: "", cpus: None, thread_priority: None }`

此默认条目匹配进程中的所有线程。

### 线程匹配

在 [apply_prime_threads](../apply.rs/apply_prime_threads.md) 执行期间，每个候选线程的起始地址通过 `resolve_address_to_module` 解析为模块名。调度器随后按顺序检查每个 `PrimePrefix`，找到匹配的前缀。首个匹配项决定该线程的 CPU 集合和优先级。

## 要求

| 要求 | 值 |
| --- | --- |
| **所属模块** | src/config.rs |
| **使用方** | [ProcessConfig](ProcessConfig.md)::`prime_threads_prefixes` |
| **解析方** | [parse_and_insert_rules](parse_and_insert_rules.md) |
| **消费方** | [apply_prime_threads](../apply.rs/apply_prime_threads.md)、[apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |