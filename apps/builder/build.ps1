param(
    $fast,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../scripts/core.ps1
. ../../deps/spiral/lib/spiral/lib.ps1


$dibParserExe = "../parser/dist/DibParser$(_exe)"
if ($fast -and (Test-Path $dibParserExe)) {
    { . $dibParserExe Builder.dib fs } | Invoke-Block
} else {
    { dotnet repl --run ../parser/DibParser.dib --exit-after-run } | Invoke-Block -EnvironmentVariables @{ "ARGS" = "Builder.dib fs" }
}

$runtime = $fast -or $env:CI ? @("--runtime", ($IsWindows ? "win-x64" : "linux-x64")) : @()
$builderArgs = @("Builder.fs", [String]::Join(" ", $runtime), "--packages", "Argu", "FSharp.Control.AsyncSeq", "System.Reactive.Linq", "--modules", [String]::Join(" ", $(GetFsxModules)), "lib/fsharp/Common.fs", "lib/fsharp/CommonFSharp.fs", "lib/fsharp/Async.fs", "lib/fsharp/AsyncSeq.fs", "lib/fsharp/Runtime.fs", "lib/fsharp/FileSystem.fs")
{ . ../../deps/spiral/workspace/target/release/spiral$(_exe) dib --path Builder.dib } | Invoke-Block -EnvironmentVariables @{ "ARGS" = [String]::Join(" ", $builderArgs) }

Write-Output "polyglot/apps/builder/build.ps1 / `$env:CI:'$env:CI'"
