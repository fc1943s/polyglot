param(
    $fast,
    $SkipNotebook,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../scripts/core.ps1
. ../../deps/spiral/lib/spiral/lib.ps1


{ pwsh ../../deps/spiral/apps/compiler/build.ps1 -fast 1 } | Invoke-Block

if (!$fast -and !$SkipNotebook) {
    { . ../../deps/spiral/workspace/target/release/spiral$(_exe) dib --path Supervisor.dib --retries 3 } | Invoke-Block
}

{ . ../parser/dist/DibParser$(_exe) Supervisor.dib fs } | Invoke-Block

$runtime = $fast -or $env:CI ? @("--runtime", ($IsWindows ? "win-x64" : "linux-x64")) : @()
$builderArgs = @("Supervisor.fs", $runtime, "--packages", "Argu", "FSharp.Control.AsyncSeq", "FSharp.Json", "Microsoft.AspNetCore.SignalR.Client", "System.Reactive.Linq", "Hopac", "FSharpx.Collections", "FParsec", "System.Management", "--modules", @(GetFsxModules), "lib/fsharp/Common.fs", "lib/fsharp/CommonFSharp.fs", "lib/fsharp/Async.fs", "lib/fsharp/AsyncSeq.fs", "lib/fsharp/Runtime.fs", "lib/fsharp/FileSystem.fs", "deps/spiral/apps/compiler/spiral_compiler.fs")
{ . ../builder/dist/Builder$(_exe) @builderArgs } | Invoke-Block

if (!$fast) {
    { . ../../deps/spiral/workspace/target/release/spiral$(_exe) dib --path Eval.dib --retries 3 } | Invoke-Block
}

{ . ../parser/dist/DibParser$(_exe) Eval.dib fs } | Invoke-Block

Write-Output "polyglot/apps/spiral/build.ps1 / `$env:CI:'$env:CI'"
