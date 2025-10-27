apt update
apt install -y snapd unzip
snap install powershell --classic

df -h
apt-get autoremove -y
apt-get clean
df -h
