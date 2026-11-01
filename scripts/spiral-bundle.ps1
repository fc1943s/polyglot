$ErrorActionPreference = "Stop"

function Resolve-PolyglotSpiralBundle {
    $candidates = @(
        (Join-Path $PSScriptRoot "../../spiral/apps/compiler/tmp"),
        (Join-Path $PSScriptRoot "../deps/spiral/apps/compiler/tmp")
    )
    foreach ($candidate in $candidates) {
        if (Test-Path -LiteralPath (Join-Path $candidate "scripts/env.ps1")) {
            return (Resolve-Path -LiteralPath $candidate).Path
        }
    }
    throw "Spiral compiler bundle not found. init.ps1 clones it to the sibling spiral repo."
}

function Ensure-SpiralRustCompiler {
    $bundle = Resolve-PolyglotSpiralBundle
    . (Join-Path $bundle "scripts/env.ps1")
    $env:SPIRAL_WORKSPACE_ROOT = $bundle
    $env:CARGO_TERM_COLOR = "never"
    $ready = $false
    try {
        $null = Resolve-SpiralDotnet
        $ready = $true
    } catch {
        $ready = $false
    }
    if (-not $ready) {
        & (Join-Path $bundle "scripts/install-dotnet.ps1")
        if ($LASTEXITCODE -ne 0) { throw "dotnet 11 install failed" }
    }
    $compiler = Get-SpiralCompilerDll "single-flight"
    if (-not (Test-Path -LiteralPath $compiler)) {
        & (Join-Path $bundle "scripts/build.ps1") -Mode single-flight
        if ($LASTEXITCODE -ne 0) { throw "SpiralCompiler build failed" }
        $compiler = Get-SpiralCompilerDll "single-flight"
    }
    if (-not (Test-Path -LiteralPath $compiler)) { throw "SpiralCompiler dll missing at $compiler" }
    [pscustomobject]@{ Bundle = $bundle; Dotnet = (Resolve-SpiralDotnet); Compiler = $compiler }
}
