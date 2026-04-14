# PID_TO_PROCESS_MAP static (process.rs)

Global lazily-initialized map from process ID (`u32`) to [`ProcessEntry`](ProcessEntry.md), protected by a `Mutex`. This static stores the parsed results of the most recent process snapshot taken by [`ProcessSnapshot::take`](ProcessSnapshot.md#take). It is an implementation detail of the snapshot infrastructure and **must not be accessed directly**; use [`ProcessSnapshot`](ProcessSnapshot.md) instead.

## Syntax

```rust
pub static PID_TO_PROCESS_MAP: Lazy<Mutex<HashMap<u32, ProcessEntry>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
```

## Type

`once_cell::sync::Lazy<std::sync::Mutex<std::collections::HashMap<u32, ProcessEntry>>>`

## Remarks

`PID_TO_PROCESS_MAP` is the backing store that [`ProcessSnapshot::take`](ProcessSnapshot.md#take) populates when it parses `SYSTEM_PROCESS_INFORMATION` records returned by `NtQuerySystemInformation`. Each key is a Windows process ID and each value is a [`ProcessEntry`](ProcessEntry.md) containing the corresponding `SYSTEM_PROCESS_INFORMATION` structure and lazily-parsed thread data.

### Why direct access is prohibited

The map's contents are only valid while the [`ProcessSnapshot`](ProcessSnapshot.md) that populated them is alive. When the snapshot is dropped, both the map and the underlying [`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md) are cleared. Any `ProcessEntry` obtained after the drop contains dangling internal pointers (the `threads_base_ptr` field points into the freed buffer). Accessing the map outside the snapshot lifetime is **undefined behavior**.

Always acquire the data through `ProcessSnapshot`:

```rust
let mut buf = SNAPSHOT_BUFFER.lock().unwrap();
let mut map = PID_TO_PROCESS_MAP.lock().unwrap();
let snapshot = ProcessSnapshot::take(&mut buf, &mut map)?;
// Use snapshot.pid_to_process safely here
// snapshot is dropped at end of scope, clearing both statics
```

### Thread safety

The `Mutex` ensures exclusive access. Because [`ProcessSnapshot::take`](ProcessSnapshot.md#take) requires `&mut` references to both the buffer and the map, only one snapshot can exist at a time.

## Requirements

| &nbsp; | &nbsp; |
|---|---|
| **Module** | `process` (`src/process.rs`) |
| **Crate dependencies** | `once_cell`, `ntapi` |
| **Initialized by** | [`ProcessSnapshot::take`](ProcessSnapshot.md#take) |
| **Cleared by** | [`ProcessSnapshot::drop`](ProcessSnapshot.md) |
| **Privileges** | None (initialization only; snapshot capture requires `SeDebugPrivilege` indirectly) |

## See Also

| Topic | Description |
|---|---|
| [`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md) | Global buffer backing `NtQuerySystemInformation` data |
| [`ProcessSnapshot`](ProcessSnapshot.md) | RAII wrapper that manages both statics safely |
| [`ProcessEntry`](ProcessEntry.md) | Per-process record stored as the map value |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd