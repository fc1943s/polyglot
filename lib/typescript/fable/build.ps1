param(
    $fast,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../../scripts/core.ps1


if (!$fast) {
    { dotnet fable --optimize --lang ts --extension .ts --noCache } | Invoke-Block
    { dotnet fable ../../../deps/Fable/src/fable-library-ts/Fable.Library.TypeScript.fsproj --lang ts --extension .ts --noCache } | Invoke-Block -OnError Continue
    Remove-Item ../../../deps/Fable/src/fable-library-ts/fable_modules -Recurse -Force -ErrorAction Ignore
}

$paths = @(
    "../../../deps/Fable/src/fable-library-ts/Array.ts",
    "../../../deps/Fable/src/fable-library-ts/Seq.ts",
    "../../../deps/Fable/src/fable-library-ts/Choice.ts",
    "../../../deps/Fable/src/fable-library-ts/FSharp.Collections.ts",
    "../../../deps/Fable/src/fable-library-ts/FSharp.Core.CompilerServices.ts",
    "../../../deps/Fable/src/fable-library-ts/FSharp.Core.ts",
    "../../../deps/Fable/src/fable-library-ts/Global.ts",
    "../../../deps/Fable/src/fable-library-ts/List.ts",
    "../../../deps/Fable/src/fable-library-ts/Map.ts",
    "../../../deps/Fable/src/fable-library-ts/MutableMap.ts",
    "../../../deps/Fable/src/fable-library-ts/MutableSet.ts",
    "../../../deps/Fable/src/fable-library-ts/Native.ts",
    "../../../deps/Fable/src/fable-library-ts/Random.ts",
    "../../../deps/Fable/src/fable-library-ts/Range.ts",
    "../../../deps/Fable/src/fable-library-ts/Result.ts",
    "../../../deps/Fable/src/fable-library-ts/Seq2.ts",
    "../../../deps/Fable/src/fable-library-ts/Set.ts",
    "../../../deps/Fable/src/fable-library-ts/System.Collections.Generic.ts",
    "../../../deps/Fable/src/fable-library-ts/System.Text.ts",
    "../../../deps/Fable/src/fable-library-ts/SystemException.ts"
)

foreach ($path in $paths) {
    (Get-Content $path) `
        -replace "`"\./fable_modules/fable-library-ts\.[\-\w\d\.]+/", "`"./" `
        | Set-Content $path
}
