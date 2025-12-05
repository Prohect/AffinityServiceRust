# AffinityServiceRust Debug Guide

Quick reference for debugging this Windows process affinity/priority management service.

## Privilege Capability Table

What rules can be applied with different privilege levels:

### Target Process Types

| Target Process | No Admin | Admin | SYSTEM |
|----------------|----------|-------|--------|
| User processes (same user) | ✅ Full access | ✅ Full access | ✅ Full access |
| Elevated user processes | ❌ ACCESS_DENIED | ✅ Full access | ✅ Full access |
| SYSTEM processes (services) | ❌ ACCESS_DENIED | ✅ Full access | ✅ Full access |
| Protected processes (csrss, smss) | ❌ ACCESS_DENIED | ❌ ACCESS_DENIED | ⚠️ Limited |

### Rule Capabilities by Privilege

| Rule | No Admin | Admin | Notes |
|------|----------|-------|-------|
| **Process Priority** | | | |
| ├ idle | ✅ | ✅ | |
| ├ below normal | ✅ | ✅ | |
| ├ normal | ✅ | ✅ | |
| ├ above normal | ✅ | ✅ | |
| ├ high | ✅ | ✅ | |
| └ real time | ✅ | ✅ | Works on user processes |
| **CPU Affinity** | ✅ | ✅ | ≤64 cores only |
| **CPU Set** | ✅ | ✅ | Works on >64 cores |
| **Prime Scheduling** | ✅ | ✅ | Thread-level CPU sets |
| **I/O Priority** | | | |
| ├ very low | ✅ | ✅ | |
| ├ low | ✅ | ✅ | |
| ├ normal | ✅ | ✅ | |
| └ high | ❌ 0xC0000061 | ✅ | Requires admin elevation |
| **Memory Priority** | ✅ | ✅ | All levels work |

### Windows Privileges Effect

Two privileges are requested at startup. Use `-noDebugPriv` and `-noIncBasePriority` flags to test their effects:

| Privilege | Purpose | Without Admin | With Admin |
|-----------|---------|---------------|------------|
| **SeDebugPrivilege** | Access other users' processes | Granted but no effect | Enables cross-user access |
| **SeIncreaseBasePriorityPrivilege** | Set high IO priority | Granted but no effect | Required for IO high |

#### Test Commands

```bash
# Non-admin tests
cargo run --release -- -console -noUAC -config test_privilege.ini -loop 1
cargo run --release -- -console -noUAC -noDebugPriv -config test_privilege.ini -loop 1
cargo run --release -- -console -noUAC -noIncBasePriority -config test_privilege.ini -loop 1
cargo run --release -- -console -noUAC -noDebugPriv -noIncBasePriority -config test_privilege.ini -loop 1

# Admin tests (UAC prompt required, output goes to log file)
powershell -Command "Start-Process -FilePath './target/release/AffinityServiceRust.exe' -ArgumentList '-config test_privilege.ini -loop 1 -logloop' -Verb RunAs -Wait"
powershell -Command "Start-Process -FilePath './target/release/AffinityServiceRust.exe' -ArgumentList '-config test_privilege.ini -loop 1 -logloop -noDebugPriv' -Verb RunAs -Wait"
powershell -Command "Start-Process -FilePath './target/release/AffinityServiceRust.exe' -ArgumentList '-config test_privilege.ini -loop 1 -logloop -noIncBasePriority' -Verb RunAs -Wait"
powershell -Command "Start-Process -FilePath './target/release/AffinityServiceRust.exe' -ArgumentList '-config test_privilege.ini -loop 1 -logloop -noDebugPriv -noIncBasePriority' -Verb RunAs -Wait"
```

#### Test Results

| Scenario | SeDebugPriv | SeIncBasePriority | Same-user | SYSTEM procs | Protected (csrss) | IO High |
|----------|-------------|-------------------|-----------|--------------|-------------------|---------|
| **No admin** | | | | | | |
| Both enabled | ✅ granted | ✅ granted | ✅ | ❌ | ❌ | ❌ |
| -noDebugPriv | ❌ | ✅ granted | ✅ | ❌ | ❌ | ❌ |
| -noIncBasePriority | ✅ granted | ❌ | ✅ | ❌ | ❌ | ❌ |
| Both disabled | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ |
| **Admin** | | | | | | |
| Both enabled | ✅ granted | ✅ granted | ✅ | ✅ | ❌ | ✅ |
| -noDebugPriv | ❌ | ✅ granted | ✅ | ❌ | ❌ | ✅ |
| -noIncBasePriority | ✅ granted | ❌ | ✅ | ✅ | ❌ | ❌ |
| Both disabled | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ |

**Key findings:**
- **Without admin**: Privileges are granted but have **no practical effect**
- **With admin + SeDebugPrivilege**: Enables access to SYSTEM processes (dwm.exe, services, etc.)
- **With admin + SeIncreaseBasePriorityPrivilege**: Enables IO priority `high`
- **Protected processes** (csrss.exe, smss.exe): Always ACCESS_DENIED, even with admin
- **Same-user processes**: Work without any privileges or elevation

### Summary

- **Run without admin**: Can manage same-user processes, but I/O priority `high` fails
- **Run with admin**: Can manage all user processes and services, I/O `high` works
- **Run as SYSTEM**: Maximum compatibility, can manage most processes

## Reading Log Files

Log files are stored in the `logs/` folder with format `YYYYMMDD.log`.

**Note:** The agent's `grep` tool respects `.gitignore`, so it **cannot search** the `logs/` folder directly. Use these alternatives:

### Using Terminal grep

```bash
# Search today's log file
grep "pattern" logs/20251205.log

# Search all log files
grep -r "AdjustTokenPrivileges" logs/

# Search with context lines
grep -B2 -A2 "ERROR" logs/*.log

# Find affinity changes
grep "affinity" logs/20251205.log
```

### Using read_file

The agent can read log files directly with `read_file` - only `grep` and `find_path` respect `.gitignore`:

```
read_file("AffinityServiceRust/logs/20251205.log")
```

### Log File Types

| File Pattern | Description |
|--------------|-------------|
| `YYYYMMDD.log` | Main application log |
| `YYYYMMDD.find.log` | Process discovery log (if enabled) |

## Quick Debug Command

**Non-admin (with console output):**
```bash
cargo run --release -- -console -noUAC -logloop -loop 3 -interval 2000 -config test.ini
```

**Admin elevation (check log file after):**
```bash
cargo run --release -- -logloop -loop 3 -interval 2000 -config test.ini
# Then check: logs/YYYYMMDD.log
# Then check: logs/YYYYMMDD.find.log
```

> **Note:** When running with admin elevation, do NOT use `-console`. The UAC elevation spawns a new process via PowerShell, and the console window closes immediately after execution. Without `-console`, output goes to log files which persist after the process exits.

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
| `-validate` | Validate config file syntax without running | - |
| `-dryrun` | Show what would be changed without applying | - |

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

### Validate Config Syntax

Before running, validate your config file for errors:
```bash
cargo run --release -- -validate -config config.ini
```

Expected output on success:
```
✓ Parsed 3 constants
✓ Parsed 7 CPU aliases
✓ Parsed 311 process rules
✓ Config is valid!
```

### Dry Run Mode

See what changes would be applied without actually making them:
```bash
cargo run --release -- -dryrun -noUAC -config test.ini
```

Example output:
```
[DRY RUN] Checking 115 running processes against 6 config rules...
  notepad.exe (PID 6628):
    - IO Priority: -> normal
    - Memory Priority: -> low
[DRY RUN] 2 change(s) would be made. Run without -dryrun to apply.
```

### Test Memory Priority Fix

```bash
cargo run --release -- -console -noUAC -logloop -loop 3 -interval 2000 -config test.ini
```

Expected output on success:
```
[HH:MM:SS]12345-affinityservicerust.exe -> Memory: low
```

### Test with Admin Privileges

Run without `-noUAC` to request elevation. **Do NOT use `-console`** as the elevated process runs in a separate window that closes immediately:
```bash
cargo run --release -- -logloop -loop 3 -interval 2000 -config test.ini
```

Then check the log file:
```bash
tail -50 logs/$(date +%Y%m%d).log   # Linux/Git Bash
type logs\YYYYMMDD.log              # Windows CMD
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
| `0xC0000061` | STATUS_PRIVILEGE_NOT_HELD - Missing required privilege (need admin) |
| `0xC000000D` | STATUS_INVALID_PARAMETER - Invalid value for this API |

## Priority Levels

### IO Priority

| Level | Value | Status |
|-------|-------|--------|
| `very low` | 0 | ✅ Works |
| `low` | 1 | ✅ Works |
| `normal` | 2 | ✅ Works |
| `high` | 3 | ✅ Works with admin elevation (fails without: STATUS_PRIVILEGE_NOT_HELD) |
| `critical` | 4 | ❌ Reserved for kernel use (STATUS_INVALID_PARAMETER) |

### Memory Priority

| Level | Value | Status |
|-------|-------|--------|
| `very low` | 1 | ✅ Works |
| `low` | 2 | ✅ Works |
| `medium` | 4 | ✅ Works |
| `below normal` | 5 | ✅ Works |
| `normal` | 3 | ✅ Works |

## Known Issues

1. **ACCESS_DENIED for SYSTEM processes**: Processes running as SYSTEM (e.g., from Task Scheduler) cannot be modified by non-elevated processes. This is expected behavior.

2. **Multiple instances**: If another instance is running with higher privileges, you'll see ACCESS_DENIED when trying to modify it.

3. **Console output lost with admin elevation**: When running with `-console` and UAC elevation, the elevated process spawns in a new window via PowerShell which closes immediately. Use log file output instead (omit `-console`).

4. **High IO priority requires admin**: IO priority `high` only works when running as administrator. Without elevation, you'll get `0xC0000061` (STATUS_PRIVILEGE_NOT_HELD).

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
