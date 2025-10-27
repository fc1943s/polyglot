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

rustup install nightly-2025-05-09
rustup default nightly-2025-05-09
rustup +nightly-2025-05-09 target add wasm32-unknown-unknown
rustup +nightly-2025-05-09 component add clippy rust-src rustfmt

rustup install nightly-2024-07-14
rustup +nightly-2024-07-14 target add wasm32-unknown-unknown
rustup +nightly-2024-07-14 component add clippy rust-src rustfmt

rustup install nightly-2024-10-07

if ($env:CI) {
    rustup uninstall stable
}

if (!(Search-Command "nix")) {
    if (!$IsWindows) {
        if (!(Search-DotnetSdk "9")) {
            curl -sSL https://dot.net/v1/dotnet-install.sh | bash /dev/stdin --version "9.0.202"
        }

        if (!(Test-Path "~/.bun/bin/bun")) {
            curl -sSL https://bun.sh/install | bash
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

    if ($IsWindows) {
        { pwsh init.ps1 -init 1 } | Invoke-Block -Linux
    }

    { pip install -r ../requirements.txt } | Invoke-Block -OnError Continue
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
        sudo apt-key add GPG-KEY-pmanager.asc

        echo "deb http://binaries2.erlang-solutions.com/ubuntu/ jammy-esl-erlang-25 contrib" | sudo tee -a /etc/apt/sources.list
        sudo apt update

        # iwr https://packages.erlang-solutions.com/erlang-solutions_1.0_all.deb -OutFile erlang-solutions_1.0_all.deb; sudo dpkg -i erlang-solutions_1.0_all.deb

        # sudo apt install -y esl-erlang
        sudo apt install -y erlang

        Set-Location /tmp
        git clone https://github.com/gleam-lang/gleam.git
        Set-Location gleam
        make install
        $ResolvedScriptDir | Set-Location

        { gleam --version } | Invoke-Block
        { erl -version } | Invoke-Block
    }
}

{ dotnet tool restore } | Invoke-Block -OnError Continue

{ dotnet paket restore } | Invoke-Block

Set-Location $ResolvedScriptDir

{ pwsh symlinks.ps1 } | Invoke-Block

{ pwsh ../lib/rust/fable/build.ps1 } | Invoke-Block

$gitPath = ResolveLink (GetFullPath "../..")

Write-Output "polyglot/scripts/init.ps1 / Get-Location: $(Get-Location) / gitPath: $gitPath"


$url = git ls-remote --get-url
$owner = ($url -split '/' | Select-Object -Last 2 | Select-Object -First 1) -replace '\.git$', '' ?? $env:GITHUB_REPOSITORY_OWNER
Write-Output "init.ps1 / url: $url / owner: $owner"

Set-Location (New-Item $gitPath -ItemType Directory -Force)
git clone --recurse-submodules https://github.com/$owner/spiral.git
{ git pull } | Invoke-Block -Location spiral -OnError Continue
Set-Location $ResolvedScriptDir

Write-Output "polyglot/scripts/init.ps1 / Get-Location: $(Get-Location) / gitPath: $gitPath"

{ pwsh ../../spiral/scripts/init.ps1 } | Invoke-Block

EnsureSymbolicLink -Path "$ResolvedScriptDir/../deps/spiral" -Target "$ResolvedScriptDir/../../spiral"

if (!$fast) {
    { pwsh dep_dotnet-interactive.ps1 } | Invoke-Block
    { pwsh dep_dotnet-repl.ps1 } | Invoke-Block
}

Invoke-Dib init.dib

{ pwsh $(ResolveLink "../deps/spiral/apps/spiral/build.ps1") -SkipPreBuild 1 } | Invoke-Block
