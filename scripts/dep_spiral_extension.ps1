param(
    $fast,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../scripts/core.ps1


$extensionsPath = @()

$extensionsPathHome = "$HOME/.vscode/extensions"
if (Test-Path $extensionsPathHome) {
    $extensionsPath += $extensionsPathHome
}

$extensionsPathHome = "$HOME/.vscode-remote/extensions"
if (Test-Path $extensionsPathHome) {
    $extensionsPath += $extensionsPathHome
}

if ($IsWindows -and $env:scoop) {
    $extensionsPathScoop = "$env:scoop/persist/vscode/data/extensions"
    if (Test-Path $extensionsPathScoop) {
        $extensionsPath += $extensionsPathScoop
    }
    $extensionsPathScoop = "$env:scoop/persist/vscode-insiders/data/extensions"
    if (Test-Path $extensionsPathScoop) {
        $extensionsPath += $extensionsPathScoop
    }
}

$spiralPath = "../deps/The-Spiral-Language"
$extensionSrcPath = "$spiralPath/VS Code Plugin"

if ($extensionsPath.Count -gt 0) {
    $json = Get-Content (Join-Path -Path $extensionSrcPath -ChildPath "package.json") | ConvertFrom-Json
    $vsixName = $json.name + "-" + $json.version + ".vsix"
    $vsixPath = Join-Path -Path $extensionSrcPath -ChildPath $vsixName

    Remove-Item "$extensionSrcPath/compiler" -Recurse -Force -ErrorAction Ignore

    $path ="$spiralPath/The Spiral Language 2/artifacts/bin/The Spiral Language 2/release/"
    Write-Output "path: $path"

    Copy-Item $path "$extensionSrcPath/compiler" -Recurse -Force

    if (!$fast) {
        { . $(Search-Command bun) install --frozen-lockfile } | Invoke-Block -Location $extensionSrcPath
        { . $(Search-Command bunx) --bun tsc --build } | Invoke-Block -Location $extensionSrcPath
    }

    { . $(Search-Command bun) --bun esbuild-base -- --minify } | Invoke-Block -Location $extensionSrcPath
    { . $(Search-Command bunx) --bun @vscode/vsce package } | Invoke-Block -Location $extensionSrcPath -OnError Continue
}

foreach ($extensionsPath in $extensionsPath) {
    $version = Get-ChildItem -Path $extensionsPath -Filter "i574n.spiral-lang-vscode-i574n-*" | Sort-Object -Property Name -Descending | Select-Object -First 1 | ForEach-Object { $_.Name.Substring("$($json.publisher).$($json.name)-".Length) }

    if ($null -eq $version) {
        Write-Output "Skipping copying extension to $extensionsPath"
        continue
    }

    $extensionDestDir = $json.publisher + "." + $json.name + "-" + $version
    $extensionPath = Join-Path -Path $extensionsPath -ChildPath $extensionDestDir

    if ((Test-Path $extensionPath) -and (Test-Path $vsixPath)) {
        Write-Output "Copying extension to $extensionPath"

        Remove-Item $extensionPath -Recurse -Force -ErrorAction Ignore

        Expand-Archive -Path $vsixPath -DestinationPath "$extensionPath/dist" -Force
        Get-ChildItem -Path "$extensionPath/dist/extension" -Recurse -Force | Where-Object { -not $_.PSIsContainer } | ForEach-Object {
            $destPath = Join-Path -Path $extensionPath -ChildPath $_.FullName.Substring("$extensionPath/dist/extension/".Length)

            if (Test-Path $destPath) {
                try {
                    [System.IO.File]::Delete($destPath)
                } catch {
                    Write-Output "Failed to delete $destPath"
                }
            } else {
                New-Item $destPath -Force | Out-Null
            }

            Move-Item $_.FullName $destPath -Force -ErrorAction SilentlyContinue
        }

        Remove-Item "$extensionPath/dist" -Recurse -Force
    }
}

if ($env:CI) {
    Remove-Item $extensionSrcPath/node_modules -Recurse -Force -ErrorAction Ignore
}
