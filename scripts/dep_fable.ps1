param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ./core.ps1


$url = git ls-remote --get-url
$owner = ($url -split '/' | Select-Object -Last 2 | Select-Object -First 1) -replace '\.git$', '' ?? $env:GITHUB_REPOSITORY_OWNER
Write-Output "dep_fable.ps1 / url: $url / owner: $owner"

Set-Location (New-Item "../deps" -ItemType Directory -Force)
git clone --recurse-submodules https://github.com/$owner/Fable.git
{ git pull } | Invoke-Block -Location Fable -OnError Continue

$path = "$HOME/.nuget/packages/fable"
$tools = Get-LastSortedItem -Path $path -Filter "tools"
$toolVersionPath = Get-LastSortedItem -Path $tools.FullName -Filter "any"

Write-Output "polyglot/scripts/dep_fable.ps1 / `$toolVersionPath: $toolVersionPath"

$projectPath = "../deps/Fable/src/Fable.Cli"

{ dotnet build -c Release "$projectPath/Fable.Cli.fsproj" } | Invoke-Block -EnvironmentVariables @{ "MSBUILDDISABLENODEREUSE" = "1" }

$releasePath = "$projectPath/bin/Release"
$dllPath = Get-LastSortedItem -Path $releasePath -Filter "fable.dll"
$dotnetVersion = $dllPath | Split-Path -Parent | Split-Path -Leaf

Write-Output "polyglot/scripts/dep_fable.ps1 / `$dllPath: $dllPath"

Copy-Item "$releasePath/$dotnetVersion/**" $toolVersionPath -Recurse -Force


Copy-Item "../deps/Fable/src/fable-library**" "$tools/.." -Recurse -Force


$path = "$HOME/.nuget/packages/fable.core"
$lib = Get-LastSortedItem -Path $path -Filter "lib"
$libVersionPath = Get-LastSortedItem -Path $lib.FullName -Filter "netstandard2.0"

Write-Output "polyglot/scripts/dep_fable.ps1 / `$libVersionPath: $libVersionPath"

$projectPath = "../deps/Fable/src/Fable.Core"

{ dotnet build -c Release "$projectPath/Fable.Core.fsproj" } | Invoke-Block -EnvironmentVariables @{ "MSBUILDDISABLENODEREUSE" = "1" }

$releasePath = "$projectPath/bin/Release"
$dllPath = Get-LastSortedItem -Path $releasePath -Filter "fable.core.dll"
$dotnetVersion = $dllPath | Split-Path -Parent | Split-Path -Leaf

Copy-Item "$releasePath/$dotnetVersion/**" $libVersionPath -Recurse -Force

{ pwsh ../lib/rust/fable/build.ps1 } | Invoke-Block
{ pwsh ../lib/typescript/fable/build.ps1 } | Invoke-Block
{ pwsh ../lib/python/fable/build.ps1 } | Invoke-Block

Write-Output "polyglot/scripts/dep_fable.ps1 / `$dotnetVersion: $dotnetVersion / `$env:CI:'$env:CI'"

if ($env:CI) {
    Remove-Item ../deps/Fable -Recurse -Force -ErrorAction Ignore
}
