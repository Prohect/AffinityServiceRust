# AffinityServiceRust Documentation

This directory contains comprehensive documentation for the AffinityServiceRust codebase.

## Structure

Documentation files parallel the source structure:

| Doc File | Source File | Description |
|----------|-------------|-------------|
| [main.md](main.md) | `src/main.rs` | Entry point and main loop |
| [cli.md](cli.md) | `src/cli.rs` | CLI parsing and help text |
| [config.md](config.md) | `src/config.rs` | Config parsing and utilities |
| [apply.md](apply.md) | `src/apply.rs` | Config application logic |
| [scheduler.md](scheduler.md) | `src/scheduler.rs` | Prime thread scheduler |
| [process.md](process.md) | `src/process.rs` | Process enumeration |
| [priority.md](priority.md) | `src/priority.rs` | Priority level definitions |
| [winapi.md](winapi.md) | `src/winapi.rs` | Windows API wrappers |
| [logging.md](logging.md) | `src/logging.rs` | Logging and error tracking |
| [error_codes.md](error_codes.md) | `src/error_codes.rs` | Error code translations |

## Documentation Format

Each file includes:

- **Overview** - Module purpose
- **Called By** - Caller relationships
- **Data Structures** - Important types with fields
- **Functions** - Detailed function documentation including:
  - Purpose
  - Parameters
  - Return values
  - Examples
  - Called by
- **Dependencies** - Required modules/crates

## Quick Reference

### For Users

- **Configuration Format:** See [cli.md](cli.md#configuration) and [config.md](config.md)
- **CPU Specification:** See [config.md](config.md#cpu-specification-parsing)
- **Prime Thread Scheduling:** See [scheduler.md](scheduler.md)
- **Priority Levels:** See [priority.md](priority.md)

### For Developers

- **Architecture Overview:** See [main.md](main.md#main-loop)
- **Adding New Settings:** See [apply.md](apply.md#apply-functions)
- **Error Handling:** See [logging.md](logging.md#error-deduplication)
- **Windows API Integration:** See [winapi.md](winapi.md)

## Doc Comments in Source

Essential documentation remains in source code as `///` doc comments:

- Algorithm explanations
- Safety notes
- Inline examples needed for context
- Complex logic descriptions

User-facing documentation (help text, config format guides) has been moved to this directory.
