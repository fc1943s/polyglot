$ScriptDir =
    [Environment]::CommandLine -match '--working-dir\s+(\S+)' ?
        $Matches[1] :
        ($m = [Environment]::CommandLine -match '--run\s+(\S+)' ? $Matches[1] : $null) -and $m -match '[\\/]' ?
            (Split-Path -Path $m -Parent) :
            [Environment]::CurrentDirectory
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"

$ProgressPreference = "SilentlyContinue"
