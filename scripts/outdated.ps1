param(
    $SkipPaket,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ./core.ps1


function CheckToml {
    param (
        [Parameter(Position = 0, Mandatory)]
        [string] $toml,

        [Parameter(Position = 1, ValueFromRemainingArguments)]
        [Object[]]$_args
    )
    $toml = [IO.Path]::GetFullPath("$ScriptDir/$toml")
    Write-Output "`nCheckToml / toml: $toml"
    { cargo outdated -m $toml @_args } | Invoke-Block -OnError Continue
}

function CheckJson {
    param (
        [string] $json
    )
    $json = [IO.Path]::GetFullPath("$ScriptDir/$json").Replace("\", "/")
    Write-Output "`nCheckJson / json: $json"
    { . $(Search-Command bun) --bun --cwd $json outdated-pre } | Invoke-Block -OnError Continue
}


if (!$SkipPaket) {
    { dotnet paket outdated --include-prereleases } | Invoke-Block -OnError Continue
}

CheckToml "../workspace/Cargo.toml" `-w

CheckToml "../apps/chat/contract/Cargo.toml"
CheckToml "../apps/chat/contract/tests/Cargo.toml"
CheckToml "../apps/plot/Cargo.toml"

CheckJson ".."
CheckJson "../apps/ipfs"
CheckJson "../apps/spiral/temp/extension"
CheckJson "../apps/spiral/vscode"
CheckJson "../deps/The-Spiral-Language/VS Code Plugin"
