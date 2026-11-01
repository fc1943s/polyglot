$script:SpiralCore = @("$PSScriptRoot/../deps/spiral/scripts/core.ps1", "$PSScriptRoot/../../spiral/scripts/core.ps1") |
    Where-Object { Test-Path -LiteralPath $_ } | Select-Object -First 1
if (!$script:SpiralCore) {
    $spiralCoreUrl = git -C $PSScriptRoot ls-remote --get-url
    $spiralCoreOwner = ($spiralCoreUrl -split '/' | Select-Object -Last 2 | Select-Object -First 1) -replace '\.git$', '' ?? $env:GITHUB_REPOSITORY_OWNER
    $spiralCoreDomain = ($spiralCoreUrl -split '/' | Select-Object -Last 3 | Select-Object -First 1) ?? $env:GITHUB_SERVER_URL -replace 'https?://', ''
    $spiralCoreGitPath = [IO.Path]::GetFullPath("$PSScriptRoot/../..")
    Write-Output "polyglot/scripts/core.ps1 / cloning spiral (the shared helpers) into $spiralCoreGitPath / owner: $spiralCoreOwner / domain: $spiralCoreDomain"
    git -C $spiralCoreGitPath clone --recurse-submodules "https://$spiralCoreDomain/$spiralCoreOwner/spiral.git"
    $script:SpiralCore = "$spiralCoreGitPath/spiral/scripts/core.ps1"
    if (!(Test-Path -LiteralPath $script:SpiralCore)) { throw "polyglot/scripts/core.ps1 / spiral's scripts/core.ps1 not found" }
}
. $script:SpiralCore
