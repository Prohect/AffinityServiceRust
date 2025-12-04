# AffinityServiceRust Debug Guide

Quick reference for debugging this Windows process affinity/priority management service.

## Quick Debug Command

```bash
cargo run --release -- -console -noUAC -logloop -loop 3 -interval 2000 -config test.ini
```

## Command Line Arguments

| Argument | Description | Default |
|----------|-------------|---------|
| `-console` | Output to console instead of log file | Log to file |
| `-noUAC` | Skip UAC elevation request | Request elevation |
| `-logloop` | Log message at start of each loop | No loop logging |
| `-loop <n>` | Run only n loops then exit | Infinite |
| `-interval <ms>` | Check interval in milliseconds (min: 16) | 5000 |
| `-config <file>` | Use custom config file | config.ini |
| `-blacklist <file>` | Use custom blacklist file | None |
| `-help` | Show basic help | - |
| `-help-all` | Show extended help with all options | - |

## Test Configuration (test.ini)

A minimal test config is provided at `test.ini`:

```ini
# Test the program itself
AffinityServiceRust.exe,normal,0,0,0,normal,low

# Test common processes
notepad.exe,normal,0,0,0,normal,low
```

### Config Format

```
process_name,priority,affinity,cpuset,prime_cpus,io_priority,memory_priority
```

- **priority**: `none`, `idle`, `below normal`, `normal`, `above normal`, `high`, `real time`
- **affinity**: Hex mask (e.g., `0xFF`) or CPU range (e.g., `0-7;16-23`)
- **cpuset**: Same format as affinity, for CPU sets
- **prime_cpus**: CPUs for prime thread scheduling
- **io_priority**: `none`, `very low`, `low`, `normal`
- **memory_priority**: `none`, `very low`, `low`, `normal`

Use `0` or `none` to skip setting that attribute.

## Common Debug Scenarios

### Test Memory Priority Fix

```bash
cargo run --release -- -console -noUAC -logloop -loop 3 -interval 2000 -config test.ini
```

Expected output on success:
```
[HH:MM:SS]12345-affinityservicerust.exe -> Memory: low
```

### Test with Admin Privileges

Run without `-noUAC` to request elevation:
```bash
cargo run --release -- -console -logloop -loop 3 -interval 2000 -config test.ini
```

### Single Loop Test

```bash
cargo run --release -- -console -noUAC -loop 1 -config test.ini
```

### Fast Iteration Test

```bash
cargo run --release -- -console -noUAC -logloop -loop 10 -interval 500 -config test.ini
```

## Error Codes

Errors are logged with format `[ERROR_TYPE][0x{code}]`:

| Code | Meaning |
|------|---------|
| `0x00000005` | ACCESS_DENIED - Need admin or process is protected |
| `0x00000057` | INVALID_PARAMETER - Wrong struct size or invalid value |
| `0x00000006` | INVALID_HANDLE - Process handle issue |

## Known Issues

1. **ACCESS_DENIED for SYSTEM processes**: Processes running as SYSTEM (e.g., from Task Scheduler) cannot be modified by non-elevated processes. This is expected behavior.

2. **Multiple instances**: If another instance is running with higher privileges, you'll see ACCESS_DENIED when trying to modify it.

## Project Structure

- `src/main.rs` - Main loop and `apply_config()` function
- `src/config.rs` - Config parsing, CPU spec parsing
- `src/priority.rs` - Priority enums (`ProcessPriority`, `IOPriority`, `MemoryPriority`)
- `src/logging.rs` - Logging functions and macros
- `src/process.rs` - Process snapshot and enumeration
- `src/scheduler.rs` - Prime thread scheduler
- `src/winapi.rs` - Windows API helpers, NTDLL imports
- `src/cli.rs` - Argument parsing

## Running Tests

```bash
cargo test
```

Unit tests are in `src/config.rs`:
- `test_parse_cpu_spec_hex`
- `test_parse_cpu_spec_ranges`
- `test_parse_cpu_spec_individual`
- `test_parse_cpu_spec_mixed`
- `test_parse_cpu_spec_empty`
- `test_cpu_indices_to_mask`
- `test_format_cpu_indices`
