param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ./core.ps1


$url = git ls-remote --get-url
$owner = ($url -split '/' | Select-Object -Last 2 | Select-Object -First 1) -replace '\.git$', '' ?? $env:GITHUB_REPOSITORY_OWNER
Write-Output "dep_dotnet-repl.ps1 / url: $url / owner: $owner"

Set-Location (New-Item "../deps" -ItemType Directory -Force)
git clone --recurse-submodules https://github.com/$owner/dotnet-repl.git

Set-Location dotnet-repl
{ git pull } | Invoke-Block -OnError Continue

Set-Location $ScriptDir/../deps

$path = "$HOME/.nuget/packages/dotnet-repl"
$tools = Get-LastSortedItem -Path $path -Filter "tools"
$netVersion = Get-LastSortedItem -Path $tools.FullName -Filter "any"

Write-Output "Tool path: $netVersion"


{ dotnet build -c Release "../deps/dotnet-repl/src/dotnet-repl/dotnet-repl.csproj" } | Invoke-Block -EnvironmentVariables @{ "MSBUILDDISABLENODEREUSE" = "1" }

Copy-Item "../deps/dotnet-repl/src/dotnet-repl/bin/Release/net9.0/**" $netVersion -Recurse -Force

Write-Output "polyglot/scripts/dep_dotnet-repl.ps1 / `$tools: $tools / `$netVersion: $netVersion / `$env:CI:'$env:CI'"

if ($env:CI) {
    Remove-Item "../deps/dotnet-interactive" -Recurse -Force -ErrorAction Ignore
    Remove-Item "../deps/dotnet-repl" -Recurse -Force -ErrorAction Ignore
}
