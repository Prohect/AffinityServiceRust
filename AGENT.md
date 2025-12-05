# Agent Environment Notes

This file documents CLI tools and workflows useful for AI agents (like Zed's Agent Panel, Cursor, etc.) working on this project.

## Recommended CLI Tools

The following tools enhance agent capabilities for bulk editing and automation:

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

### Helix Editor

**Version:** 25.07.1  
**Installed via:** winget

#### Notes

- TUI editor (like Vim) - requires interactive terminal
- Cannot be used programmatically via the agent's terminal tool
- User must run manually: `hx filename.txt`

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