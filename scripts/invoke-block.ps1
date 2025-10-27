param(
    [Parameter(Position = 0, Mandatory)]
    [string] $code,

    [Parameter(Position = 1, ValueFromRemainingArguments)]
    [Object[]]$_args,

    $ScriptDir = $PSScriptRoot
)
$ErrorActionPreference = "Stop"
. $ScriptDir/core.ps1


$mergedArgs = @{
    "ScriptBlock" = [ScriptBlock]::Create($code)
}
$key = $null
foreach ($item in $_args) {
    if ($item -match "^-") {
        if ($key) {
            $mergedArgs[$key] = $True
        }
        $key = $item -replace "^-"
    } elseif ($null -ne $key) {
        if ($item.Contains("=")) {
            $item = ConvertFrom-StringData -StringData $item
        }
        $mergedArgs[$key] = $item
        $key = $null
    }
}
if ($key) {
    $mergedArgs[$key] = $True
}
# Write-Output "invoke-block.ps1 / Get-Location: $(Get-Location) / path: $path / _args: $($mergedArgs | ConvertTo-Json)"

{ Invoke-Block @mergedArgs } | Invoke-Block
