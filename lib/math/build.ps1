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
    { . ../../deps/spiral/workspace/target/release/spiral$(_exe) dib --path "$projectName.dib" --retries $($fast -or !$env:CI ? 1 : 2) } | Invoke-Block -OnError Continue
}

{ . ../../apps/parser/dist/DibParser$(_exe) "$projectName.dib" spi } | Invoke-Block

{ . ../../apps/spiral/dist/Supervisor$(_exe) --build-file "$projectName.spi" "$projectName.fsx" --timeout 300000 } | Invoke-Block

$runtime = $fast -or $env:CI ? @("--runtime", ($IsWindows ? "win-x64" : "linux-x64")) : @()
$builderArgs = @("$projectName.fsx", $runtime, "--packages", "Fable.Core", "--modules", @(GetFsxModules), "lib/fsharp/Common.fs")
{ . ../../apps/builder/dist/Builder$(_exe) @builderArgs } | Invoke-Block

$targetDir = GetTargetDir $projectName

{ BuildFable $targetDir $projectName "rs" } | Invoke-Block

$path = "$targetDir/$projectName.rs"
if (!(Test-Path $path)) {
    $path = "$targetDir/target/rs/target/Builder/$projectName/$projectName.rs"
}
if (!(Test-Path $path)) {
    $path = "$targetDir/target/rs/$projectName.rs"
}
Write-Output "polyglot/lib/math/build.ps1 / path: $path"
(Get-Content $path) `
    -replace "`"../../../../../deps", "`"../../deps" `
    -replace "`"./lib", "`"../../lib" `
    -replace ".fsx`"]", ".rs`"]" `
    | FixRust `
    | Set-Content "$projectName.rs"

cargo fmt --

{ cargo test --release } | Invoke-Block -OnError Continue

Write-Output "polyglot/lib/math/build.ps1 / `$targetDir: $targetDir / `$projectName: $projectName / `$env:CI:'$env:CI'"

if ($env:CI) {
    Remove-Item $targetDir -Recurse -Force -ErrorAction Ignore
}
