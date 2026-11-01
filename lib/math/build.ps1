param(
    $fast,
    $SkipNotebook,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../scripts/core.ps1
. ../../deps/spiral/lib/spiral/lib.ps1


$projectName = "math"

if (!$fast -and !$SkipNotebook) {
    Invoke-Notebook "$projectName.spi" @("--output-path", "$ScriptDir/$projectName.spi.ipynb") -Retries ($fast -or !$env:CI ? 1 : 2)
}

$targetDir = GetTargetDir $projectName

$rustTests = 12
if (!(BuildSpiral "$projectName.spi" "$projectName.rs" "lib/math")) {
    throw "RUST-FAILED lib/math / compile"
}
Push-Location ../../workspace
try {
    $rustOutput = cargo +nightly-2026-10-18 test --release --package $projectName 2>&1 | ForEach-Object { "$_" }
    $rustExit = $LASTEXITCODE
} finally {
    Pop-Location
}
$rustOutput | ForEach-Object { Write-Output "polyglot/lib/math/build.ps1 / cargo test / $_" }
if ($rustExit -ne 0 -or !($rustOutput -match "^test result: ok\. $rustTests passed; 0 failed")) {
    throw "RUST-FAILED lib/math / cargo test exit code $($rustExit): expected 'test result: ok. $rustTests passed; 0 failed'"
}
Write-Output "RUST-OK lib/math"
Write-Output "polyglot/lib/math/build.ps1 / `$targetDir: $targetDir / `$projectName: $projectName / `$env:CI:'$env:CI'"

if ($env:CI) {
    Remove-Item $targetDir -Recurse -Force -ErrorAction Ignore
    ClearCargoTarget "../../workspace"
}
