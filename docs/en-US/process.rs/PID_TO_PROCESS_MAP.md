# PID_TO_PROCESS_MAP static (process.rs)

Global process map stored as a `Lazy<Mutex<HashMap<u32, ProcessEntry>>>`.

## Syntax

```rust
pub static PID_TO_PROCESS_MAP: Lazy<Mutex<HashMap<u32, ProcessEntry>>> = Lazy::new(|| Mutex::new(HashMap::new()));
```

## Remarks

Stores the parsed process entries from each snapshot. The map is cleared and repopulated on each `ProcessSnapshot::take()` call. Previously owned by `ProcessSnapshot`, now a shared static to allow the snapshot data to persist across the lifetime of a loop iteration while being safely shared.

`ProcessEntry` implements `Send` (unsafe impl) since it is only accessed through the Mutex, ensuring single-threaded access. The raw pointers inside are only valid for the lifetime of the snapshot buffer.

## See also

- [ProcessSnapshot](ProcessSnapshot.md)
- [SNAPSHOT_BUFFER](SNAPSHOT_BUFFER.md)
- [ProcessEntry](ProcessEntry.md)