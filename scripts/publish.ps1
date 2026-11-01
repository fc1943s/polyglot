param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ./core.ps1


{ pwsh ../deps/spiral/scripts/publish-tree.ps1 -Root .. -Include '*.png' } | Invoke-Block
