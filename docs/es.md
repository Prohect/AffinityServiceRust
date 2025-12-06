# es (Everything CLI)

**Purpose:** Fast file and folder search on Windows drives.  
**Source:** CLI component of the "Everything" search tool by voidtools.  
**Download:** https://www.voidtools.com/downloads/#cli (es.exe, portable).

## Installation

1. Download `es.exe` from the link above.
2. Place it in a directory in your PATH (e.g., `C:\Users\%USERNAME%\bin`) or run with full path.

## Usage

Basic search for files/folders by name:
```
es "filename"
```

### Common Options

- `-i` - Case-insensitive search.
- `-r` - Enable regex (Perl-compatible).
- `-p` - Output paths only (no size/date info).
- `-n <num>` - Limit results to <num> items.
- `-s` - Include subfolders (default).
- `-d` - Search directories only.
- `-f` - Search files only.
- `-path "<path>"` - Search within specific path.
- `-size <op><size>` - Filter by size (e.g., `-size >1MB`).

### Examples

- Exact match: `es "discordproxystart.exe"`
- Case-insensitive: `es -i "config.ini"`
- Regex: `es -r "discord.*\.exe"`
- Paths only: `es -p "cs2.exe"`
- In specific folder: `es -path "C:\Program Files" "notepad.exe"`
- Large files: `es -size >100MB`

## Notes

- Extremely fast due to real-time indexing.
- Supports wildcards (*, ?) without -r.
- For full help: `es /?`
- Requires "Everything" service running (install the full Everything tool if not).
```
