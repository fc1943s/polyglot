param(
    $fast,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../../../scripts/core.ps1


if (!$fast) {
    { . $(Search-Command bun) install --frozen-lockfile } | Invoke-Block
}

{ wasm-pack build --target web --dev <# --reference-types # --weak-refs --no-typescript #> } | Invoke-Block

Remove-Item ./dist -Recurse -Force -ErrorAction Ignore
{ . $(Search-Command bunx) --bun esbuild --bundle --minify --loader:.wasm=file --outdir=dist content_script.ts service_worker.ts devtools.ts } | Invoke-Block -OnError Continue
{ Copy-Item ./public/* ./dist -Recurse -Force } | Invoke-Block -OnError Continue

if (!$fast) {
    { . $(Search-Command bun) test:e2e } | Invoke-Block -OnError Continue
}
