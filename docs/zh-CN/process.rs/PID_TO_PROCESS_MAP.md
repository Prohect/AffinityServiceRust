# PID_TO_PROCESS_MAP 静态变量 (process.rs)

全局进程映射表，存储为 `Lazy<Mutex<HashMap<u32, ProcessEntry>>>`。

## 语法

```rust
pub static PID_TO_PROCESS_MAP: Lazy<Mutex<HashMap<u32, ProcessEntry>>> = Lazy::new(|| Mutex::new(HashMap::new()));
```

## 备注

存储每次快照解析的进程条目。每次 `ProcessSnapshot::take()` 调用时清空并重新填充。之前由 `ProcessSnapshot` 拥有，现改为共享静态变量以允许快照数据在循环迭代生命周期内安全共享。

`ProcessEntry` 实现了 `Send`（unsafe impl），因为它仅通过 Mutex 访问，确保单线程访问。内部的原始指针仅在快照缓冲区的生命周期内有效。

## 另请参阅

- [ProcessSnapshot](ProcessSnapshot.md)
- [SNAPSHOT_BUFFER](SNAPSHOT_BUFFER.md)
- [ProcessEntry](ProcessEntry.md)