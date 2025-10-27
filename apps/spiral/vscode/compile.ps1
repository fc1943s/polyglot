param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../../scripts/core.ps1

# { . $(Search-Command bun) --bun build --target node --outdir out `
#     src/extension.ts `
#     media/cellOutputScrollButtons.ts `
# } | Invoke-Block

{ . $(Search-Command bunx) --bun esbuild --bundle --outdir=out --external:vscode --format=cjs --platform=node `
    src/extension.ts `
    media/cellOutputScrollButtons.ts `
} | Invoke-Block -OnError Continue
