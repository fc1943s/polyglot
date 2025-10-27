param(
    $fast,
    $sequential,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../scripts/core.ps1


if (!$fast) {
    { . ../../apps/spiral/dist/Supervisor$(_exe) `
        --exit-on-error `
        $(!$sequential ? @("--parallel") : @()) `
        --execute-command "../../deps/spiral/workspace/target/release/spiral$(_exe) dib --path Async.dib --retries 3" `
        --execute-command "../../deps/spiral/workspace/target/release/spiral$(_exe) dib --path AsyncSeq.dib --retries 3" `
        --execute-command "../../deps/spiral/workspace/target/release/spiral$(_exe) dib --path Common.dib --retries 3" `
        --execute-command "../../deps/spiral/workspace/target/release/spiral$(_exe) dib --path CommonFSharp.dib --retries 3" `
        --execute-command "../../deps/spiral/workspace/target/release/spiral$(_exe) dib --path FileSystem.dib --retries 3" `
        --execute-command "../../deps/spiral/workspace/target/release/spiral$(_exe) dib --path Runtime.dib --retries 3" `
    } | Invoke-Block
}

{ . ../../apps/parser/dist/DibParser$(_exe) Async.dib fs AsyncSeq.dib fs Common.dib fs CommonFSharp.dib fs FileSystem.dib fs Runtime.dib fs } | Invoke-Block
