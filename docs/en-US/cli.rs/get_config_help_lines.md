# get_config_help_lines function (cli.rs)

Returns a vector of static string slices containing the complete configuration file reference template. This template documents the INI-style configuration format including terminology, field descriptions, CPU specification formats, priority levels, ideal processor syntax, and process group syntax. The returned lines are suitable for embedding at the top of converted configuration files or printing to the console.

## Syntax

```AffinityServiceRust/src/cli.rs#L168-170
pub fn get_config_help_lines() -> Vec<&'static str> {
    vec![
        r#"
```

## Parameters

None.

## Return value

Type: `Vec<&'static str>`

A vector containing one or more static string slices. Each slice is a multi-line block of comment-prefixed documentation text (lines begin with `##`). The content covers:

| Section | Description |
|---------|-------------|
| **Terminology** | Definitions of P-core, E-core, `p`, `pp`, `e` shorthand for Intel hybrid CPUs. |
| **Config Format** | The full field order: `process_name:priority:affinity:cpuset:prime_cpus[@prefixes]:io_priority:memory_priority:ideal_processor:grade`. |
| **CPU Specification Formats** | All supported formats including ranges (`0-7`), individual CPUs (`0;4;8`), hex bitmasks (`0xFF`), and alias references (`*alias`). |
| **Priority Levels** | Valid values for process priority, I/O priority, and memory priority fields. |
| **Ideal Processor Syntax** | Format for `*alias[@prefix1;prefix2;...]` with multi-segment chaining examples. |
| **Process Groups** | Named and anonymous `{ }` group syntax for applying a single rule to multiple processes. |

## Remarks

- The returned strings use `##` as the comment prefix (double hash), which is the comment syntax recognized by the AffinityServiceRust configuration parser (lines starting with `#` are comments).
- This function is called by [convert](../config.rs/convert.md) to prepend a help header to converted Process Lasso configuration files, and by [print_config_help](print_config_help.md) to display the reference on the console.
- The content is compiled into the binary as `&'static str` literals, so there is no file I/O or allocation beyond the `Vec` itself.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `cli` |
| Callers | [print_config_help](print_config_help.md), [print_help_all](print_help_all.md), [convert](../config.rs/convert.md) |
| Callees | None |
| API | None |
| Privileges | None |

## See Also

| Resource | Link |
|----------|------|
| print_config_help | [print_config_help](print_config_help.md) |
| print_help_all | [print_help_all](print_help_all.md) |
| convert | [convert](../config.rs/convert.md) |
| read_config | [read_config](../config.rs/read_config.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
