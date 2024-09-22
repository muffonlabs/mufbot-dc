#!/bin/bash

GREEN='\033[0;32m'
NC='\033[0m'
RED='\033[0;31m'

spin() {
    local pid=$1
    local delay=0.1
    local spinstr='|/-\'
    while [ "$(ps a | awk '{print $1}' | grep $pid)" ]; do
        local temp=${spinstr#?}
        printf " [%c]  " "$spinstr"
        local spinstr=$temp${spinstr%"$temp"}
        sleep $delay
        printf "\b\b\b\b\b\b"
    done
    printf "    \b\b\b\b"
}

run_with_spinner() {
    local command="$1"
    local message="$2"
    echo -en "$message"
    eval "$command" &
    spin $!
    wait $!
    echo -e "${GREEN} DONE${NC}"
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

# check if the service name is set
if [ -z "$SERVICE_NAME" ]; then
    echo -e "${RED}Please set the SERVICE_NAME in the .env file${NC}"
    exit
fi

# check if the latest release is available
if [ -z "$LATEST" ]; then
    echo -e "${RED}Could not get the latest release${NC}"
    exit
fi


if [ -f "mufbot-dc" ]; then
    # delete the old version
    run_with_spinner "rm -rf mufbot-dc" "${GREEN}[+] Deleting the old version of mufbot...${NC}"
fi

# install dependencies
if ! command -v wget &> /dev/null; then
    run_with_spinner "apt-get -qq update && apt-get -qq install -y wget" "${GREEN}[+] Installing wget...${NC}"
fi

# download the latest release
run_with_spinner "wget -q $LATEST && chmod +x mufbot-dc" "${GREEN}[+] Downloading the latest release of mufbot...${NC}"

# stop the service
if [ -f $SERVICE_FILE ]; then
    run_with_spinner "systemctl stop $SERVICE_NAME && rm $SERVICE_FILE" "${GREEN}[+] Stopping the service...${NC}"
fi

# create start script
run_with_spinner "mkdir -p /root/.scripts && echo '#!/bin/bash
cd ~/mufbot-dc
chmod +x mufbot-dc
./mufbot-dc' > $START_SCRIPT && chmod +x $START_SCRIPT" "${GREEN}[+] Creating the start script...${NC}"

# create the service file
run_with_spinner "echo '[Unit]
Description=mufbot Discord Bot
After=multi-user.target
[Service]
Type=simple
ExecStart=/bin/bash $START_SCRIPT
Restart=on-failure
[Install]
WantedBy=multi-user.target' > $SERVICE_FILE" "${GREEN}[+] Creating the service file...${NC}"

# start the service
run_with_spinner "systemctl daemon-reload && systemctl enable $SERVICE_NAME && systemctl start $SERVICE_NAME" "${GREEN}[+] Starting the service...${NC}"

echo -e "${GREEN}mufbot has been installed successfully!${NC}"
