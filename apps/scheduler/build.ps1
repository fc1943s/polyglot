param(
    $fast,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../scripts/core.ps1


if (!$fast) {
    { . ../../deps/spiral/workspace/target/release/spiral$(_exe) dib --path Tasks.dib --retries 3 } | Invoke-Block

    { . ../parser/dist/DibParser$(_exe) Tasks.dib spi } | Invoke-Block
}
