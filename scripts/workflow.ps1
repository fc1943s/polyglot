param(
    $fast,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"

. ./core.ps1


{ pwsh init.ps1 } | Invoke-Block

{ pwsh build.ps1 } | Invoke-Block

if ($env:CI) {
    Remove-Item ../target -Recurse -Force -ErrorAction Ignore
}

{ pwsh publish.ps1 } | Invoke-Block

{ pwsh ../dist/scripts/publish-bin.ps1 } | Invoke-Block

if ($env:CI) {
    Remove-Item ../workspace/target -Recurse -Force -ErrorAction Ignore
}
