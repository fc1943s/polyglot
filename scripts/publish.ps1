param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ./core.ps1


Remove-Item ../dist -Recurse -Force -ErrorAction Ignore

rsync -av `
    --exclude '.elixir_ls' `
    --exclude '.git' `
    --exclude '.history' `
    --exclude '.paket' `
    --exclude '.vscode' `
    --exclude 'bin' `
    --exclude 'build' `
    --exclude 'deps' `
    --exclude 'fable_modules' `
    --exclude 'node_modules' `
    --exclude 'obj' `
    --exclude 'paket-files' `
    --exclude 'pkg' `
    --exclude 'target' `
`
    --include 'LICENSE' `
    --include 'Dockerfile' `
    --include '*.ans' `
    --include '*.cs' `
    --include '*.css' `
    --include '*.csproj' `
    --include '*.dependencies' `
    --include '*.dib' `
    --include '*.editorconfig' `
    --include '*.exs' `
    --include '*.fs' `
    --include '*.fsproj' `
    --include '*.fsx' `
    --include '*.gleam' `
    --include '*.gitattributes' `
    --include '*.gitignore' `
    --include '*.html' `
    --include '*.ico' `
    --include '*.png' `
    --include '*.ipynb' `
    --include '*.js' `
    --include '*.json' `
    --include '*.log' `
    --include '*.md' `
    --include '*.nix' `
    --include '*.out' `
    --include '*.ps1' `
    --include '*.py' `
    --include '*.references' `
    --include '*.rs' `
    --include '*.sln' `
    --include '*.spi' `
    --include '*.spiproj' `
    --include '*.spir' `
    --include '*.toml' `
    --include '*.ts' `
    --include '*.vsix' `
    --include '*.wasm' `
    --include '*.webm' `
    --include '*.yaml' `
    --include '*.yml' `
    --include '*.zip' `
`
    --include '*/' `
    --exclude '*' `
    --no-links `
    --prune-empty-dirs `
    ../ `
    ../dist/

Get-ChildItem -Path ../dist -Recurse -Force | Where-Object { $_.Name.StartsWith(".") } | ForEach-Object {
    Rename-Item -Path $_.FullName -NewName "_$($_.Name.Substring(1))"
}

{ . ../apps/dir-tree-html/dist/DirTreeHtml$(_exe) --dir ../dist --html ../dist/index.html } | Invoke-Block
