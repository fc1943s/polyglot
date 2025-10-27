param(
    $fast,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../scripts/core.ps1


{ pwsh ./contract/build.ps1 -SkipTests 1 } | Invoke-Block

{ pwsh ./contract/tests/build.ps1 } | Invoke-Block -Retries 3
