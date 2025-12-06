<#
.SYNOPSIS
    Process daily find logs to discover new processes not in config.ini or blacklist.ini,
    then search for them using Everything CLI (es) to get file paths.

.DESCRIPTION
    This script analyzes .find.log files in temp/logs, extracts process names,
    filters out those already configured or blacklisted, and uses es.exe to search
    for exact matches. Results are saved to temp/new_processes_results.txt.

    Requirements:
    - PowerShell (Windows)
    - Everything search tool with es.exe in PATH
    - temp/logs symlink pointing to daily logs

.USAGE
    Run from the project root:
    powershell.exe -ExecutionPolicy Bypass -File process_logs.ps1

    Or from temp/:
    powershell.exe -ExecutionPolicy Bypass -File ../process_logs.ps1

.NOTES
    - Assumes config.ini and blacklist.ini are in the same directory.
    - Output format: Process name, followed by es results or "Not found", separated by ---.
    - Useful for updating AffinityServiceRust config with new processes.
#>

[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

# Paths (relative to script location)
$logsPath = "temp/logs"
$configPath = "config.ini"
$blacklistPath = "blacklist.ini"
$outputFile = "temp/new_processes_results.txt"

# Function to extract processes from a log file
# Parses lines like "[time]find process.exe" or "[time]apply_config: [OPEN][INVALID_PARAMETER] pid-process.exe"
function Get-ProcessesFromLog {
    param ($logFile)
    $processes = @()
    try {
        $content = Get-Content $logFile -Encoding UTF8
    } catch {
        Write-Warning "Could not read $logFile : $_"
        return $processes
    }
    foreach ($line in $content) {
        # Match "find process.exe"
        if ($line -match "find\s+(\S+\.exe)") {
            $processes += $matches[1]
        }
        # Match "pid-process.exe" in error messages
        elseif ($line -match "\[OPEN\]\[INVALID_PARAMETER\]\s+\d+-(.+?\.exe)") {
            $processes += $matches[1]
        }
    }
    return $processes
}

# Get all unique processes from logs
Write-Host "Scanning log files in $logsPath..."
$allProcesses = @{}
$logFiles = Get-ChildItem "$logsPath/*.find.log" -ErrorAction SilentlyContinue
if ($logFiles.Count -eq 0) {
    Write-Warning "No .find.log files found in $logsPath. Ensure the symlink is set up."
    exit
}
$logFiles | ForEach-Object {
    $procs = Get-ProcessesFromLog $_.FullName
    foreach ($proc in $procs) {
        $allProcesses[$proc] = $true
    }
}
Write-Host "Found $($allProcesses.Count) unique processes in logs."

# Get existing processes from config.ini
Write-Host "Loading processes from $configPath..."
$configProcesses = @{}
try {
    $content = Get-Content $configPath -Encoding UTF8
} catch {
    Write-Warning "Could not read $configPath : $_"
    exit
}
foreach ($line in $content) {
    if ($line -match "\b(\w+\.exe)\b") {
        $configProcesses[$matches[1]] = $true
    }
}
Write-Host "Found $($configProcesses.Count) processes in config."

# Get existing processes from blacklist.ini
Write-Host "Loading processes from $blacklistPath..."
$blacklistProcesses = @{}
try {
    $content = Get-Content $blacklistPath -Encoding UTF8
} catch {
    Write-Warning "Could not read $blacklistPath : $_"
    exit
}
foreach ($line in $content) {
    if ($line -match "\b(\w+\.exe)\b") {
        $blacklistProcesses[$matches[1]] = $true
    }
}
Write-Host "Found $($blacklistProcesses.Count) processes in blacklist."

# Filter new processes (not in config or blacklist)
$newProcesses = @()
foreach ($proc in $allProcesses.Keys) {
    if (-not $configProcesses.ContainsKey($proc) -and -not $blacklistProcesses.ContainsKey($proc)) {
        $newProcesses += $proc
    }
}
Write-Host "Found $($newProcesses.Count) new processes to search."

# Search with es and output
Write-Host "Searching for new processes using es.exe..."
$output = ""
foreach ($proc in $newProcesses) {
    $output += "Process: $proc`n"
    $tempFile = "temp\temp_$proc.txt"
    try {
        & es -export-txt $tempFile -utf8-bom -r "^$proc$" 2>$null
        if (Test-Path $tempFile) {
            $result = Get-Content $tempFile -Encoding UTF8
            Remove-Item $tempFile -ErrorAction SilentlyContinue
            if ($result -and $result.Count -gt 0) {
                $output += "Found:`n"
                $result | ForEach-Object { $output += "  $_`n" }
            } else {
                $output += "Not found`n"
            }
        } else {
            $output += "Not found`n"
        }
    } catch {
        Write-Warning "es.exe not found or failed for $proc : $_"
        $output += "Not found`n"
    }
    $output += "---`n"
}

try {
    $output | Out-File $outputFile -Encoding UTF8
    Write-Host "Results saved to $outputFile"
} catch {
    Write-Error "Failed to save output: $_"
}
