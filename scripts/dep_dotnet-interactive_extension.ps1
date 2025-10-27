param(
    $fast,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../scripts/core.ps1
. ../deps/spiral/lib/spiral/lib.ps1


$interactivePath = "../deps/dotnet-interactive"

$extensionSrcPath = "$interactivePath/src/polyglot-notebooks-vscode-insiders"

$json = Get-Content (Join-Path $extensionSrcPath "package.json") | ConvertFrom-Json
$vsixName = $json.name + "-" + $json.version + ".vsix"
$vsixPath = Join-Path $extensionSrcPath $vsixName

if (!$fast) {
    { npm install } | Invoke-Block -Location "$extensionSrcPath/../polyglot-notebooks-ui-components"
    { npm install } | Invoke-Block -Location $extensionSrcPath
}

Remove-Item $vsixPath -Force -ErrorAction Ignore

Write-Output "Compiling..."
{ . $(Search-Command bun) --bun compile } | Invoke-Block -Location $extensionSrcPath

$path = Join-Path $extensionSrcPath "out/deps/dotnet-interactive/src/polyglot-notebooks-vscode-insiders/src/vscode-common/documentSemanticTokenProvider.js"
Write-Output "path: $path"
(Get-Content $path) `
    -replace [regex]::Escape("require(`"../../"), "require(`"../../../../../../" `
    | Set-Content $path

$path = Join-Path $extensionSrcPath "out/deps/spiral/deps/polyglot/deps/Fable/src/fable-library-ts/Array.js"
(Get-Content $path) `
    -replace "\(`"\./fable_modules/fable-library-ts\.[\-\w\d\.]+/", "(`"./" `
    | Set-Content $path

$path = Join-Path $extensionSrcPath "out/deps/spiral/deps/polyglot/deps/Fable/src/fable-library-ts/Choice.js"
(Get-Content $path) `
    -replace "\(`"\./fable_modules/fable-library-ts\.[\-\w\d\.]+/", "(`"./" `
    | Set-Content $path

$path = Join-Path $extensionSrcPath "out/deps/spiral/deps/polyglot/deps/Fable/src/fable-library-ts/Double.js"
(Get-Content $path) `
    -replace "\(`"\./fable_modules/fable-library-ts\.[\-\w\d\.]+/", "(`"./" `
    | Set-Content $path

$path = Join-Path $extensionSrcPath "out/deps/spiral/deps/polyglot/deps/Fable/src/fable-library-ts/FSharp.Collections.js"
(Get-Content $path) `
    -replace "\(`"\./fable_modules/fable-library-ts\.[\-\w\d\.]+/", "(`"./" `
    | Set-Content $path

$path = Join-Path $extensionSrcPath "out/deps/spiral/deps/polyglot/deps/Fable/src/fable-library-ts/FSharp.Core.js"
(Get-Content $path) `
    -replace "\(`"\./fable_modules/fable-library-ts\.[\-\w\d\.]+/", "(`"./" `
    | Set-Content $path

$path = Join-Path $extensionSrcPath "out/deps/spiral/deps/polyglot/deps/Fable/src/fable-library-ts/List.js"
(Get-Content $path) `
    -replace "\(`"\./fable_modules/fable-library-ts\.[\-\w\d\.]+/", "(`"./" `
    | Set-Content $path

$path = Join-Path $extensionSrcPath "out/deps/spiral/deps/polyglot/deps/Fable/src/fable-library-ts/Map.js"
(Get-Content $path) `
    -replace "\(`"\./fable_modules/fable-library-ts\.[\-\w\d\.]+/", "(`"./" `
    | Set-Content $path

$path = Join-Path $extensionSrcPath "out/deps/spiral/deps/polyglot/deps/Fable/src/fable-library-ts/Result.js"
(Get-Content $path) `
    -replace "\(`"\./fable_modules/fable-library-ts\.[\-\w\d\.]+/", "(`"./" `
    | Set-Content $path

$path = Join-Path $extensionSrcPath "out/deps/spiral/deps/polyglot/deps/Fable/src/fable-library-ts/Seq.js"
(Get-Content $path) `
    -replace "\(`"\./fable_modules/fable-library-ts\.[\-\w\d\.]+/", "(`"./" `
    | Set-Content $path

$path = Join-Path $extensionSrcPath "out/deps/spiral/deps/polyglot/deps/Fable/src/fable-library-ts/SystemException.js"
(Get-Content $path) `
    -replace "\(`"\./fable_modules/fable-library-ts\.[\-\w\d\.]+/", "(`"./" `
    | Set-Content $path

$path = Join-Path $extensionSrcPath "out/deps/spiral/deps/polyglot/deps/Fable/src/fable-library-ts/System.Text.js"
(Get-Content $path) `
    -replace "\(`"\./fable_modules/fable-library-ts\.[\-\w\d\.]+/", "(`"./" `
    | Set-Content $path

$dir = Join-Path $extensionSrcPath "out/deps/spiral/deps/polyglot/deps/Fable/src/fable-library-ts/lib"
New-Item $dir -ItemType Directory -Force | Out-Null
Copy-Item "$ScriptDir/../deps/Fable/src/fable-library-ts/lib/ts/big.js" $(Join-Path $dir "big.js") -Force
# { . $(Search-Command bun) --bun build big.ts --minify --target=node --outfile=big.js } | Invoke-Block -Location $dir
# Remove-Item $(Join-Path $dir "big.ts") -Force -ErrorAction Ignore

Write-Output "Packaging..."
{ . $(Search-Command bun)--bun package } | Invoke-Block -Location $extensionSrcPath


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

foreach ($extensionsPath in $extensionsPath) {
    $version = Get-ChildItem -Path $extensionsPath -Filter "$($json.publisher).$($json.name)-*" | Sort-Object -Property Name -Descending | Select-Object -First 1 | ForEach-Object { $_.Name.Substring("$($json.publisher).$($json.name)-".Length) }

    if ($null -eq $version) {
        Write-Output "Skipping copying extension to $extensionsPath"
        continue
    }

    $extensionDestDir = $json.publisher + "." + $json.name + "-" + $version
    $extensionPath = Join-Path $extensionsPath $extensionDestDir
    Write-Output "Copying extension to $extensionPath"

    $jsonPath = Join-Path $extensionPath "package.json"
    if (Test-Path $jsonPath) {
        $currentJson = Get-Content $jsonPath | ConvertFrom-Json
    } else {
        Write-Output "jsonPath not found. jsonPath: $JsonPath"
    }

    Remove-Item $extensionPath -Recurse -Force -ErrorAction Ignore

    Expand-Archive $vsixPath "$extensionPath/dist" -Force
    Get-ChildItem -Path "$extensionPath/dist/extension" -Recurse -Force | Where-Object { -not $_.PSIsContainer } | ForEach-Object {
        $destPath = Join-Path $extensionPath $_.FullName.Substring("$extensionPath/dist/extension/".Length)

        if (Test-Path $destPath) {
            try {
                [System.IO.File]::Delete($destPath)
            } catch {
                Write-Output "Failed to delete $destPath"
            }
        } else {
            New-Item $destPath -Force | Out-Null
        }

        Move-Item $_.FullName $destPath -Force -ErrorAction Ignore
    }

    Remove-Item "$extensionPath/dist" -Recurse -Force

    Update-Json -jsonPath "$extensionPath/package.json" -ContentModifier {
        param($jsonContent)
        $jsonContent.version = $version
        $jsonContent.contributes.configuration.properties."dotnet-interactive.requiredInteractiveToolVersion".default = $currentJson.contributes.configuration.properties."dotnet-interactive.requiredInteractiveToolVersion".default
        if (-not $jsonContent.__metadata) {
            $jsonContent | Add-Member -NotePropertyName "__metadata" -NotePropertyValue @{}
        }
        $jsonContent.__metadata = $currentJson.__metadata
        return $jsonContent
    }
}
