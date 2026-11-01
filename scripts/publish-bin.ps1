param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ./core.ps1


Set-Location ..

$dir = Split-Path -Leaf (Get-Location)
if ($dir -ne "dist") {
    throw "Invalid location (dir <> dist): $(Get-Location)"
    exit 1
}

$files = @(
    Get-ChildItem -Path . -Recurse -File -Force `
    | Where-Object {
        $_.FullName.Replace("\", "/").Contains("/dist") `
            -and ($_.Extension -eq ".exe" -or $_.Extension -eq "") `
            -and !$_.Name.StartsWith("_") `
            -and $_.Name -ne "LICENSE"
    } `
    | Select-Object FullName, DirectoryName, Name
)

$files | ForEach-Object {
    Write-Output "`n`npath: $($_.FullName)"


    Remove-Item $_.FullName -Force
}
if ($Error.Count -gt 0) {
    throw "Failed to process file ($Error)"
    exit 1
}

{ & (pwsh ../deps/spiral/scripts/dir-tree-html.ps1 | Select-Object -Last 1) --dir . --html index.html } | Invoke-Block
