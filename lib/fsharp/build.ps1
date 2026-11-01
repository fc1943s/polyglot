param(
    $fast,
    $sequential,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../scripts/core.ps1

$livebook = Join-Path $ScriptDir "../../deps/spiral/apps/cells/spi/run_notebook.ps1"
$notebooks = @("Async", "AsyncSeq", "Common", "CommonFSharp", "FileSystem", "Runtime")

if (!$fast) {
    $runs = $notebooks | ForEach-Object -ThrottleLimit ($sequential ? 1 : 3) -Parallel {
        $notebook = $_
        $arguments = @("--path", "$using:ScriptDir/$notebook.livemd", "--output-path", "$using:ScriptDir/$notebook.livemd.ipynb", "--no-spi")
        $log = @()
        $exitCode = 1
        foreach ($attempt in 1..3) {
            $log = pwsh -NoProfile -File $using:livebook @arguments 2>&1 | ForEach-Object { "$_" }
            $exitCode = $LASTEXITCODE
            if ($exitCode -eq 0) { break }
        }
        [pscustomobject]@{ Notebook = $notebook; ExitCode = $exitCode; Log = $log }
    }
    $runs | Sort-Object Notebook | ForEach-Object { Write-Output "polyglot/lib/fsharp/build.ps1 / $($_.Notebook) / exit $($_.ExitCode)" }
    $failed = @($runs | Where-Object ExitCode -ne 0)
    foreach ($run in $failed) { $run.Log | Select-Object -Last 30 | ForEach-Object { Write-Output "polyglot/lib/fsharp/build.ps1 / $($run.Notebook) / $_" } }
    if ($failed) { throw "polyglot/lib/fsharp/build.ps1 / notebooks failed: $(($failed | ForEach-Object Notebook) -join ', ')" }
}

foreach ($notebook in $notebooks) {
    { pwsh -NoProfile -File $livebook --path "$ScriptDir/$notebook.livemd" --no-spi --fs-path "$ScriptDir/$notebook.fs" --export-only } | Invoke-Block
}
