param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ./core.ps1


$upstream = "https://github.com/markmead/hyperui.git"
Write-Output "dep_hyperui.ps1 / upstream: $upstream"

Set-Location (New-Item "../deps" -ItemType Directory -Force)
if ((Test-Path hyperui) -and !(Test-Path hyperui/src/styles/component.css)) {
    $aside = "hyperui-fork-$(Get-Date -Format yyyyMMddHHmmss)"
    Write-Output "dep_hyperui.ps1 / moving the previous fork checkout aside: $aside"
    Move-Item hyperui $aside
}
if (!(Test-Path hyperui)) {
    git clone --depth 1 $upstream hyperui
}
{ git pull } | Invoke-Block -Location hyperui -OnError Continue

if (!(Test-Path hyperui/public/component.css)) {
    throw "dep_hyperui.ps1 / upstream no longer ships public/component.css"
}
