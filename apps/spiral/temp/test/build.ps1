param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../../../scripts/core.ps1


{ . ../../../../deps/spiral/workspace/target/release/spiral$(_exe) dib --path build.dib } | Invoke-Block

$targetDir = "./fable_modules/fable-library-rust/target"

Remove-Item $targetDir -Recurse -Force -ErrorAction Ignore
