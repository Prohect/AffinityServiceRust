# Agent Environment Notes

Documents CLI tools and workflows useful for AI agents working on projects.

## General Tool Usage Notes

- `grep` and `find_path` respect `.gitignore` and do not show files or directories that are gitignored (for example `/logs`, `/target`, `/temp`). Symlinks can also be ignored by git on Windows.
- `read_file` can read files that are .gitignored; for very large files it may return a symbol outline — use `start_line`/`end_line` to fetch regions.
- For gitignored files, always refer to project files using the project root prefix, e.g. `AffinityServiceRust/...`, and find the full path before editing — do not guess paths.

## Agent Workflow Tips

### Bulk Refactoring (grep + sed)

For large-scale code changes across multiple files:

1. **Use agent's grep/search tool** to find all occurrences of a pattern
2. **Use `sed`** for batch find/replace in config/data files
3. **Use agent's edit tool** for code files requiring logic changes

#### Example: Reorder CSV fields in 300+ lines

```sh
# Reorder fields: move field 2 to position 5
# Before: name:priority:affinity:cpuset:prime[@regexes]:io:memory
# After:  name:affinity:cpuset:prime[@regexes]:priority:io:memory
sed -i 's/^\([^#@*][^:]*\):\([^:]*\):\([^:]*\):\([^:]*\):\([^:]*\):\([^:]*\):\([^:]*\)$/\1:\3:\4:\5:\2:\6:\7/' config.ini
```

### Finding Rust Function Doc Ranges

To find the full range of a Rust function including its documentation (/// comments), use `awk` to locate the doc start based on Rust's syntax rules. Docs appear immediately after the previous top-level item's closing `}` or `;` and before the function declaration.

#### Example: Find doc start for `parse_cpu_spec`

```sh
cd AffinityServiceRust && awk '/^pub fn parse_cpu_spec/ { print "Doc starts at " (last_end + 1); exit } /^}/ || /;$/ { last_end = NR }' src/config.rs
```

### Shell Escaping Issues

- The terminal runs through `sh` (Git Bash)
- PowerShell `$_` variables get interpreted by sh before reaching PowerShell
- **Prefer sed/perl/awk** over PowerShell for complex one-liners
- For complex PowerShell, write a `.ps1` script file instead

### Git for Quick Restore

Always safe to experiment - restore with:

```sh
git checkout -- file.txt           # Single file
git checkout -- src/ config.ini    # Multiple paths
```
