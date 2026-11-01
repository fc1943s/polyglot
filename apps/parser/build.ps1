param(
    $fast,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../scripts/core.ps1
. ../../deps/spiral/lib/spiral/lib.ps1


foreach ($notebook in "JsonParser", "Parser") {
    if (!$fast) { Invoke-Notebook "$notebook.livemd" @("--no-spi") }
    Invoke-Notebook "$notebook.livemd" @("--no-spi", "--fs-path", "$ScriptDir/$notebook.fs", "--export-only")
}

Write-Output "polyglot/apps/parser/build.ps1 / `$env:CI:'$env:CI'"
