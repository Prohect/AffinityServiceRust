# Main Module Documentation

Application entry point and main loop.

## Overview

This module implements:
- CLI argument parsing delegation
- Privilege management
- Configuration file monitoring and hot-reload
- Main service loop with grade-based scheduling
- Process discovery ([`-find` mode](#-find-flag))
- Utility modes ([`-convert`](#-convert), [`-validate`](#-validate), [`-processlogs`](#-processlogs))

## Called By

- Operating system - Program entry point
- User - Direct execution with various CLI flags

## Data Structures

### Module Imports

```rust
mod apply;      // Config application
mod cli;        // CLI parsing
mod config;     // Config file parsing
mod error_codes;// Error translation
mod logging;    // Logging infrastructure
mod priority;   // Priority enums
mod process;    // Process enumeration
mod scheduler;  // Prime thread scheduler
mod winapi;     // Windows API wrappers
```

## Entry Point

### main

Application entry point.

```rust
fn main() -> windows::core::Result<()>
```

**Flow:**
1. Parse CLI arguments ([`parse_args()`](cli.md#parse_args))
2. Handle help modes (`-help`, `-helpall`)
3. Handle utility modes ([`-convert`](#-convert), [`-autogroup`](#-autogroup))
4. Load configuration ([`read_config()`](config.md#config-file-format))
5. Enable privileges ([`enable_debug_privilege()`](winapi.md#enable_debug_privilege), [`enable_inc_base_priority_privilege()`](winapi.md#enable_inc_base_priority_privilege))
6. Set timer resolution (if specified)
7. Request UAC elevation ([`request_uac_elevation()`](winapi.md#request_uac_elevation))
8. Cleanup child processes ([`terminate_child_processes()`](winapi.md#terminate_child_processes))
9. Initialize prime thread scheduler
10. **Main loop:**
    - Take process snapshot ([`ProcessSnapshot::take()`](process.md#processsnapshottake) in [process.rs](process.md))
    - Apply configs by grade
    - Handle [`-find` mode](#-find-flag)
    - Sleep for interval
    - Check for config file changes (hot reload)

## Main Loop

### Grade-Based Scheduling

Rules are organized by "grade" - how frequently they're applied:

```rust
for (grade, grade_configs) in &configs {
    if !current_loop.is_multiple_of(*grade) {
        continue; // Skip this grade this loop
    }
    // Apply configs for this grade...
}
```

**Grade Values:**
| Grade | Frequency | Use Case |
|-------|-----------|----------|
| 1 | Every loop | Critical processes (games) |
| 2 | Every 2nd loop | Semi-critical |
| 5 | Every 5th loop | Background utilities |
| 10 | Every 10th loop | Rarely changing |

### Config Application

For each process matching a config:

```rust
let result = apply_config(pid, config, &mut scheduler, &mut processes, cli.dry_run);
```

**Result Processing:**
- Errors logged to `.find.log` via [`log_to_find()`](logging.md#log_to_find)
- Changes logged to main log with formatting:
  ```
  [HH:MM:SS] 12345::process.exe::Change 1
             Change 2
             Change 3
  ```

### Dry Run Mode

With `-dryrun` flag:
- Changes calculated but not applied
- Report shown: `[DRY RUN] N change(s) would be made`
- Exits after one iteration

### Loop Control

**Termination Conditions:**
- `-loop <count>` reached
- Dry run completed
- Error (logged, continues on most errors)

**Interval:** Configurable via `-interval <ms>` (minimum 16ms)

## Configuration Hot Reload

### File Monitoring

Each loop checks modification times:

```rust
if metadata(&config_file)?.modified()? != last_config_mod_time {
    // Reload config...
}
```

### Reload Process

1. Log change detection
2. Parse new config
3. If valid:
   - Update `configs` HashMap
   - Update scheduler constants
   - Log success
4. If invalid:
   - Log errors
   - Keep previous configuration

### Blacklist Reload

Similar monitoring for optional blacklist file.

## Process Discovery Mode

### -find Flag

When enabled, scans for unmanaged processes:

```rust
if cli.find_mode {
    // Enumerate all processes via ToolHelp32
    for each process {
        if !in_configs && !in_blacklist && is_affinity_unset(pid, name) {
            log_process_find(&name);
        }
    }
}
```

**Criteria:**
- Process name not in any config
- Process name not in blacklist
- Process has default system affinity

**Output:** `logs/YYYYMMDD.find.log`

**Deduplication:** Each process logged once per session via [`FINDS_SET`](logging.md#finds_set)

## Utility Modes

### -validate

Validates config syntax without running:

```rust
if cli.validate_mode {
    config_result.print_report();
    return Ok(());
}
```

Outputs errors/warnings to console.

### -convert

Converts Process Lasso config format:

```rust
if cli.convert_mode {
    convert(cli.in_file_name, cli.out_file_name);
    return Ok(());
}
```

See [config.md](config.md#process-lasso-conversion) for conversion details.

### -autogroup

Groups identical rules:

```rust
if cli.autogroup_mode {
    sort_and_group_config(cli.in_file_name, cli.out_file_name);
    return Ok(());
}
```

See [config.md](config.md#auto-grouping) for grouping algorithm.

### -processlogs

Processes `.find.log` files to discover new processes:

```rust
if cli.process_logs_mode {
    process_logs(&configs, &blacklist, cli.in_file_name.as_deref(), cli.out_file_name.as_deref());
    return Ok(());
}
```

**Algorithm:**
1. Scan `logs/*.find.log` files
2. Extract discovered process names
3. Filter out known configs and blacklist
4. Search for executable paths via `es.exe` (Everything)
5. Write results to file for manual review

## apply_config Function

Applies all configuration to a single process.

```rust
fn apply_config(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process: &mut ProcessEntry,
    prime_core_scheduler: &mut PrimeThreadScheduler,
) -> ApplyConfigResult
```

**Parameters:**
- `config`: [`ProcessConfig`](config.md#processconfig)
- `prime_core_scheduler`: [`PrimeThreadScheduler`](scheduler.md#primethreadscheduler)
- `process`: [`ProcessEntry`](process.md#processentry)
- Returns: [`ApplyConfigResult`](apply.md#applyconfigresult)

**Operations (in order):**
1. **Priority** - [`apply_priority()`](apply.md#apply_priority)
2. **Affinity** - [`apply_affinity()`](apply.md#apply_affinity)
3. **CPU Sets** - [`apply_process_default_cpuset()`](apply.md#apply_process_default_cpuset)
4. **I/O Priority** - [`apply_io_priority()`](apply.md#apply_io_priority)
5. **Memory Priority** - [`apply_memory_priority()`](apply.md#apply_memory_priority)
6. **Prime Scheduling** (if configured):
   - Drop module cache ([`drop_module_cache()`](winapi.md#drop_module_cache))
   - Set alive in scheduler ([`set_alive()`](scheduler.md#reset_alive--set_alive))
   - Prefetch cycles ([`prefetch_all_thread_cycles()`](apply.md#prefetch_all_thread_cycles))
   - Apply prime threads ([`apply_prime_threads()`](apply.md#apply_prime_threads) in [apply.rs](apply.md))
   - Apply ideal processors ([`apply_ideal_processors()`](apply.md#apply_ideal_processors))
   - Update thread stats ([`update_thread_stats()`](apply.md#update_thread_stats))

**Early Exit:** If process handle cannot be obtained, returns empty result immediately.

## Privilege Management

### Requested Privileges

1. **SeDebugPrivilege** - Access to protected processes (for thread start addresses)
2. **SeIncreaseBasePriorityPrivilege** - I/O priority "high" setting

### Flags to Disable

| Flag | Effect |
|------|--------|
| `-noDebugPriv` | Don't request `SeDebugPrivilege` |
| `-noIncBasePriority` | Don't request `SeIncreaseBasePriorityPrivilege` |
| `-noUAC` | Don't request elevation |

### UAC Elevation

If not admin and `-noUAC` not set:

```rust
match request_uac_elevation(*use_console().lock().unwrap()) {
    Ok(_) => { /* Process exits here, elevated instance takes over */ }
    Err(e) => { log!("Failed to elevate: {}", e); }
}
```

**Note:** Original process exits; elevated copy spawned via PowerShell.

## Timer Resolution

Optional system timer resolution adjustment:

```rust
if cli.time_resolution != 0 {
    NtSetTimerResolution(cli.time_resolution, true, &mut current_resolution);
}
```

**Units:** 100-nanosecond intervals (10000 = 1ms)

**Warning:** Very low values (<1ms) may impact system stability.

## Dependencies

- All submodules (`apply`, `cli`, `config`, etc.)
- `chrono::Local` - Timestamp for logging
- `encoding_rs` - Character encoding for `-processlogs`
- `std::collections` - HashMap for configs
- `std::fs` - Config file monitoring
- `std::thread` - Sleep between loops
- `windows` - Win32 API (ToolHelp for `-find`)

## Performance Characteristics

| Operation | Cost | Notes |
|-----------|------|-------|
| Process snapshot | O(P + T) | Single syscall |
| Config application | O(C × O_a) | C=configs, O_a=application cost |
| Prime scheduling | O(T_p log T_p) | Sorting threads |
| Hot reload check | O(1) | Metadata check |
| Find mode | O(P) | Additional iteration |

**Typical Loop:** from 1.6ms to 10ms depending on thread counts and mainly configuration.
