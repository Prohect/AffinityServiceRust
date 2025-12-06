# es (Everything CLI)

**Purpose:** Fast file and folder search on Windows drives.  
**Source:** CLI component of the "Everything" search tool by voidtools.  
**Download:** https://www.voidtools.com/downloads/#cli (es.exe, portable).  
**Version:** 1.1.0.30 (based on `es --help` output).

## Installation

1. Download `es.exe` from the link above.
2. Place it in a directory in your PATH (e.g., `C:\Users\%USERNAME%\bin`) or run with full path.
3. Requires the "Everything" service running (install the full Everything tool if not).

## Usage

Basic search for files/folders by name:
```
es.exe [options] search text
```

ES uses the Everything search syntax. Example: `es Everything ext:exe;ini`

### Search Options

- `-r <search>`, `-regex <search>`: Search using regular expressions (Perl-compatible).
- `-i`, `-case`: Match case.
- `-w`, `-ww`, `-whole-word`, `-whole-words`: Match whole words.
- `-p`, `-match-path`: Match full path and file name.
- `-a[RHSDAVNTPLCOIE]`: DIR style attributes search.
  - R = Read only
  - H = Hidden
  - S = System
  - D = Directory
  - A = Archive
  - V = Device
  - N = Normal
  - T = Temporary
  - P = Sparse file
  - L = Reparse point
  - C = Compressed
  - O = Offline
  - I = Not content indexed
  - E = Encrypted
  - Prefix a flag with `-` to exclude (e.g., `-a-H` for not hidden).

### Sort Options

- `-s`: Sort by full path.
- `-sort <name[-ascending|-descending]>`, `-sort-<name>[-ascending|-descending]`: Set sort by name (options: name, path, size, extension, date-created, date-modified, date-accessed, attributes, file-list-file-name, run-count, date-recently-changed, date-run).
- `-sort-ascending`, `-sort-descending`: Set sort order.
- `/on`, `/o-n`, `/os`, `/o-s`, `/oe`, `/o-e`, `/od`, `/o-d`: DIR style sorts (N=Name, S=Size, E=Extension, D=Date modified; `-` for descending).

### Display Options

- `-name`: Show name column.
- `-path-column`: Show path column.
- `-full-path-and-name`, `-filename-column`: Show full path and name.
- `-extension`, `-ext`: Show extension.
- `-size`: Show size.
- `-date-created`, `-dc`: Show date created.
- `-date-modified`, `-dm`: Show date modified.
- `-date-accessed`, `-da`: Show date accessed.
- `-attributes`, `-attribs`, `-attrib`: Show attributes.
- `-file-list-file-name`: Show file list filename.
- `-run-count`: Show run count.
- `-date-run`: Show date run.
- `-date-recently-changed`, `-rc`: Show date recently changed.
- `-highlight`: Highlight results.
- `-highlight-color <color>`: Highlight color (0-255).
- `-csv`, `-efu`, `-txt`, `-m3u`, `-m3u8`, `-tsv`: Change display format.
- `-size-format <format>`: Size format (0=auto, 1=Bytes, 2=KB, 3=MB).
- `-date-format <format>`: Date format (0=auto, 1=ISO-8601, 2=FILETIME, 3=ISO-8601(UTC)).
- `-filename-color <color>`, `-name-color <color>`, `-path-color <color>`, `-extension-color <color>`, `-size-color <color>`, `-date-created-color <color>`, `-dc-color <color>`, `-date-modified-color <color>`, `-dm-color <color>`, `-date-accessed-color <color>`, `-da-color <color>`, `-attributes-color <color>`, `-file-list-filename-color <color>`, `-run-count-color <color>`, `-date-run-color <color>`, `-date-recently-changed-color <color>`, `-rc-color <color>`: Set column color (0-255).
- `-filename-width <width>`, `-name-width <width>`, `-path-width <width>`, `-extension-width <width>`, `-size-width <width>`, `-date-created-width <width>`, `-dc-width <width>`, `-date-modified-width <width>`, `-dm-width <width>`, `-date-accessed-width <width>`, `-da-width <width>`, `-attributes-width <width>`, `-file-list-filename-width <width>`, `-run-count-width <width>`, `-date-run-width <width>`, `-date-recently-changed-width <width>`, `-rc-width <width>`: Set column width (0-200).
- `-no-digit-grouping`: Don't group numbers with commas.
- `-size-leading-zero`, `-run-count-leading-zero`: Format numbers with leading zeros (use with `-no-digit-grouping`).
- `-double-quote`: Wrap paths and filenames with double quotes.

### Export Options

- `-export-csv <out.csv>`, `-export-efu <out.efu>`, `-export-txt <out.txt>`, `-export-m3u <out.m3u>`, `-export-m3u8 <out.m3u8>`, `-export-tsv <out.txt>`: Export to file.
- `-no-header`: Do not output column header for CSV, EFU, TSV.
- `-utf8-bom`: Store UTF-8 byte order mark at start of exported file.

### General Options

- `-h`, `-help`: Display this help.
- `-instance <name>`: Connect to unique Everything instance name.
- `-ipc1`, `-ipc2`: Use IPC version 1 or 2.
- `-pause`, `-more`: Pause after each page of output.
- `-hide-empty-search-results`: Don't show results when no search.
- `-empty-search-help`: Show help when no search specified.
- `-timeout <milliseconds>`: Timeout in milliseconds to wait for database load.
- `-set-run-count <filename> <count>`: Set run count for filename.
- `-inc-run-count <filename>`: Increment run count for filename by one.
- `-get-run-count <filename>`: Display run count for filename.
- `-get-result-count`: Display result count for search.
- `-get-total-size`: Display total result size for search.
- `-save-settings`, `-clear-settings`: Save or clear settings.
- `-version`: Display ES version and exit.
- `-get-everything-version`: Display Everything version and exit.
- `-exit`: Exit Everything (returns after process closes).
- `-save-db`: Save Everything database to disk (returns after completion).
- `-reindex`: Force Everything to reindex (returns after completion).
- `-no-result-error`: Set error level if no results found.

### Common Options (from original doc)

- `-n <num>`, `-max-results <num>`: Limit results to <num>.
- `-path "<path>"`: Search within specific path.
- `-size <op><size>`: Filter by size (e.g., `-size >1MB`).
- `-d`, `-ad`: Directories only.
- `-f`, `-a-d`: Files only.

## Examples

- Exact match: `es "discordproxystart.exe"`
- Case-insensitive: `es -i "config.ini"`
- Regex: `es -r "discord.*\.exe"`
- Paths only: `es -p "cs2.exe"`
- In specific folder: `es -path "C:\Program Files" "notepad.exe"`
- Large files: `es -size >100MB`
- Match start of filename: `es -r "^discord"`
- Match end of filename: `es -r "\.exe$"`
- Whole words: `es -w "exe"`
- Export to CSV: `es -export-csv results.csv "steam.exe"`
- Get result count: `es -get-result-count "notepad.exe"`

## Notes

- Extremely fast due to real-time indexing.
- Supports wildcards (*, ?) without `-r`.
- For exact filename matches, use regex with anchors: `es -r "^filename$"`
- Internal `-` in options can be omitted (e.g., `-nodigitgrouping`).
- Switches can start with `/`.
- Use double quotes to escape spaces and switches.
- Switches can be disabled by prefixing with `no-` (e.g., `-no-size`).
- Use `^` prefix or wrap with double quotes to escape `\ & | > < ^`.
- Typical installation paths:
  - Everything.exe: Often in `C:\Program Files\Everything\` or `E:\Programme Files\Everything\`.
  - es.exe: Portable, commonly in user PATH like `E:\Programme Files\commandLineInterface\` or `C:\Users\%USERNAME%\bin\`.
- For full help: `es /?` or `es -h`.