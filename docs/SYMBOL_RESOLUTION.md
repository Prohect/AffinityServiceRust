# Symbol Resolution Feature

## Overview

AffinityServiceRust now automatically resolves thread start addresses to human-readable module names using Microsoft's symbol server. This helps identify which DLLs or modules threads belong to when tracking prime threads.

## How It Works

When prime thread tracking is enabled (`?x` or `??x` syntax), the service:

1. **Initializes Symbol Handler**: Connects to Microsoft's symbol server for each tracked process
2. **Downloads PDB Files**: Automatically downloads debug symbols to `C:\Symbols` directory
3. **Resolves Addresses**: Converts raw memory addresses to `module.dll+offset` format
4. **Caches Symbols**: Reuses downloaded PDB files for faster subsequent lookups

## Example Output

### Before (Without Symbol Resolution)
```
Thread 12345 -> (promoted, [0], cycles=1549316906, start=0x7FFE12345678)
```

### After (With Symbol Resolution)
```
Thread 12345 -> (promoted, [0], cycles=1549316906, start=ucrtbase.dll+0x3780)
```

## Configuration

### Basic Usage

No configuration needed - symbol resolution is automatic when prime thread tracking is enabled.

```ini
# Track top 5 threads - symbols automatically resolved
notepad.exe:normal:*a:*p:?5*p:normal:normal
```

### Proxy Support

For corporate networks requiring HTTP proxy for internet access:

```bash
AffinityServiceRust.exe -config config.ini -proxy http://proxy.example.com:8080
```

The proxy URL is passed to the symbol server for PDB downloads.

## Symbol Cache

### Location
- **Fixed path**: `C:\Symbols\`
- **Created automatically**: Directory is created if it doesn't exist
- **Persistent**: PDB files are cached permanently for reuse

### Disk Usage
- Typical PDB sizes: 100KB - 50MB per module
- Common modules (ntdll, kernel32, etc.): ~1-5MB each
- Complex applications (games, browsers): 50-200MB total
- Budget 500MB-1GB for extensive symbol caching

### Cache Structure
```
C:\Symbols\
├── ntdll.pdb\
│   └── F97E6A620FC3F1F2C1BCB4A66E68A1A81\
│       └── ntdll.pdb
├── kernel32.pdb\
│   └── 9632DB2D10FAB25F3C4E5D6F7A8B9C0D1\
│       └── kernel32.pdb
└── ...
```

Files are organized by module name and GUID for version tracking.

## Requirements

### Minimum Requirements
- **Internet access**: To download symbols from Microsoft's symbol server
- **Disk space**: ~500MB recommended for symbol cache
- **Windows**: Symbol resolution uses Windows DbgHelp.dll

### Optional Requirements
- **Admin privileges**: Recommended for full access to system process threads
- **HTTP proxy**: Only if required by your network

### What Happens Without Admin?
- Symbol resolution still works for user processes
- System process threads may show `0x0` addresses
- Service runs successfully with graceful degradation

## Troubleshooting

### No Module Names Showing

**Symptom**: Thread start addresses show as `0x0` or raw hex addresses

**Causes & Solutions**:
1. **Not running as admin**: 
   - Solution: Run with admin privileges or accept limited resolution for system processes
   
2. **First run delay**:
   - Solution: First PDB download may take 30 seconds; subsequent runs are instant
   
3. **Network issues**:
   - Check: Firewall blocking access to `msdl.microsoft.com`
   - Solution: Configure proxy with `-proxy` parameter

### Symbol Download Failures

**Symptom**: Log shows "Failed to initialize symbol handler"

**Common Causes**:
1. **No internet access**: Symbols require Microsoft's symbol server
2. **Proxy authentication**: NTLM/Kerberos proxy auth may not work with DbgHelp
3. **Firewall**: Blocked HTTPS access to `msdl.microsoft.com`

**Workaround**: Pre-download symbols using SystemInformer or WinDbg on another machine, then copy `C:\Symbols` directory.

### Disk Space

**Symptom**: Disk space warnings or PDB download failures

**Solution**: 
```powershell
# Check symbol cache size
Get-ChildItem 'C:\Symbols' -Recurse | Measure-Object -Property Length -Sum

# Clear old symbols (WARNING: Deletes all cached symbols!)
Remove-Item 'C:\Symbols' -Recurse -Force
```

## Performance Impact

### Minimal Overhead
- **Symbol initialization**: < 100ms per process (one-time per process)
- **Address resolution**: < 1ms per thread (cached modules)
- **Service loop**: No measurable impact
- **Memory**: +2MB for DbgHelp.dll

### When Symbols Are Downloaded
- First access to a new module version: 5-30 seconds
- Subsequent accesses: Instant (cached)
- Downloads happen asynchronously - service loop continues

## Integration with Prime Thread Tracking

Symbol resolution enhances prime thread tracking features:

### Thread Tracking Report
```ini
# Track top 10 threads and log on process exit
cs2.exe:normal:*a:*p:?10*p:normal:normal
```

Output includes resolved module names:
```
Thread tracking report for PID 12345 (cs2.exe):
  [1] TID: 67890 | Cycles: 1234567890 | StartAddress: 0x7FFE12345678 (cs2.exe+0x12F450)
  [2] TID: 67891 | Cycles: 987654321 | StartAddress: 0x7FFE23456789 (nvwgf2umx.dll+0xAB120)
```

### Module-Based Filtering
```ini
# Only promote threads from specific modules
cs2.exe:normal:*a:*p:*p@cs2.exe;nvwgf2umx.dll:normal:normal
```

Module names are automatically resolved for prefix matching.

### Per-Module CPU Assignment
```ini
# Different CPU sets per module
cs2.exe:normal:*a:*p:*p@cs2.exe;*e@background.dll:normal:normal
```

Threads are identified by their start module.

## Advanced Usage

### Symbol Server Configuration

Default symbol path:
```
SRV*c:\symbols*https://msdl.microsoft.com/download/symbols
```

With proxy:
```
SRV*c:\symbols*http://proxy:8080*https://msdl.microsoft.com/download/symbols
```

### Debugging Symbol Resolution

Enable debug output to see symbol handler activity:

```bash
# Run with console output to see symbol initialization messages
AffinityServiceRust.exe -config config.ini -console
```

Look for:
- `Symbol handler initialized for PID XXXX`
- Thread promotion/demotion logs with module names

### Pre-Populating Symbol Cache

For offline or restricted environments:

1. On internet-connected machine:
   - Run SystemInformer or WinDbg
   - Browse to target process threads
   - Symbols download automatically to `C:\Symbols`

2. Copy symbol cache:
   ```bash
   robocopy "C:\Symbols" "\\target-machine\C$\Symbols" /E
   ```

3. Target machine uses cached symbols without downloading

## Limitations

1. **Cache path**: Hardcoded to `C:\Symbols` (not configurable in current version)
2. **Symbol server**: Only Microsoft's public symbol server supported
3. **Private symbols**: Application's own PDB files must be manually placed in cache
4. **32-bit processes**: Limited support on 64-bit systems (cross-architecture limitations)

## Future Enhancements

Planned improvements:
- Configurable symbol cache path
- `-nosymbols` flag to disable for performance
- Multiple symbol server support
- Private symbol server configuration
- Download progress indicators

## See Also

- [Prime Thread Scheduling](../README.md#prime-core-scheduling) - Overview of prime thread features
- [Test Plan](TEST_SYMBOL_RESOLVE.md) - Detailed testing documentation
- [Test Results](../TEST_RESULTS_SYMBOL_RESOLVE.md) - Validation test results

## References

- Microsoft Symbol Server: https://msdl.microsoft.com/download/symbols
- DbgHelp Documentation: https://docs.microsoft.com/en-us/windows/win32/debug/dbghelp-functions
- Symbol Path Format: https://docs.microsoft.com/en-us/windows-hardware/drivers/debugger/symbol-path