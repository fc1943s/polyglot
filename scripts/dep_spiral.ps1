param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ./core.ps1


$url = git ls-remote --get-url
$owner = ($url -split '/' | Select-Object -Last 2 | Select-Object -First 1) -replace '\.git$', '' ?? $env:GITHUB_REPOSITORY_OWNER
$domain = ($url -split '/' | Select-Object -Last 3 | Select-Object -First 1) ?? $env:GITHUB_SERVER_URL -replace 'https?://', ''
Write-Output "dep_spiral.ps1 / url: $url / owner: $owner / domain: $domain"

Set-Location (New-Item "../deps" -ItemType Directory -Force)
git clone --recurse-submodules https://$domain/$owner/The-Spiral-Language.git
{ git pull } | Invoke-Block -Location The-Spiral-Language -OnError Continue
Set-Location $ScriptDir
