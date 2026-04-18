# Logging module statics (logging.rs)

This page documents all global static variables defined in the `logging` module. These statics manage log file handles, console mode flags, time buffers, and failure-tracking data structures. Each static is lazily initialized via `once_cell::sync::Lazy` and protected by `std::sync::Mutex` for thread-safe access.

> **Note:** Each static has a corresponding convenience macro (e.g., `get_logger!()`) that locks the mutex and returns the guard. Prefer the macros over direct `.lock().unwrap()` calls for consistency.

---

## FINDS_SET

Tracks process names that have already been logged by [`log_process_find`](log_process_find.md) during the current session, preventing duplicate log entries for the same process name.

### Syntax

```rust
pub static FINDS_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::default()));
```

### Type

`once_cell::sync::Lazy<std::sync::Mutex<HashSet<String>>>`

### Remarks

- Populated by [`log_process_find`](log_process_find.md): if `FINDS_SET.lock().unwrap().insert(process_name)` returns `true`, the process name was not yet seen this session and is logged. If it returns `false`, the entry already exists and logging is skipped.
- The set is never explicitly cleared during runtime. It resets naturally when the process restarts.
- The `HashSet` is the project's [`HashSet`](../collections.rs/HashSet.md) alias (`FxHashSet`), using a fast non-cryptographic hash.

---

## USE_CONSOLE

Controls whether log output is written to stdout (console mode) or to log files (file mode).

### Syntax

```rust
pub static USE_CONSOLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
```

### Type

`once_cell::sync::Lazy<std::sync::Mutex<bool>>`

### Remarks

- Default: `false` (file mode).
- Set to `true` when the application is launched with the `console` CLI flag.
- When `true`, [`log_message`](log_message.md), [`log_pure_message`](log_pure_message.md), and [`log_to_find`](log_to_find.md) all write to `stdout` instead of their respective log files.
- Accessed via the `get_use_console!()` macro.

---

## DUST_BIN_MODE

When enabled, causes [`log_message`](log_message.md) to silently discard all output. This is used during startup phases where logging is intentionally suppressed (e.g., before elevation).

### Syntax

```rust
pub static DUST_BIN_MODE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
```

### Type

`once_cell::sync::Lazy<std::sync::Mutex<bool>>`

### Remarks

- Default: `false` (logging enabled).
- When `true`, `log_message` returns immediately without writing anything.
- Does **not** affect [`log_pure_message`](log_pure_message.md) or [`log_to_find`](log_to_find.md) — only `log_message` checks this flag.
- Accessed via the `get_dust_bin_mod!()` macro.

---

## LOCAL_TIME_BUFFER

Cached `DateTime<Local>` value used by all logging functions for timestamp formatting. The caller is responsible for refreshing this value (by assigning `Local::now()`) at the start of each scheduling cycle so that log timestamps within a cycle share a consistent time without repeatedly querying the system clock.

### Syntax

```rust
pub static LOCAL_TIME_BUFFER: Lazy<Mutex<DateTime<Local>>> = Lazy::new(|| Mutex::new(Local::now()));
```

### Type

`once_cell::sync::Lazy<std::sync::Mutex<chrono::DateTime<chrono::Local>>>`

### Remarks

- Initialized to `Local::now()` on first access.
- Logging functions format timestamps as `HH:MM:SS` via `time.format("%H:%M:%S")`.
- The buffer is also used by [`get_log_path`](get_log_path.md) to determine the current date for daily log file rotation (`YYYYMMDD.log`).
- Accessed via the `get_local_time!()` macro.

---

## LOG_FILE

File handle for the main daily log file. Opened in append-create mode on first access, targeting a path like `logs/YYYYMMDD.log`.

### Syntax

```rust
pub static LOG_FILE: Lazy<Mutex<File>> =
    Lazy::new(|| Mutex::new(
        OpenOptions::new().append(true).create(true).open(get_log_path("")).unwrap()
    ));
```

### Type

`once_cell::sync::Lazy<std::sync::Mutex<std::fs::File>>`

### Remarks

- The file path is computed by [`get_log_path("")`](get_log_path.md) at initialization time, using the date from [`LOCAL_TIME_BUFFER`](#local_time_buffer).
- Opened with `append(true).create(true)`, meaning the file is created if it doesn't exist and all writes go to the end.
- Used by [`log_message`](log_message.md) and [`log_pure_message`](log_pure_message.md) when `USE_CONSOLE` is `false`.
- Accessed via the `get_logger!()` macro.
- The `logs/` directory is created by `get_log_path` if it does not already exist.

---

## FIND_LOG_FILE

File handle for the `.find` daily log file. Opened in append-create mode on first access, targeting a path like `logs/YYYYMMDD.find.log`.

### Syntax

```rust
pub static FIND_LOG_FILE: Lazy<Mutex<File>> =
    Lazy::new(|| Mutex::new(
        OpenOptions::new().append(true).create(true).open(get_log_path(".find")).unwrap()
    ));
```

### Type

`once_cell::sync::Lazy<std::sync::Mutex<std::fs::File>>`

### Remarks

- The file path is computed by [`get_log_path(".find")`](get_log_path.md), producing paths like `logs/YYYYMMDD.find.log`.
- Used by [`log_to_find`](log_to_find.md) and indirectly by [`log_process_find`](log_process_find.md) when `USE_CONSOLE` is `false`.
- Accessed via the `get_logger_find!()` macro.
- Separate from the main log file to allow independent review of process-discovery output.

---

## FINDS_FAIL_SET

Tracks process names that failed access checks during `-find` mode (specifically `ACCESS_DENIED` errors). These processes are excluded from future find-mode iterations to avoid repeated failed attempts and log noise.

### Syntax

```rust
pub static FINDS_FAIL_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::default()));
```

### Type

`once_cell::sync::Lazy<std::sync::Mutex<HashSet<String>>>`

### Remarks

- Populated by [`is_affinity_unset`](../winapi.rs/is_affinity_unset.md) when `OpenProcess` or `GetProcessAffinityMask` returns error code `5` (`ACCESS_DENIED`).
- Consulted by the find-mode logic in the scheduler to skip processes known to be inaccessible (e.g., anti-cheat services, protected processes).
- The `HashSet` is the project's [`HashSet`](../collections.rs/HashSet.md) alias (`FxHashSet`).
- Accessed via the `get_fail_find_set!()` macro.
- Never explicitly cleared during runtime. Resets on process restart.

---

## PID_MAP_FAIL_ENTRY_SET

Per-PID failure tracking map that records which operations have already failed for a given process, preventing redundant error log messages. Each PID maps to a secondary `HashMap` of [`ApplyFailEntry`](ApplyFailEntry.md) keys with `bool` alive flags.

### Syntax

```rust
pub static PID_MAP_FAIL_ENTRY_SET: Lazy<Mutex<HashMap<u32, HashMap<ApplyFailEntry, bool>>>> =
    Lazy::new(|| Mutex::new(HashMap::default()));
```

### Type

`once_cell::sync::Lazy<std::sync::Mutex<HashMap<u32, HashMap<ApplyFailEntry, bool>>>>`

### Remarks

- **Structure**: The outer map is keyed by PID (`u32`). Each value is an inner map keyed by [`ApplyFailEntry`](ApplyFailEntry.md) (a composite of TID, process name, operation, and error code) with a `bool` value indicating whether the entry is "alive" (still relevant to a running process).

- **Populated by**: [`is_new_error`](is_new_error.md), which inserts new failure entries and returns `true` only on the first occurrence of a given failure tuple. If the entry already exists, it marks it as alive and returns `false`.

- **Purged by**: [`purge_fail_map`](purge_fail_map.md), which implements a mark-and-sweep algorithm:
  1. Marks all entries as dead (`alive = false`).
  2. Re-marks entries whose PIDs and process names appear in the list of currently running processes as alive (`alive = true`).
  3. Removes map entries where all entries are dead, preventing unbounded growth.

- **Process name consistency**: The inner map is cleared if a new entry's process name does not match the existing entries' process name for the same PID. This handles PID reuse — when a new process gets the same PID as a terminated one, the stale failure entries are discarded.

- Both the outer and inner `HashMap` types are the project's [`HashMap`](../collections.rs/HashMap.md) alias (`FxHashMap`).

- Accessed via the `get_pid_map_fail_entry_set!()` macro.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `logging.rs` |
| **Dependencies** | `once_cell::sync::Lazy`, `std::sync::Mutex`, `std::fs::File`, `chrono::{DateTime, Local}`, [`HashMap`](../collections.rs/HashMap.md), [`HashSet`](../collections.rs/HashSet.md), [`ApplyFailEntry`](ApplyFailEntry.md) |
| **Platform** | Windows (file paths use Windows conventions) |
| **Initialized** | Lazily on first access |

## See Also

| Topic | Link |
|-------|------|
| is_new_error function | [is_new_error](is_new_error.md) |
| purge_fail_map function | [purge_fail_map](purge_fail_map.md) |
| log_message function | [log_message](log_message.md) |
| log_to_find function | [log_to_find](log_to_find.md) |
| log_process_find function | [log_process_find](log_process_find.md) |
| get_log_path function | [get_log_path](get_log_path.md) |
| ApplyFailEntry struct | [ApplyFailEntry](ApplyFailEntry.md) |
| Operation enum | [Operation](Operation.md) |
| collections module | [collections.rs](../collections.rs/README.md) |
| logging module overview | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
