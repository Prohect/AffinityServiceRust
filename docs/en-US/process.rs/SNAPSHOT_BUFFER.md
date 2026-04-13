# SNAPSHOT_BUFFER static (process.rs)

Global buffer for `ProcessSnapshot`, stored as a `Lazy<Mutex<Vec<u8>>>`.

## Syntax

```rust
pub static SNAPSHOT_BUFFER: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(vec![0u8; 32]));
```

## Remarks

This buffer is shared across loop iterations to avoid repeated heap allocations. It grows monotonically as needed. Access through `ProcessSnapshot::take()` only — do not use directly.

## See also

- [ProcessSnapshot](ProcessSnapshot.md)
- [PID_TO_PROCESS_MAP](PID_TO_PROCESS_MAP.md)