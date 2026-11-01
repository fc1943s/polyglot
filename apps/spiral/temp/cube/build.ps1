param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../../../scripts/core.ps1
. ../../../../deps/spiral/lib/spiral/lib.ps1

$projectName = "cube"
$expected = "cube: 60 frames, checksum 970392"
$targetDir = GetTargetDir $projectName
New-Item -ItemType Directory -Force $targetDir | Out-Null
$exe = $IsWindows ? ".exe" : ""

$backends = [ordered]@{
    "C" = @{ Out = "$projectName.c"; Build = { gcc -O2 "$projectName.c" -o "$targetDir/cube_c$exe" -lm }; Run = { & "$targetDir/cube_c$exe" } }
    "Rust" = @{ Out = "$projectName.rs"; Build = { rustc -O --edition 2021 "$projectName.rs" -o "$targetDir/cube_rs$exe" }; Run = { & "$targetDir/cube_rs$exe" } }
    "Delphi" = @{ Out = "$projectName.pas"; Build = { fpc -O2 -v0 "-FE$targetDir" "-FU$targetDir" "$projectName.pas" }; Run = { & "$targetDir/$projectName$exe" } }
    "Fsharp" = @{ Out = "$projectName.fsx"; Build = { }; Run = { dotnet fsi --quiet --nowarn:25 "$projectName.fsx" } }
    "TypeScript" = @{ Out = "$projectName.ts"; Build = { }; Run = { bun run "$projectName.ts" } }
    "Python + Cuda" = @{ Out = "$projectName.py"; Build = { }; Run = { python "$projectName.py" } }
    "Lua" = @{ Out = "$projectName.lua"; Build = { }; Run = { lua "$projectName.lua" } }
    "Zig" = @{ Out = "$projectName.zig"; Build = { zig build-exe -O ReleaseFast -fno-emit-implib "$projectName.zig" "-femit-bin=$targetDir/cube_zig$exe" }; Run = { & "$targetDir/cube_zig$exe" } }
}

$failed = @()
foreach ($backend in $backends.Keys) {
    $b = $backends[$backend]
    if (!(BuildSpiral "$projectName.spi" $b.Out "apps/spiral/temp/cube ($backend)" -Backend $backend)) { $failed += "$backend compile"; continue }
    & $b.Build
    if ($LASTEXITCODE) { $failed += "$backend build"; continue }
    $sw = [Diagnostics.Stopwatch]::StartNew()
    $output = @(& $b.Run | ForEach-Object { "$_" })
    $line = $output | Where-Object { $_ -like "cube: *" } | Select-Object -Last 1
    Write-Output "polyglot/apps/spiral/temp/cube/build.ps1 / $backend / $line / $($sw.ElapsedMilliseconds) ms"
    if ($line -ne $expected) { $failed += "$backend output '$line'" }
}
if ($failed) { throw "cube: $($failed -join '; ')" }
Write-Output "polyglot/apps/spiral/temp/cube/build.ps1 / all $($backends.Count) backends: $expected"
