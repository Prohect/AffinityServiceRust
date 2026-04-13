# AffinityServiceRust Documentation

This directory contains comprehensive documentation for the AffinityServiceRust codebase.

Each source module (`src/*.rs`) is documented as a directory, with every top-level item (function, struct, enum, static) in its own markdown file following an MSDN-like schema.

## Structure

| Directory | Source File | Description |
|-----------|-------------|-------------|
| [main.rs/](main.rs/) | `src/main.rs` | Entry point, main loop, and orchestration |
| [cli.rs/](cli.rs/) | `src/cli.rs` | CLI argument parsing and help text |
| [config.rs/](config.rs/) | `src/config.rs` | Config parsing, CPU specs, and utilities |
| [apply.rs/](apply.rs/) | `src/apply.rs` | Config application logic |
| [scheduler.rs/](scheduler.rs/) | `src/scheduler.rs` | Prime thread scheduler with hysteresis |
| [process.rs/](process.rs/) | `src/process.rs` | Process and thread enumeration |
| [event_trace.rs/](event_trace.rs/) | `src/event_trace.rs` | ETW process monitoring |
| [priority.rs/](priority.rs/) | `src/priority.rs` | Priority level definitions |
| [winapi.rs/](winapi.rs/) | `src/winapi.rs` | Windows API wrappers |
| [logging.rs/](logging.rs/) | `src/logging.rs` | Logging and error tracking |
| [error_codes.rs/](error_codes.rs/) | `src/error_codes.rs` | Error code translations |

## Documentation Schema

Each item file follows this structure:

| Section | Description |
|---------|-------------|
| **Title** | `# ItemName type (module.rs)` |
| **Short description** | One-paragraph summary |
| **Syntax** | Rust code block with full signature |
| **Parameters** | Per-parameter description (functions) |
| **Members** | Per-field description (structs/enums) |
| **Return value** | What the function returns |
| **Remarks** | Algorithms, examples, edge cases, platform notes |
| **Requirements** | Table of module, callers, callees, Windows API, privileges |

Cross-references between items use relative markdown links:

```
[ProcessConfig](config.rs/ProcessConfig.md)
[apply_priority](apply.rs/apply_priority.md)
[PrimeThreadScheduler](scheduler.rs/PrimeThreadScheduler.md)
```

## Quick Reference

### For Users

- **Configuration Format:** See [read_config](config.rs/read_config.md) and [parse_cpu_spec](config.rs/parse_cpu_spec.md)
- **CLI Options:** See [CliArgs](cli.rs/CliArgs.md) and [parse_args](cli.rs/parse_args.md)
- **Priority Levels:** See [ProcessPriority](priority.rs/ProcessPriority.md), [IOPriority](priority.rs/IOPriority.md), [MemoryPriority](priority.rs/MemoryPriority.md)
- **Prime Thread Scheduling:** See [PrimeThreadScheduler](scheduler.rs/PrimeThreadScheduler.md)

### For Developers

- **Main Loop:** See [main](main.rs/main.md), [apply_config_process_level](main.rs/apply_config_process_level.md), and [apply_config_thread_level](main.rs/apply_config_thread_level.md)
- **Adding New Settings:** See [apply.rs/](apply.rs/) for the apply function pattern
- **Error Handling:** See [is_new_error](logging.rs/is_new_error.md) and [Operation](logging.rs/Operation.md)
- **Windows API Integration:** See [winapi.rs/](winapi.rs/) for handle management and CPU set operations
- **Process Enumeration:** See [ProcessSnapshot](process.rs/ProcessSnapshot.md)
- **ETW Process Monitoring:** See [event_trace.rs/](event_trace.rs/) for reactive process detection