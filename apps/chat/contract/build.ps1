param(
    $fast,
    $SkipNotebook,
    $SkipTests,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../../scripts/core.ps1
. ../../../deps/spiral/lib/spiral/lib.ps1

$projectName = "chat_contract"
$contractToolchain = "nightly-2024-07-14"

$livebook = Join-Path $ScriptDir "../../../deps/spiral/apps/cells/spi/run_notebook.ps1"
$spi = Join-Path $ScriptDir "$projectName.spi"
$ipynb = Join-Path $ScriptDir "$projectName.spi.ipynb"
if (!$fast -and !$SkipNotebook) {
    { pwsh -NoProfile -File $livebook --path $spi --output-path $ipynb } | Invoke-Block -Retries $($fast -or !$env:CI ? 1 : 3)
}

function Get-WasmExports([string] $Path) {
    $bytes = [IO.File]::ReadAllBytes($Path)
    $at = @(8)
    $leb = { $r = 0; $shift = 0; do { $b = $bytes[$at[0]++]; $r = $r -bor (($b -band 0x7f) -shl $shift); $shift += 7 } while ($b -band 0x80); $r }
    while ($at[0] -lt $bytes.Length) {
        $id = $bytes[$at[0]++]
        $size = & $leb
        $end = $at[0] + $size
        if ($id -eq 7) {
            $count = & $leb
            for ($i = 0; $i -lt $count; $i++) {
                $length = & $leb
                $name = [Text.Encoding]::UTF8.GetString($bytes, $at[0], $length)
                $at[0] += $length
                $kind = $bytes[$at[0]++]
                $null = & $leb
                if ($kind -eq 0) { $name }
            }
        }
        $at[0] = $end
    }
}

if (!(BuildSpiral "$ScriptDir/$projectName.spi" "$ScriptDir/$projectName.rs" "polyglot/apps/chat/contract")) {
    throw "RUST-FAILED polyglot/apps/chat/contract / compile"
}
{ cargo +$contractToolchain build --release --target wasm32-unknown-unknown -p $projectName } | Invoke-Block -EnvironmentVariables @{ "AUTOMATION" = "False" }
$cargoTargetDir = (cargo +$contractToolchain metadata --format-version 1 --no-deps | ConvertFrom-Json).target_directory
$rustWasm = "$cargoTargetDir/wasm32-unknown-unknown/release/$projectName.wasm"
if (!(Test-Path $rustWasm)) { throw "RUST-FAILED polyglot/apps/chat/contract / no $rustWasm" }
$rustExports = @(Get-WasmExports $rustWasm)
$contractMethods = @('new', 'is_valid_alias', 'generate_cid', 'generate_cid_borsh', 'claim_alias', 'get_account_info', 'get_alias_map', 'get_alias_map_borsh')
$missing = @($contractMethods | Where-Object { $_ -notin $rustExports })
if ($missing) { throw "RUST-FAILED polyglot/apps/chat/contract / the wasm doesn't export $($missing -join ', ') (exports: $($rustExports -join ', '))" }
New-Item dist -ItemType Directory -Force | Out-Null
Copy-Item $rustWasm dist/chat.wasm -Force
Write-Output "RUST-COMPILED polyglot/apps/chat/contract / dist/chat.wasm $((Get-Item dist/chat.wasm).Length) B / exports $($rustExports -join ', ')"

if (!$fast -and !$SkipTests) {
    { pwsh tests/build.ps1 } | Invoke-Block
    Write-Output "RUST-OK polyglot/apps/chat/contract / the NEAR sandbox tests passed on dist/chat.wasm"
}

if ($env:CI) {
    ClearCargoTarget ".."
}
