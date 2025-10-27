param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../../scripts/core.ps1


{ pwsh ./cube/build.ps1 } | Invoke-Block

{ pwsh ./extension/build.ps1 } | Invoke-Block

{ pwsh ./test/build.ps1 } | Invoke-Block


if (!$env:CI) {
    if ($IsLinux) {
        { sudo apt-get update } | Invoke-Block -Linux -Distro ubuntu
        { sudo add-apt-repository ppa:thomas-schiex/blender -y } | Invoke-Block -Linux -Distro ubuntu -OnError Continue
        { sudo apt install -y blender } | Invoke-Block -Linux -Distro ubuntu -OnError Continue
    }

    { pwsh ./blender/build.ps1 } | Invoke-Block
}
