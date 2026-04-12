# Config Module Documentation

Configuration file parsing, CPU specification handling, and config manipulation utilities.

## Overview

This module provides:
- INI-style config file parsing
- CPU specification parsing (ranges, masks, aliases)
- Process group handling
- Config conversion from Process Lasso format
- Auto-grouping of identical rules

## Called By

- [main.rs](main.md) - Main config loading via `read_config()`
- CLI `-convert` mode - Process Lasso conversion
- CLI `-autogroup` mode - Rule grouping optimization

## Data Structures

### ProcessConfig

Complete configuration for a single process.

```rust
pub struct ProcessConfig {
    pub name: String,                           // Process executable name
    pub priority: ProcessPriority,              // Process priority class
    pub affinity_cpus: Vec<u32>,                // Hard affinity CPU list
    pub cpu_set_cpus: Vec<u32>,                 // CPU Set CPU list
    pub cpu_set_reset_ideal: bool,              // Reset ideal processors after CPU set change
    pub prime_threads_cpus: Vec<u32>,           // Prime scheduling CPUs
    pub prime_threads_prefixes: Vec<PrimePrefix>, // Module-specific rules
    pub track_top_x_threads: i32,               // Track top N threads (0=off, >0=track, <0=track only)
    pub io_priority: IOPriority,                // I/O priority
    pub memory_priority: MemoryPriority,        // Memory priority
    pub ideal_processor_rules: Vec<IdealProcessorRule>, // Ideal processor assignments
}
```

**Type References:**
- `priority`: [`ProcessPriority`](priority.md#processpriority)
- `io_priority`: [`IOPriority`](priority.md#iopriority)
- `memory_priority`: [`MemoryPriority`](priority.md#memorypriority)
- `prime_threads_prefixes`: [`PrimePrefix`](#primeprefix)
- `ideal_processor_rules`: [`IdealProcessorRule`](#idealprocessorrule)

### PrimePrefix

Module-specific prefix rule for prime thread scheduling.

```rust
pub struct PrimePrefix {
    pub prefix: String,              // Module name prefix to match
    pub cpus: Option<Vec<u32>>,      // Specific CPUs for this prefix (None = use prime_threads_cpus)
    pub thread_priority: ThreadPriority, // Thread priority to apply
}
```

Used in [`ProcessConfig::prime_threads_prefixes`](#processconfig) for per-module CPU and priority assignments.

**Type References:**
- `thread_priority`: [`ThreadPriority`](priority.md#threadpriority)

### IdealProcessorRule

Rule for assigning ideal processors to threads based on module prefix.

```rust
pub struct IdealProcessorRule {
    pub cpus: Vec<u32>,              // CPU indices for ideal processor assignment
    pub prefixes: Vec<String>,       // Module name prefixes to match (empty = all threads)
}
```

Used in [`ProcessConfig::ideal_processor_rules`](#processconfig). When `prefixes` is empty, the rule applies to all threads.

### IdealProcessorPrefix

Module prefix with specific CPUs (internal helper for parsing).

```rust
pub struct IdealProcessorPrefix {
    pub prefix: String,              // Module name prefix
    pub cpus: Vec<u32>,              // CPU indices for this prefix
}
```

### ConfigConstants

Scheduler behavior constants.

```rust
pub struct ConfigConstants {
    pub min_active_streak: u8,      // Consecutive intervals before promotion (default: 2)
    pub keep_threshold: f64,        // Fraction to stay prime (default: 0.69)
    pub entry_threshold: f64,       // Fraction to become candidate (default: 0.42)
}
```

### ConfigResult

Result of config parsing with statistics and errors.

```rust
pub struct ConfigResult {
    pub configs: HashMap<u32, HashMap<String, ProcessConfig>>, // grade -> (name -> config)
    pub constants: ConfigConstants,
    pub constants_count: usize,
    pub aliases_count: usize,
    pub groups_count: usize,
    pub group_members_count: usize,
    pub process_rules_count: usize,
    pub redundant_rules_count: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}
```

## CPU Specification Parsing

### parse_cpu_spec

Parses CPU specification strings into sorted CPU index vectors.

**Supported Formats:**
- `"0"` or empty → Empty vec (no change)
- `"0xFF"` → Hex bitmask (legacy, ≤64 cores)
- `"0-7"` → CPU range inclusive
- `"0;4;8"` → Individual CPUs separated by semicolons
- `"0-7;64-71"` → Multiple ranges for >64 core systems

**Examples:**
```rust
parse_cpu_spec("0-3")     // → [0, 1, 2, 3]
parse_cpu_spec("0;2;4")   // → [0, 2, 4]
parse_cpu_spec("0x0F")    // → [0, 1, 2, 3]
parse_cpu_spec("")        // → []
parse_cpu_spec("0")       // → []
```

**Called By:**
- [`read_config()`](config.md#config-file-format) - Main config loading function
- `resolve_cpu_spec()` (internal) - Alias resolution
- [`convert()`](config.md#process-lasso-conversion) - Process Lasso conversion

### cpu_indices_to_mask

Converts CPU indices to bitmask (for ≤64 core systems).

```rust
pub fn cpu_indices_to_mask(cpus: &[u32]) -> usize
```

**Example:**
```rust
cpu_indices_to_mask(&[0, 1, 2, 3])  // → 0x0F
```

### format_cpu_indices

Formats CPU indices as compact string (ranges when possible).

```rust
pub fn format_cpu_indices(cpus: &[u32]) -> String
```

**Examples:**
```rust
format_cpu_indices(&[0, 1, 2, 3])   // → "0-3"
format_cpu_indices(&[0, 2, 4])      // → "0,2,4"
format_cpu_indices(&[])             // → "0"
```

## Ideal Processor Parsing

### parse_ideal_processor_spec

Parses ideal processor specification with module prefix filtering.

**Format:** `*alias[@prefix1;prefix2]`

- `*` is a required prefix marker for each rule segment
- `alias` is a CPU alias name (must be defined in ALIAS section)
- `@prefix` optionally filters threads by their start module name

**Multi-segment:** `*p@engine.dll*e@helper.dll`

**Example:**
```rust
// Simple
parse_ideal_processor_spec("*pN01", line_num, &aliases, &mut errors)
// → [IdealProcessorRule { cpus: [2,3,4,5,6,7], prefixes: [] }]

// With module filter
parse_ideal_processor_spec("*pN01@cs2.exe;nvwgf2umx.dll", ...)
// → [IdealProcessorRule { cpus: [2,3,4,5,6,7], prefixes: ["cs2.exe", "nvwgf2umx.dll"] }]
```

**Called By:** `parse_and_insert_rules()` (internal) - When parsing rule field 6 or 7

## Prime Thread Specification Parsing

The `prime_cpus` field supports advanced syntax:

```
[?[?]x]*alias[@module1[!priority];module2[!priority]*alias2@module3...]
```

**Components:**
- `?x*cpus` - Track top x threads, apply rules, log on exit
- `??x*cpus` - Monitor only: track and log on exit, don't apply CPU sets
- `*alias@module1;module2` - Only affect threads from specified modules
- `*alias1@mod1*alias2@mod2` - Multi-segment: different CPUs per module
- `module!priority` - Set explicit thread priority
- `module` - Auto-boost (current priority + 1 tier)

**Examples:**
```ini
# Track top 10 threads on P-cores (except 0-1)
game.exe:normal:*a:*p:?10*pN01:normal:normal:1

# Multi-segment: CS2 on P-cores, NVIDIA on E-cores
cs2.exe:normal:*a:*p:*p@cs2.exe*e@nvwgf2umx.dll:normal:normal:1

# Per-module thread priorities
cs2.exe:normal:*a:*p:*pN01@cs2.exe!time critical;nvwgf2umx.dll!above normal:normal:normal:1

# Monitor only (no apply): track top 20 threads
game.exe:normal:*a:*p:??20*pN01:normal:normal:1
```

## Config File Format

### Constants Section

```ini
@MIN_ACTIVE_STREAK = 2    # Consecutive intervals before promotion
@ENTRY_THRESHOLD = 0.42   # Fraction of max cycles to become candidate
@KEEP_THRESHOLD = 0.69    # Fraction of max cycles to stay prime
```

### Aliases Section

```ini
*a = 0-19           # All cores
*p = 0-7            # P-cores
*e = 8-19           # E-cores
*pN01 = 2-7         # P-cores except 0-1
```

Aliases support all CPU specification formats including multiple ranges for >64 core systems.

### Rules Section

**Basic rule:**
```ini
process.exe:priority:affinity:cpuset:prime:io:memory:ideal:grade
```

**CPU Set Reset Ideal:**

Prefix the cpuset field with `@` to enable resetting thread ideal processors after applying the CPU set:
```ini
# After setting CPU set to 0-3, redistribute thread ideal processors across CPUs 0-3
game.exe:normal:*a:@0-3:*p:normal:normal:1
```

This prevents Windows from clamping threads to narrow CPU ranges after CPU set changes.

**Examples:**
```ini
# Simple rule (grade defaults to 1)
cs2.exe:normal:*a:*p:*pN01:normal:normal

# With ideal processor and explicit grade
background.exe:normal:*a:*p:*p:normal:normal:*p:5

# CPU set with ideal processor reset
game.exe:normal:*a:@0-3:*p:normal:normal:1

# Process group
browsers { chrome.exe: firefox.exe: msedge.exe }:normal:*e:0:0:low:below normal:1
```

## Process Lasso Conversion

The `convert()` function converts Process Lasso INI format to AffinityServiceRust format.

**Parsed Fields:**
- `NamedAffinities=alias,cpus,alias,cpus...` → `*alias = cpus`
- `DefaultPriorities=process,priority,process,priority...` → priority field
- `DefaultAffinitiesEx=process,mask,cpuset,process,mask,cpuset...` → affinity/cpuset fields

**Called By:** CLI `-convert` mode

**Example:**
```bash
AffinityServiceRust.exe -convert -in prolasso.ini -out my_config.ini
```

## Auto-Grouping

The `sort_and_group_config()` function merges rules with identical settings into named groups.

**Algorithm:**
1. Parse existing config preserving preamble (comments, constants, aliases)
2. Collect all rules and group by identical rule strings
3. Create named groups (`grp_0`, `grp_1`, etc.) for rules with multiple processes
4. Format single-line groups if under 128 characters, otherwise multi-line

**Output Format:**
```ini
# Input:
explorer.exe:none:*a:*e:0:none:none:0:4
cmd.exe:none:*a:*e:0:none:none:0:4
notepad.exe:none:*a:*e:0:none:none:0:4

# Output:
grp_0 { cmd.exe: explorer.exe: notepad.exe }:none:*a:*e:0:none:none:0:4
```

**Called By:** CLI `-autogroup` mode

**Example:**
```bash
AffinityServiceRust.exe -autogroup -in config.ini -out config_grouped.ini
```

## Error Handling

Config parsing collects errors and warnings without failing immediately:

- **Errors:** Invalid syntax, undefined aliases, unclosed groups
- **Warnings:** Unknown priority levels, empty aliases, redundant rules

Use `ConfigResult::print_report()` to display parse results.

## Helper Functions

### read_list

Reads a simple list file (blacklist, etc.).

```rust
pub fn read_list<P: AsRef<Path>>(path: P) -> Result<Vec<String>>
```

Filters: empty lines, comments (lines starting with `#`)

### read_utf16le_file

Reads UTF-16 LE encoded files (Process Lasso configs).

```rust
pub fn read_utf16le_file(path: &str) -> Result<String>
```

## Dependencies

- `crate::cli::get_config_help_lines` - For conversion output template
- `crate::log` - Logging macro
- `crate::logging` - Error logging utilities
- `crate::priority` - Priority enum definitions
- `std::collections` - HashMap for configs and aliases
- `std::fs` - File I/O
