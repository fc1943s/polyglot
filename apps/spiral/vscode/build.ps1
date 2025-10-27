param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../../scripts/core.ps1


{ . $(Search-Command bun) install --frozen-lockfile } | Invoke-Block

EnsureSymbolicLink -Path "./LICENSE" -Target "../../../LICENSE"

{ pwsh ./compile.ps1 } | Invoke-Block

{ . $(Search-Command bunx) --bun @vscode/vsce package --pre-release --out "out/" } | Invoke-Block -OnError Continue
