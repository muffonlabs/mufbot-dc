#!/bin/bash

# get the latest release from the MUFbot repository
# then install it here

GREEN='\033[0;32m'
NC='\033[0m'
RED='\033[0;31m'

sp="/-\|"
sc=0

spin() {
    printf "\b${sp:sc++:1}"
    ((sc==${#sp})) && sc=0
}

startspin() {
    printf " "
    while true; do
        spin
        sleep 0.1
    done
}

endspin() {
    printf "\r%s\n" "$@"
}

# check if the script is run as root
if [ "$EUID" -ne 0 ]; then
    echo -e "${RED}Please run as root${NC}"
    exit
fi

START_SCRIPT="/root/.scripts/mufbot-start.sh"
SERVICE_NAME=$(grep SERVICE_NAME .env | cut -d '=' -f 2)
SERVICE_FILE="/etc/systemd/system/$SERVICE_NAME.service"
URL="https://api.github.com/repos/muffonlabs/mufbot-dc/releases/latest"
LATEST=$(curl -s $URL | grep "browser_download_url" | cut -d '"' -f 4)

echo -n "$GREEN[+] Downloading the latest release of MUFbot...$NC"
startspin
wget $latest
chmod +x mufbot-dc
endspin "$GREEN[+] Download complete$NC"


# stop the service
echo -n "$GREEN[+] Stopping the service...$NC"
startspin
if [ -f $SERVICE_FILE ]; then
    systemctl stop $SERVICE_NAME
    rm $SERVICE_FILE
fi
endspin "$GREEN[+] Service stopped$NC"

# create start script
echo -n "$GREEN[+] Creating the start script...$NC"
startspin
if [ -f $START_SCRIPT ]; then
    rm $START_SCRIPT
fi

mkdir -p /root/.scripts

echo "#!/bin/bash
cd ~/mufbot-dc
chmod +x mufbot-dc
./mufbot-dc" > $START_SCRIPT
chmod +x $START_SCRIPT

endspin "$GREEN[+] Start script created$NC"

# create the service file
echo -n "$GREEN[+] Creating the service file...$NC"
startspin

echo "[Unit]
Description=MUFbot Discord Client
After=multi-user.target

[Service]
Type=simple
ExecStart=/bin/bash $START_SCRIPT
Restart=on-failure

[Install]
WantedBy=multi-user.target" > $SERVICE_FILE

endspin "$GREEN[+] Service file created$NC"

# start the service
echo -n "$GREEN[+] Starting the service...$NC"
startspin
systemctl daemon-reload
systemctl enable $SERVICE_NAME
systemctl start $SERVICE_NAME
endspin "$GREEN[+] Service started$NC"
