# Project-Specific Agent Notes

Documents project-specific CLI tools and workflows useful for AI agents working on AffinityServiceRust.

> **Important:** When running AffinityServiceRust via the terminal tool, always use `-console` and `-noUAC` to see output directly. Without `-console`, output goes to log files only. Except when the project needs admin elevation, you will need to use PowerShell to directly run the project (not via cargo) without `-noUAC` flag, this requires user to check UAC.
>
> ```sh
> cargo run --release -- -console -noUAC -validate -config config.ini
> ```

## Repository-Specific Tool Usage Notes

Important notes for agents when using those tools in this repository:
- check .gitignore before reading files. For reference, the current .gitignore contents are:
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

Cargo index symlink (project-specific)
- There is a symbolic link in the src directory named `index.crates.io` that points to a local Cargo index directory (for example: `C:\Users\FSOS\.cargo\registry\src`). This exposes local crate source under `AffinityServiceRust/src/index.crates.io/...`.
- Because some built-in search tools respect .gitignore or git symlink rules, you may not find these files with `find_path`/`grep` in the panel. Recommended ways to access the crate source:
  - Use the terminal with an explicit path to search the symlinked index (this follows the filesystem regardless of git ignore rules). Example:
    cd AffinityServiceRust && grep -nR --color=never "PATTERN" src/index.crates.io || true
  - Or use `read_file` with the exact path to the file (for example `AffinityServiceRust/src/index.crates.io/index.crates.io-1949cf8c6b5b557f/ntapi-0.4.1/src/ntexapi.rs`) — `read_file` can access .gitignored files and symlinked content.
- When giving paths to tools, always start paths with one of the repository root directories (e.g., `AffinityServiceRust/src/index.crates.io/...`).

Summary of best practices
- See [src_outline.md](src_outline.md) for a quick outline of the src code structure.
- Use the panel's built-in tools for quick lookups and diagnostics.
- For searching crate sources under `index.crates.io`, prefer terminal grep or `read_file` with explicit paths.
- Always find and confirm the full file path before making edits; use `read_file`'s outline to identify line ranges (or cli regex tools) for large files.
- Use `temp/` for temporary files (e.g., OCR outputs, preprocessed images) to keep the repository clean, as it is gitignored.
- Logs are stored in `logs/` directory by default for `-processlogs` mode.

## Recommended CLI Tools

The following tools enhance agent capabilities for bulk editing and automation:

- **Important:** Use [scripts/generate_outline.sh](scripts/generate_outline.sh) to generate the src code outline. This script provides a comprehensive overview of the project's structure and is essential for understanding the codebase. Usage: `./scripts/generate_outline.sh > src_outline.md`
- See [docs/sed-perl-awk.md](docs/sed-perl-awk.md) for detailed usage of sed, perl, and awk.

## Additional Tools

- See [docs/tesseract.md](docs/tesseract.md) for detailed Tesseract OCR usage and preprocessing.
- See [docs/imagemagick.md](docs/imagemagick.md) for detailed ImageMagick usage.
- See [docs/es.md](docs/es.md) for detailed es (Everything CLI) usage. For exact filename matches, use `es -r "^filename$"` with regex anchors.
