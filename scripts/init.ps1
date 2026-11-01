param(
    $fast,
    $init,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ./core.ps1

$ResolvedScriptDir = ResolveLink $ScriptDir
$ResolvedScriptDir | Set-Location


Write-Output "polyglot/scripts/init.ps1 / ScriptDir: $ScriptDir / ResolvedScriptDir: $ResolvedScriptDir"

if (!(Search-Command "rustup")) {
    if ($IsWindows) {
        $rustupExePath = Join-Path -Path ([System.IO.Path]::GetTempPath()) -ChildPath "rustup-init.exe"
        Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile $rustupExePath -ErrorAction Stop
        Start-Process -FilePath $rustupExePath -Wait
    }
    else {
        $rustupScriptPath = Join-Path -Path ([System.IO.Path]::GetTempPath()) -ChildPath "rustup.sh"
        Invoke-WebRequest -Uri "https://sh.rustup.rs" -OutFile $rustupScriptPath -ErrorAction Stop
        /bin/sh $rustupScriptPath -y
        $env:PATH = "~/.cargo/bin:$env:PATH"
    }
}

rustup install nightly-2026-10-18
rustup default nightly-2026-10-18
rustup +nightly-2026-10-18 target add wasm32-unknown-unknown
rustup +nightly-2026-10-18 component add clippy rust-src rustfmt

rustup install nightly-2024-07-14
rustup +nightly-2024-07-14 target add wasm32-unknown-unknown
rustup +nightly-2024-07-14 component add clippy rust-src rustfmt

rustup install nightly-2026-10-18

rustup install nightly-2026-10-18

if ($env:CI) {
    rustup uninstall stable
}

if (!(Search-Command "nix")) {
    if (!$IsWindows) {
        if (!(Search-DotnetSdk "9")) {
            curl -sSL https://dot.net/v1/dotnet-install.sh | bash /dev/stdin --version "9.0.202"
        }

        if (!(Test-Path "~/.bun/bin/bun")) {
            { curl -sSL https://bun.sh/install | bash } | Invoke-Block
            $env:PATH = "~/.bun/bin:$env:PATH"
        }

        if (!(Search-Command "pip")) {
            sudo apt install -y python3-pip
        }
    }
    else {
        if (!(Test-Path "~/.bun/bin/bun.exe")) {
            { Invoke-RestMethod bun.sh/install.ps1 | Invoke-Expression } | Invoke-Block -OnError Continue
        }

        { choco install rsync -y } | Invoke-Block -OnError Continue

        $distributions = wsl --list --quiet
        if (-not ($distributions -like "Ubuntu")) {
            wsl --install Ubuntu --no-launch
        }
    }

    { sudo sh init.sh } | Invoke-Block -Linux -OnError Continue
    { sh init.sh } | Invoke-Block -Linux -OnError Continue

    if ($IsWindows) {
        { pwsh init.ps1 -init 1 } | Invoke-Block -Linux
    }

    $pipArgs = $IsLinux ? @("--break-system-packages") : @()
    { pip install @pipArgs -r ../requirements.txt } | Invoke-Block -OnError Continue
}
else {
    mkdir -p ~/.bun/bin
    ln -s /run/current-system/sw/bin/bun ~/.bun/bin/bun
    ln -s /run/current-system/sw/bin/bunx ~/.bun/bin/bunx
}

if ($init) {
    exit
}

if (!(Search-Command "gleam")) {
    if (!$IsWindows) {
        wget https://binaries2.erlang-solutions.com/GPG-KEY-pmanager.asc
        { sudo apt-key add GPG-KEY-pmanager.asc } | Invoke-Block -OnError Continue
        { apt-key add GPG-KEY-pmanager.asc } | Invoke-Block -OnError Continue

        { echo "deb http://binaries2.erlang-solutions.com/ubuntu/ jammy-esl-erlang-26 contrib" | sudo tee -a /etc/apt/sources.list } | Invoke-Block -OnError Continue
        { echo "deb http://binaries2.erlang-solutions.com/ubuntu/ jammy-esl-erlang-26 contrib" | tee -a /etc/apt/sources.list } | Invoke-Block -OnError Continue

        { sudo apt update } | Invoke-Block -OnError Continue
        { apt update } | Invoke-Block -OnError Continue


        { sudo apt install -y erlang } | Invoke-Block -OnError Continue
        { apt install -y erlang } | Invoke-Block -OnError Continue

        Set-Location /tmp
        git clone https://github.com/gleam-lang/gleam.git
        Set-Location gleam
        make install
        $ResolvedScriptDir | Set-Location

        { gleam --version } | Invoke-Block
        { erl -version } | Invoke-Block
    }
}

Set-Location $ResolvedScriptDir

{ pwsh symlinks.ps1 } | Invoke-Block


$gitPath = ResolveLink (GetFullPath "../..")

Write-Output "polyglot/scripts/init.ps1 / Get-Location: $(Get-Location) / gitPath: $gitPath"


$url = git ls-remote --get-url
$owner = ($url -split '/' | Select-Object -Last 2 | Select-Object -First 1) -replace '\.git$', '' ?? $env:GITHUB_REPOSITORY_OWNER
$domain = ($url -split '/' | Select-Object -Last 3 | Select-Object -First 1) ?? $env:GITHUB_SERVER_URL -replace 'https?://', ''
Write-Output "init.ps1 / url: $url / owner: $owner / domain: $domain"

Set-Location (New-Item $gitPath -ItemType Directory -Force)
git clone --recurse-submodules https://$domain/$owner/spiral.git
{ git pull } | Invoke-Block -Location spiral -OnError Continue
Set-Location $ResolvedScriptDir

Write-Output "polyglot/scripts/init.ps1 / Get-Location: $(Get-Location) / gitPath: $gitPath"

{ pwsh ../../spiral/scripts/init.ps1 } | Invoke-Block

EnsureSymbolicLink -Path "$ResolvedScriptDir/../deps/spiral" -Target "$ResolvedScriptDir/../../spiral"

Invoke-PwshNotebook init.livemd


{ pwsh $(ResolveLink "../deps/spiral/apps/spiral/build.ps1") } | Invoke-Block
