installed() { [ "$(dpkg-query -W -f='${Status}' "$1" 2>/dev/null)" = "install ok installed" ]; }

cargo_build_script_linker=build-essential
if ! installed snapd || ! installed unzip || ! installed "$cargo_build_script_linker"; then
    apt update
    apt install -y snapd unzip "$cargo_build_script_linker"

    df -h
    apt-get autoremove -y
    apt-get clean
    df -h
fi

if ! snap list powershell >/dev/null 2>&1; then
    snap install powershell --classic
fi
