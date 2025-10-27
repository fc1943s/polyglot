param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ./core.ps1


$url = git ls-remote --get-url
$owner = ($url -split '/' | Select-Object -Last 2 | Select-Object -First 1) -replace '\.git$', '' ?? $env:GITHUB_REPOSITORY_OWNER
Write-Output "dep_hyperui.ps1 / url: $url / owner: $owner"

Set-Location (New-Item "../deps" -ItemType Directory -Force)
git clone --recurse-submodules https://github.com/$owner/hyperui.git
{ git pull } | Invoke-Block -Location hyperui -OnError Continue

{ . $(Search-Command bun) install --frozen-lockfile } | Invoke-Block -Location hyperui
{ . $(Search-Command bun) --bun run build } | Invoke-Block -Location hyperui

if ($env:CI) {
    Remove-Item hyperui/node_modules -Recurse -Force -ErrorAction Ignore
}
