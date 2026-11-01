param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ./core.ps1

EnsureSymbolicLink -Path "../apps/chat/contract/lib" -Target "../lib"
EnsureSymbolicLink -Path "../apps/spiral/temp/cube/lib" -Target "../lib"
