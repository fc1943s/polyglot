param(
    $fast,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../scripts/core.ps1


if (!$fast) {
    { . ../../deps/spiral/workspace/target/release/spiral$(_exe) dib --path Perf.dib --retries 3 } | Invoke-Block
}

{ . ../parser/dist/DibParser$(_exe) Perf.dib fs } | Invoke-Block
