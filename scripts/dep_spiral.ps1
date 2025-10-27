param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ./core.ps1


$url = git ls-remote --get-url
$owner = ($url -split '/' | Select-Object -Last 2 | Select-Object -First 1) -replace '\.git$', '' ?? $env:GITHUB_REPOSITORY_OWNER
Write-Output "dep_spiral.ps1 / url: $url / owner: $owner"

Set-Location (New-Item "../deps" -ItemType Directory -Force)
git clone --recurse-submodules https://github.com/$owner/The-Spiral-Language.git
{ git pull } | Invoke-Block -Location The-Spiral-Language -OnError Continue
Set-Location $ScriptDir

{ dotnet build -c Release "../deps/The-Spiral-Language/The Spiral Language 2/The Spiral Language 2.fsproj" } | Invoke-Block
Copy-Item "../deps/The-Spiral-Language/VS Code Plugin/core/" "../deps/The-Spiral-Language/The Spiral Language 2/artifacts/bin/The Spiral Language 2" -Recurse -Force
