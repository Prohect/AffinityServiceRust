# Project-Specific Agent Notes

Documents project-specific CLI tools and workflows useful for AI agents working on AffinityServiceRust.

> **Important:** When running AffinityServiceRust via the terminal tool, always use `-console` and `-noUAC` to see output directly. Without `-console`, output goes to log files only. When the project needs admin elevation, use a `.ps1` script with `Start-Process -Verb RunAs` to launch the pre-built binary directly (not via cargo), then read the log file after it exits.
>
> ```sh
> # Non-admin quick test (output to console)
> cargo run --release -- -console -noUAC -logloop -loop 5 -interval 2000 -config test.ini
>
> # Admin test (output to log file — do NOT use -console)
> # Write a .ps1 and run it:
> # $proc = Start-Process -FilePath ".\target\release\AffinityServiceRust.exe" `
> #   -ArgumentList "-logloop -loop 5 -interval 2000 -config test.ini -skip_log_before_elevation" `
> #   -Verb RunAs -PassThru
> # $proc.WaitForExit(20000) | Out-Null
> # Get-Content (Get-ChildItem logs/*.log | Sort-Object LastWriteTime | Select-Object -Last 1).FullName | Select-Object -Last 80
> ```

## Repository-Specific Tool Usage Notes

Important notes for agents when using those tools in this repository:
- Check `.gitignore` before reading files. For reference, the current `.gitignore` contents are:
```AffinityServiceRust/.gitignore#L1-16
/target
/logs
/temp
/Links
/.idea
/.Cargo
/src/index.crates.io
Cargo.lock

test_*.ps1
test_*.bat
*.ini

# Except for the main config and blacklist files
!config.ini
!blacklist.ini
```

- **`test.ini` is gitignored** (matches `*.ini`, not excepted). `find_path` and `grep` will not find it. Use `read_file` with the explicit path `AffinityServiceRust/test.ini` to read it, or the terminal to inspect it.
- Similarly, any `test_*.ps1` or `test_*.bat` helper scripts written during a session are gitignored and won't appear in `find_path`/`grep`. Clean them up with `delete_path` when done.

Cargo index symlink (project-specific)
- There is a symbolic link in the src directory named `index.crates.io` that points to a local Cargo index directory (for example: `C:\Users\FSOS\.cargo\registry\src`). This exposes local crate source under `AffinityServiceRust/src/index.crates.io/...`.
- Because some built-in search tools respect .gitignore or git symlink rules, you may not find these files with `find_path`/`grep` in the panel. Recommended ways to access the crate source:
  - Use the terminal with an explicit path to search the symlinked index (this follows the filesystem regardless of git ignore rules). Example:
    cd AffinityServiceRust && grep -nR --color=never "PATTERN" src/index.crates.io || true
  - Or use `read_file` with the exact path to the file (for example `AffinityServiceRust/src/index.crates.io/index.crates.io-1949cf8c6b5b557f/ntapi-0.4.1/src/ntexapi.rs`) — `read_file` can access .gitignored files and symlinked content.
- When giving paths to tools, always start paths with one of the repository root directories (e.g., `AffinityServiceRust/src/index.crates.io/...`).

## Module Resolution

Thread start address resolution uses `psapi GetMappedFileName` — no dbghelp, no symbol server, no internet access required. The result is always `module.dll+0xOffset` format (e.g., `ntdll.dll+0x92220`). There is no `-proxy` flag and no symbol cache on disk.

## Summary of Best Practices

- See [src_outline.md](src_outline.md) for a quick outline of the src code structure.
- Use the panel's built-in tools for quick lookups and diagnostics.
- For searching crate sources under `index.crates.io`, prefer terminal grep or `read_file` with explicit paths.
- Always find and confirm the full file path before making edits; use `read_file`'s outline to identify line ranges (or CLI regex tools) for large files.
- Use `temp/` for temporary files (e.g., OCR outputs, preprocessed images) to keep the repository clean, as it is gitignored.
- Logs are stored in `logs/` directory and are gitignored — use `read_file` with an explicit path or terminal `type`/`Get-Content` to read them.
- When writing PowerShell one-liners for the terminal, avoid `$variable` syntax — sh intercepts `$` before PowerShell sees it. Write a `.ps1` file instead and run it with `powershell -ExecutionPolicy Bypass -File script.ps1`. Delete it when done.

## Recommended CLI Tools

The following tools enhance agent capabilities for bulk editing and automation:

- **Important:** Use [scripts/generate_outline.sh](scripts/generate_outline.sh) to generate the src code outline. This script provides a comprehensive overview of the project's structure and is essential for understanding the codebase. Usage: `./scripts/generate_outline.sh > src_outline.md`
- See [docs/sed-perl-awk.md](docs/sed-perl-awk.md) for detailed usage of sed, perl, and awk.

## Additional Tools

- See [docs/tesseract.md](docs/tesseract.md) for detailed Tesseract OCR usage and preprocessing.
- See [docs/imagemagick.md](docs/imagemagick.md) for detailed ImageMagick usage.
- See [docs/es.md](docs/es.md) for detailed es (Everything CLI) usage. For exact filename matches, use `es -r "^filename$"` with regex anchors.