param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ./core.ps1


Set-Location ..

$dir = Split-Path -Leaf (Get-Location)
if ($dir -ne "dist") {
    throw "Invalid location (dir <> dist): $(Get-Location)"
    exit 1
}

$files = @(
    Get-ChildItem -Path . -Recurse -File -Force `
    | Where-Object {
        $_.FullName.Replace("\", "/").Contains("/dist") `
            -and ($_.Extension -eq ".exe" -or $_.Extension -eq "") `
            -and !$_.Name.StartsWith("_") `
            -and $_.Name -ne "LICENSE"
    } `
    | Select-Object FullName, DirectoryName, Name
)

# $files | ForEach-Object -ThrottleLimit 4 -Parallel {
$files | ForEach-Object {
    Write-Output "`n`npath: $($_.FullName)"

    # $retryCount = 0
    # while ($retryCount -lt 20) {
    #     if ($retryCount -gt 0) {
    #         Write-Host "Retrying upload of $($_.Name) - $retryCount ($Error)"
    #     }
    #     $Error.Clear()

    #     $output = . $(Search-Command bun) --bun start $_.FullName --cwd ../apps/ipfs
    #     $status = $output `
    #     | Select-String -Pattern "status: '(\w+)'" `
    #     | ForEach-Object { $_.Matches.Groups[1].Value }

    #     if ($status -ne "pinned") {
    #         $Error.Add("Invalid status: $output") | Out-Null
    #         $retryCount++
    #         continue
    #     }

    #     $cid = $output `
    #     | Select-String -Pattern "cid: '(\w+)'" `
    #     | ForEach-Object { $_.Matches.Groups[1].Value } `
    #     | Select-Object -First 1

    #     Write-Output "$($_.Name) - cid: '$cid'"

    #     $hash = $cid

    #     # $output = fleek ipfs add $_.FullName
    #     # $hash = $output `
    #     # | Select-String -Pattern "https://\S+/(\S+)$" `
    #     # | ForEach-Object { $_.Matches.Groups[1].Value }

    #     # if (!$hash) {
    #     #     $Error.Add("Hash error: $output") | Out-Null
    #     #     $retryCount++
    #     #     continue
    #     # }

    #     # Write-Output "hash: '$hash'"

    #     # $hash | Set-Content "$($_.DirectoryName)/$($_.Name).hash"

    #     $dwebUrl = "https://dweb.link/ipfs/$hash"
    #     # cf-ipfs.com
    #     # nftstorage.link

    #     try {
    #         $response = Invoke-WebRequest `
    #             -Uri $dwebUrl `
    #             -MaximumRedirection 10 `
    #             -Headers @{ "Range" = "bytes=1000-1001" } `
    #             -ErrorAction SilentlyContinue
    #     }
    #     catch {
    #         if ($_.Exception.Response.StatusCode -eq 504) {
    #             $Error.Clear()
    #             $Error.Add($_.Exception.Message) | Out-Null
    #             $retryCount++
    #             continue
    #         }
    #         Write-Output "Error: $($_ | ConvertTo-Json)"
    #     }

    #     if ($Error.Count -gt 0) {
    #         $retryCount++
    #         continue
    #     }

    #     $dwebCid = $response.Headers["X-Ipfs-Roots"]

    #     Write-Output "$($_.Name) - dwebCid: '$dwebCid'"

    #     $dwebCid | Set-Content "$($_.DirectoryName)/$($_.Name).cid"

    #     break
    # }

    Remove-Item $_.FullName -Force
}
if ($Error.Count -gt 0) {
    throw "Failed to process file ($Error)"
    exit 1
}

{ . ../apps/dir-tree-html/dist/DirTreeHtml$(_exe) --dir . --html index.html } | Invoke-Block
