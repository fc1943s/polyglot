param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ./core.ps1


{ . ../deps/spiral/workspace/target/release/spiral$(_exe) dib --path build.dib } | Invoke-Block
