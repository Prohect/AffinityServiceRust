# SNAPSHOT_BUFFER 静态变量 (process.rs)

`ProcessSnapshot` 的全局缓冲区，存储为 `Lazy<Mutex<Vec<u8>>>`。

## 语法

```rust
pub static SNAPSHOT_BUFFER: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(vec![0u8; 32]));
```

## 备注

此缓冲区在循环迭代间共享以避免重复堆分配。按需单调增长。仅通过 `ProcessSnapshot::take()` 访问——不要直接使用。

## 另请参阅

- [ProcessSnapshot](ProcessSnapshot.md)
- [PID_TO_PROCESS_MAP](PID_TO_PROCESS_MAP.md)