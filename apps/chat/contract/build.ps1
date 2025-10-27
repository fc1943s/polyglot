param(
    $fast,
    $SkipNotebook,
    $SkipTests,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../../scripts/core.ps1
. ../../../deps/spiral/lib/spiral/lib.ps1


$projectName = "chat_contract"

if (!$fast -and !$SkipNotebook) {
    { . ../../../deps/spiral/workspace/target/release/spiral$(_exe) dib --path "$projectName.dib" --retries $($fast -or !$env:CI ? 1 : 2) } | Invoke-Block
}

{ . ../../../apps/parser/dist/DibParser$(_exe) "$projectName.dib" spi } | Invoke-Block

{ . ../../../apps/spiral/dist/Supervisor$(_exe) --build-file "$projectName.spi" "$projectName.fsx" --timeout 300000 } | Invoke-Block

$runtime = $fast -or $env:CI ? @("--runtime", ($IsWindows ? "win-x64" : "linux-x64")) : @()
$builderArgs = @("$projectName.fsx", $runtime, "--packages", "Fable.Core", "--modules", @(GetFsxModules), "lib/fsharp/Common.fs")
{ . ../../../apps/builder/dist/Builder$(_exe) @builderArgs } | Invoke-Block

$targetDir = GetTargetDir $projectName

{ BuildFable $targetDir $projectName "rs" "CONTRACT" } | Invoke-Block

(Get-Content "$targetDir/target/rs/$projectName.rs") `
    -replace "../../../../../lib", "../../../lib" `
    -replace "`"./lib", "`"../../../lib" `
    -replace "`"../../../../../deps", "`"../../../deps" `
    -replace ".fsx`"]", ".rs`"]" `
    -replace ".rs`"]", "_contract.rs`"]" `
    -replace "use fable_library_rust::DateTime_::DateTime;", "type DateTime = ();" `
    | FixRust `
    | Set-Content "$projectName.rs"

cargo fmt --

{ cargo +nightly-2024-07-14 build --release --target wasm32-unknown-unknown } | Invoke-Block -EnvironmentVariables @{ "AUTOMATION" = "False" }
New-Item dist -ItemType Directory -Force | Out-Null
Copy-Item ../../../workspace/target/wasm32-unknown-unknown/release/chat_contract.wasm dist/chat.wasm -Force

if (!$fast -and !$SkipTests) {
    { ../../../workspace/target/release/chat_contract_tests } | Invoke-Block -Linux -EnvironmentVariables @{ "NEAR_RPC_TIMEOUT_SECS" = 100 }
}

Write-Output "polyglot/apps/chat/contract/build.ps1 / `$targetDir = $targetDir / `$projectName: $projectName / `$env:CI:'$env:CI'"
