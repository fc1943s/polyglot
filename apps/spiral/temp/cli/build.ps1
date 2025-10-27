param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../../../scripts/core.ps1


dotnet fable --optimize --lang rs --extension .rs

# cargo build --release --manifest-path "$ScriptDir/Cargo.toml"


# . "$ScriptDir/../../target/release/cli" SpiFsxBuild -- --spi-path="$PSScriptRoot/cli.spi"

# dotnet fable `
#     "$ScriptDir" `
#     --optimize `
#     --lang rs `
#     --extension .rs `
#     --outDir "$ScriptDir"

cargo fmt --

cargo build --release

{ . "$ScriptDir/target/release/cli" } | Invoke-Block
