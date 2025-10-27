param(
    $fast,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"

. ../../../../scripts/core.ps1
. ../../../../deps/spiral/lib/spiral/lib.ps1


$Path = GetFullPath "./build.py"
echo "blender / Path: $Path"
blender -b --python $Path
