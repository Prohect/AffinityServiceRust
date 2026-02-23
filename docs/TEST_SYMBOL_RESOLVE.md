# Test Plan: Automatic Module Resolve & PDB File Download

## Feature Overview

The new feature enables automatic resolution of thread start addresses to module names (e.g., `ntdll.dll+0x3C320`) by:
1. Initializing DbgHelp symbol handler with Microsoft symbol server
2. Automatically downloading PDB files to `C:\Symbols\`
3. Resolving thread start addresses to module+offset format
4. Supporting HTTP proxy configuration for corporate networks

## Prerequisites

- **Admin elevation**: Required for `SeDebugPrivilege` to read thread start addresses
- **Internet access**: To download symbols from `https://msdl.microsoft.com/download/symbols`
- **Disk space**: PDB files can be large (some >80MB). Budget ~500MB-1GB for testing
- **Test process**: A process with prime thread tracking enabled (e.g., `cs2.exe`, `notepad.exe`)

## Test Environment Setup

### 1. Check Current Symbol Cache State

Before testing, record the current state:

```bash
# List most recent 20 files in C:\Symbols with timestamps
powershell -Command "Get-ChildItem 'C:\Symbols' -Recurse -File | Select-Object LastWriteTime, Length, FullName | Sort-Object LastWriteTime -Descending | Select-Object -First 20"
```

Save this output to compare after testing.

### 2. Prepare Test Config

Create a test config entry with prime thread tracking:

```ini
# Test config for symbol resolution
@MIN_ACTIVE_STREAK = 1
@ENTRY_THRESHOLD = 0.1
@KEEP_THRESHOLD = 0.3

*a = 0-15
*p = 0-7
*e = 8-15

# Track top 5 threads for notepad (simple test case)
notepad.exe:normal:*a:*p:?5*p:normal:normal

# Track top 10 threads for a game (comprehensive test)
# cs2.exe:normal:*a:*p:?10*p@cs2.exe;nvwgf2umx.dll:normal:normal
```

## Test Cases

### Test 1: Basic Symbol Resolution (No Proxy)

**Objective**: Verify symbol initialization and PDB download without proxy.

**Steps**:
1. Record current timestamp: `powershell -Command "Get-Date"`
2. Clear existing test logs: `del logs\*.log`
3. Run with test config:
   ```bash
   AffinityServiceRust.exe -config test_symbol.ini -console -interval 3000 -logloop -loop 5
   ```
4. Start `notepad.exe` during the test
5. Close `notepad.exe` before the 5th loop ends
6. Check console output for:
   - `Symbol handler initialized for PID XXXX, will download symbols to c:\symbols`
   - Thread tracking report with module names (e.g., `ntdll.dll+0x3C320`)
7. Check `C:\Symbols` for new PDB files:
   ```bash
   powershell -Command "Get-ChildItem 'C:\Symbols' -Recurse -File | Where-Object {$_.LastWriteTime -gt (Get-Date).AddMinutes(-10)} | Select-Object LastWriteTime, Length, FullName"
   ```

**Expected Results**:
- Symbol handler initialized successfully
- PDB files downloaded (e.g., `ntdll.pdb`, `kernel32.pdb`)
- Thread start addresses resolved to `module.dll+offset` format
- No errors in logs

**Success Criteria**:
- ✅ At least 1 new PDB file downloaded to `C:\Symbols\` with timestamp after test start
- ✅ Thread report shows resolved module names (not just `0x0` or raw addresses)
- ✅ No "Failed to initialize symbol handler" errors

### Test 2: Symbol Resolution with Proxy

**Objective**: Verify proxy configuration for symbol downloads in corporate networks.

**Steps**:
1. Set up test proxy (if available) or skip if no proxy needed
2. Run with proxy parameter:
   ```bash
   AffinityServiceRust.exe -config test_symbol.ini -console -proxy http://proxy.example.com:8080 -interval 3000 -logloop -loop 5
   ```
3. Start and close `notepad.exe` during test
4. Verify console shows: `Using proxy for symbol downloads: http://proxy.example.com:8080`
5. Check for successful symbol initialization and resolution

**Expected Results**:
- Proxy setting logged
- Symbols downloaded through proxy (if proxy is functional)
- Thread addresses resolved correctly

**Success Criteria**:
- ✅ Proxy configuration logged
- ✅ Symbol resolution works (or shows appropriate proxy errors)

### Test 3: Multiple Processes with Different Modules

**Objective**: Verify symbol caching and multi-process handling.

**Steps**:
1. Create config tracking multiple processes:
   ```ini
   notepad.exe:normal:*a:*p:?3*p:normal:normal
   calc.exe:normal:*a:*p:?3*p:normal:normal
   mspaint.exe:normal:*a:*p:?3*p:normal:normal
   ```
2. Run service:
   ```bash
   AffinityServiceRust.exe -config test_multi.ini -console -interval 3000
   ```
3. Start all three processes within 10 seconds
4. Let them run for 30 seconds
5. Close all three processes
6. Check logs for symbol resolution for each PID

**Expected Results**:
- Each process gets its own symbol handler initialized
- PDB files are reused if same modules appear in multiple processes
- Thread reports for each process show resolved addresses

**Success Criteria**:
- ✅ Symbol handler initialized for each unique PID
- ✅ Module names resolved for all tracked processes
- ✅ No memory leaks or handle leaks (check with Task Manager)

### Test 4: Module Prefix Filtering

**Objective**: Verify module-based thread filtering works with resolved names.

**Steps**:
1. Create config with prefix filtering:
   ```ini
   # Only track threads from specific modules
   explorer.exe:normal:*a:*p:?10*p@shell32.dll;windows.storage:normal:normal
   ```
2. Run service and observe `explorer.exe` threads
3. Verify only threads from `shell32.dll` and modules starting with `windows.storage` are promoted

**Expected Results**:
- Only matching threads are promoted to prime CPUs
- Non-matching threads are ignored
- Module names are case-insensitive matched

**Success Criteria**:
- ✅ Only threads with matching module prefixes are promoted
- ✅ Module resolution happens before prefix matching
- ✅ Logs show which modules were matched/ignored

### Test 5: Symbol Resolution Without Admin Privileges

**Objective**: Verify graceful degradation when not elevated.

**Steps**:
1. Run without admin elevation:
   ```bash
   AffinityServiceRust.exe -config test_symbol.ini -console -noUAC -interval 3000 -logloop -loop 3
   ```
2. Start `notepad.exe`
3. Close `notepad.exe`

**Expected Results**:
- Service runs without admin privileges
- Thread start addresses show as `0x0` (cannot read without SeDebugPrivilege)
- No symbol downloads attempted
- No crashes or errors

**Success Criteria**:
- ✅ Service runs successfully without elevation
- ✅ Thread tracking works but shows `0x0` for start addresses
- ✅ Logs indicate admin privileges are needed for symbol resolution

### Test 6: Stress Test - Large PDB Downloads

**Objective**: Verify handling of large PDB files and network delays.

**Steps**:
1. Track a complex process (e.g., browser, IDE):
   ```ini
   chrome.exe:normal:*a:*p:?20*p:normal:normal
   ```
2. Start Chrome and open multiple tabs
3. Monitor `C:\Symbols` directory size growth
4. Check for download timeouts or errors

**Expected Results**:
- Large PDB files (50MB+) download successfully
- Symbol resolution doesn't block main service loop
- Service remains responsive during downloads

**Success Criteria**:
- ✅ Large PDB files download completely
- ✅ Service check interval is not affected by symbol downloads
- ✅ No crashes during symbol loading

### Test 7: Symbol Cleanup on Process Exit

**Objective**: Verify proper cleanup of symbol handlers and module cache.

**Steps**:
1. Enable verbose logging and run:
   ```bash
   AffinityServiceRust.exe -config test_symbol.ini -console -interval 2000
   ```
2. Start `notepad.exe` (PID recorded, e.g., 12345)
3. Wait for symbols to initialize
4. Close `notepad.exe`
5. Check logs for cleanup messages
6. Verify no lingering handles with Process Explorer

**Expected Results**:
- Symbol handler cleaned up when process exits
- Module cache cleared for the PID
- Thread report generated on process exit

**Success Criteria**:
- ✅ Symbol cleanup called on process exit
- ✅ No memory leaks after multiple process start/stop cycles
- ✅ Module cache properly cleared

## Test Execution Checklist

Before each test:
- [ ] Record current timestamp and `C:\Symbols` state
- [ ] Clear or rotate log files
- [ ] Note the config being tested
- [ ] Verify admin elevation status

During test:
- [ ] Monitor console output for errors
- [ ] Watch `C:\Symbols` directory for new files
- [ ] Check Task Manager for memory/handle usage
- [ ] Note any unexpected behavior

After each test:
- [ ] Review logs for errors or warnings
- [ ] Compare `C:\Symbols` timestamps to identify new downloads
- [ ] Verify thread reports show resolved module names
- [ ] Document any issues or unexpected behavior

## Success Metrics

The feature is working correctly if:
1. **Symbol Handler Initialization**: 100% success rate for elevated processes
2. **PDB Downloads**: New PDB files appear in `C:\Symbols` with recent timestamps
3. **Module Resolution**: Thread start addresses resolve to `module.dll+offset` format
4. **Proxy Support**: Proxy parameter correctly passed to symbol server
5. **No Regressions**: Existing functionality (affinity, priority, etc.) still works
6. **Graceful Degradation**: Works without elevation (shows `0x0` addresses)
7. **No Leaks**: Memory and handles properly released on process exit

## Known Issues to Watch For

- **Network timeout**: PDB downloads may take 5-30 seconds for first access
- **Disk space**: Large games (e.g., CS2) may download 100MB+ of symbols
- **Proxy authentication**: NTLM/Kerberos proxy auth may not work with DbgHelp
- **Symbol server unavailable**: Microsoft symbol server occasionally has outages
- **Windows Defender**: May scan downloaded PDB files, causing delays

## Verification Commands

```bash
# Check recent PDB downloads (last 10 minutes)
powershell -Command "Get-ChildItem 'C:\Symbols' -Recurse -File | Where-Object {$_.LastWriteTime -gt (Get-Date).AddMinutes(-10)} | Select-Object LastWriteTime, Length, FullName | Sort-Object LastWriteTime -Descending"

# Count total PDB files
powershell -Command "(Get-ChildItem 'C:\Symbols' -Recurse -File -Filter '*.pdb').Count"

# Check C:\Symbols size
powershell -Command "'{0:N2} MB' -f ((Get-ChildItem 'C:\Symbols' -Recurse | Measure-Object -Property Length -Sum).Sum / 1MB)"

# Watch symbol downloads in real-time (run in separate terminal)
powershell -Command "Get-ChildItem 'C:\Symbols' -Recurse -File | Sort-Object LastWriteTime -Descending | Select-Object -First 5 | Format-Table LastWriteTime, Length, Name -AutoSize; Start-Sleep 2; cls" -NoExit

# Check if process has debug privilege enabled
# (Run as admin in PowerShell after starting AffinityServiceRust.exe)
Get-Process AffinityServiceRust | ForEach-Object { $_.Handle }
```

## Final Validation Before Push

Before pushing to repository:
1. ✅ Run Test 1 (Basic Symbol Resolution) successfully
2. ✅ Run Test 3 (Multiple Processes) successfully
3. ✅ Run Test 5 (Without Admin) to verify graceful degradation
4. ✅ Verify no new compiler warnings or errors
5. ✅ Update CHANGELOG.md or README.md with feature description
6. ✅ Ensure code compiles cleanly: `cargo build --release`
7. ✅ Test on at least one target machine with clean `C:\Symbols`

## Post-Test Cleanup

```bash
# Optional: Clear symbol cache to test fresh downloads
# WARNING: This deletes all downloaded symbols!
# powershell -Command "Remove-Item 'C:\Symbols' -Recurse -Force"

# Clear test logs
del logs\*.log
```
