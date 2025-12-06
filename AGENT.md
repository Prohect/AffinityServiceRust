# Agent Environment Notes

This file documents CLI tools and workflows useful for AI agents (like Zed's Agent Panel, Cursor, etc.) working on this project.

> **Important:** When running AffinityServiceRust via the terminal tool, always use `-console` and `-noUAC` to see output directly. Without `-console`, output goes to log files only. Except the project needs admin elevation, you will need to use PowerShell to run the project with out `-noUAC` flag, this requires user to check UAC.
>
> ```sh
> cargo run --release -- -console -noUAC -validate -config config.ini
> ```

## Agent's Built-in Tools

The Zed Agent Panel provides several built-in tools. Here's how they work in this project:

### grep (Built-in)

The agent has a built-in `grep` tool that returns **line numbers** with matches, which is useful for locating code before editing.

#### Features

- Returns line number ranges like `L126-130` for each match
- Shows surrounding context lines
- Supports regex patterns
- Can filter by file glob pattern with `include_pattern`

#### Example Output

```
Found 2 matches:

## Matches in AffinityServiceRust\config.ini

### L126-130
# ==================== PROCESS GROUPS ====================

# Windows system processes (low IO)
windows {
    # text input

### L298-306
# code language rust's code analyzer
rust-analyzer.exe,none,*e,0,0,low,none
# windows 错误报告收集器 - very low IO
wermgr.exe,none,*e,0,0,very low,none
```

#### Workflow

1. Use agent's grep to find patterns and get line numbers
2. Use `read_file` with `start_line`/`end_line` to see more context
3. Use `edit_file` to make targeted changes

#### Limitations: .gitignore Respected

The agent's `grep` and `find_path` tools **respect `.gitignore`** - they won't search folders like `/logs`, `/target`, `/temp` that are listed in `.gitignore`.

**To search log files for debugging**, use the terminal with standard grep:

```sh
# Search today's log file
grep "pattern" logs/20251205.log

# Search all log files
grep -r "AdjustTokenPrivileges" logs/

# Search with context lines
grep -B2 -A2 "ERROR" logs/*.log
```

Alternatively, use `read_file` directly if you know the log file path:
- The agent can read `.gitignore`'d files with `read_file`
- Only `grep` and `find_path` respect `.gitignore`

### read_file (Built-in)

For large code files, `read_file` returns a **file outline** with symbol names and line numbers instead of full content. This acts as an outline/symbol view.

Example outline output:
```
pub struct ProcessConfig [L27-38]
 pub name [L28]
 pub priority [L29]
pub fn parse_cpu_spec [L83-122]
fn mask_to_cpu_indices [L124-126]
```

Use `start_line`/`end_line` parameters to read specific sections.

### diagnostics (Built-in)

Get compiler errors and warnings. Invoke after edits to check for issues:
- With path: shows all diagnostics for that file
- Without path: shows project-wide summary

## Recommended CLI Tools

The following tools enhance agent capabilities for bulk editing and automation:

### sed / perl / awk (via Git for Windows)

**Purpose:** Regex find/replace for text files  
**Source:** Bundled with Git for Windows (available in Git Bash or any shell after Git install)

#### sed (Stream Editor)

```sh
# Find lines matching pattern
sed -n '/pattern/p' file.txt

# Replace in-place
sed -i 's/old/new/g' file.txt

# Replace with capture groups
sed -i 's/^\(.*\.exe,.*\)$/\1,suffix/' file.txt

# Delete lines matching pattern
sed -i '/pattern/d' file.txt
```

#### perl (One-liner regex)

```sh
# Replace in-place (best regex support)
perl -i -pe 's/old/new/g' file.txt

# With UTF-8 support
perl -i -CSD -pe 's/old/new/g' file.txt

# Multi-line patterns
perl -i -0pe 's/start.*?end/replacement/gs' file.txt
```

#### awk (Pattern processing)

```sh
# Print lines matching pattern
awk '/pattern/' file.txt

# Replace and print
awk '{gsub(/old/, "new"); print}' file.txt > output.txt

# Conditional processing
awk -F',' '/\.exe,/ {print $0 ",testNone"; next} {print}' file.txt
```

#### Notes

- Available via Git Bash (included with Git for Windows)
- `sed -i` modifies files in-place
- `perl` has the most powerful regex (PCRE)
- Prefer these over PowerShell for complex regex (avoids escaping issues)

## Agent Workflow Tips

### Bulk Refactoring (grep + sed)

For large-scale code changes across multiple files:

1. **Use agent's grep/search tool** to find all occurrences of a pattern
2. **Use `sed`** for batch find/replace in config/data files
3. **Use agent's edit tool** for code files requiring logic changes

#### Example: Reorder CSV fields in 300+ lines

```sh
# Reorder fields: move field 2 to position 5
# Before: name,priority,affinity,cpuset,prime,io,memory
# After:  name,affinity,cpuset,prime,priority,io,memory
sed -i 's/^\([^#@*][^,]*\),\([^,]*\),\([^,]*\),\([^,]*\),\([^,]*\),\([^,]*\),\([^,]*\)$/\1,\3,\4,\5,\2,\6,\7/' config.ini
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

## Additional Tools

### Tesseract OCR

**Version:** 5.4.0.20240606  
**Installation Path:** `C:\Program Files\Tesseract-OCR\`  
**Installed via:** winget (`UB-Mannheim.TesseractOCR`)

#### Usage

Extract text from an image:

```sh
tesseract image.png output       # outputs to output.txt
tesseract image.png stdout       # outputs directly to console
```

#### Common Options

- `-l <lang>` - Specify language (e.g., `-l eng`, `-l chi_sim` for Simplified Chinese)
- `--psm <n>` - Page segmentation mode (0-13)
- `--oem <n>` - OCR engine mode (0-3)

#### Examples

```sh
# Basic OCR to text file
tesseract screenshot.png result

# Output to console with Chinese language
tesseract image.png stdout -l chi_sim

# PDF output
tesseract document.png output pdf
```

#### Notes

- May need to open a new terminal session for PATH to update after installation
- Additional language packs can be installed separately
