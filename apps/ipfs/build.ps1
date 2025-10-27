param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../scripts/core.ps1


{ . $(Search-Command bun) install --frozen-lockfile } | Invoke-Block
