# ddgr: DuckDuckGo from the Command Line

`ddgr` is a command-line tool to search DuckDuckGo from the terminal. It is fast, lightweight, and respects your privacy.

## Basic Usage

Search for keywords:
```sh
ddgr "rust thread affinity"
```

Limit the number of results (e.g., 3 results):
```sh
ddgr -n 3 "core_affinity crate"
```

## Advanced Search

### Search within a specific site
Use the `-w` (site) flag to restrict results to a domain:
```sh
ddgr -w docs.rs "available_parallelism"
ddgr -w github.com "rust-lang affinity"
```

### Time-limited results
Restrict results to a specific time period (e.g., past month):
```sh
ddgr -t m "rust 1.80 features"
```
Options: `d` (day), `w` (week), `m` (month), `y` (year).

### Interactive Mode
Launch `ddgr` without arguments to enter an interactive shell:
```sh
ddgr
```
Common interactive commands:
- `n`: Next page of results
- `p`: Previous page
- `f [index]`: Focus/open a specific result (usually opens in system browser)
- `q`: Quit

## Why use ddgr?
- **Avoid "Packet Too Large" errors**: Unlike some integrated AI fetch tools, `ddgr` uses standard HTTP requests and returns text, avoiding many API-related size limitations.
- **Privacy**: Does not track your searches like a standard browser might.
- **Speed**: Extremely low overhead compared to a full web browser.

## Integration with other tools
You can pipe `ddgr` output to other CLI utilities:
```sh
ddgr -n 5 "rust affinity" | grep "docs.rs"
```
