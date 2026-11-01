param(
    $fast,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../scripts/core.ps1

$livebook = Join-Path $ScriptDir "../../deps/spiral/apps/cells/spi/run_notebook.ps1"
$spi = Join-Path $ScriptDir "Perf.spi"
if (!$fast) {
    { pwsh -NoProfile -File $livebook --path $spi --output-path (Join-Path $ScriptDir "Perf.spi.ipynb") } | Invoke-Block -Retries $($env:CI ? 3 : 1)
}