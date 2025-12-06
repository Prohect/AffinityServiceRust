# Agent Environment Notes

This file documents CLI tools and workflows useful for AI agents (like Zed's Agent Panel, Cursor, etc.) working on this project.

> **Important:** When running AffinityServiceRust via the terminal tool, always use `-console` and `-noUAC` to see output directly. Without `-console`, output goes to log files only. Except the project needs admin elevation, you will need to use PowerShell to directly run the project (not via cargo) with out `-noUAC` flag, this requires user to check UAC.
>
> ```sh
> cargo run --release -- -console -noUAC -validate -config config.ini
> ```

## Agent's Built-in Tools (brief)

The Zed Agent Panel provides built‑in tools such as grep, read_file, and diagnostics in its UI. Do not duplicate the panel's full help here — use the panel for exact behavior and outputs.

Important notes for agents when using those tools in this repository:
- `grep` and `find_path` respect `.gitignore` and may not show files or directories that are ignored (for example `/logs`, `/target`, `/temp`). Symlinks can be ignored by git on Windows.
- `read_file` can read files that are .gitignored; for very large files it may return a symbol outline — use `start_line`/`end_line` to fetch regions.
- Always refer to project files using the project root prefix, e.g. `AffinityServiceRust/...`, and find the full path before editing — do not guess paths.

Cargo index symlink (project-specific)
- There is a symbolic link in the project root named `index.crates.io` that points to a local Cargo index directory (for example: `C:\Users\FSOS\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f`). This exposes local crate source under `AffinityServiceRust/index.crates.io/...`.
- Because some built-in search tools respect .gitignore or git symlink rules, you may not find these files with `find_path`/`grep` in the panel. Recommended ways to access the crate source:
  - Use the terminal with an explicit path to search the symlinked index (this follows the filesystem regardless of git ignore rules). Example:
    cd AffinityServiceRust && grep -nR --color=never "PATTERN" index.crates.io || true
  - Or use `read_file` with the exact path to the file (for example `AffinityServiceRust/index.crates.io/ntapi-0.4.1/src/ntexapi.rs`) — `read_file` can access .gitignored files and symlinked content.
- When giving paths to tools, always start paths with one of the repository root directories (e.g., `AffinityServiceRust/index.crates.io/...`).

Summary of best practices
- Use the panel's built-in tools for quick lookups and diagnostics.
- For searching crate sources under `index.crates.io`, prefer terminal grep or `read_file` with explicit paths.
- Always find and confirm the full file path before making edits; use `read_file`'s outline to identify line ranges for large files.

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
