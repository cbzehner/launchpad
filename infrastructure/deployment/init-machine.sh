#!/usr/bin/env bash
set -Eefuo pipefail

# Navigate to the script location so any subsequent commands are relative to the script.
script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)

## Operating System Maintenence
apt update && apt upgrade && apt autoremove

## Install Dependencies
# docker-compose (https://docs.docker.com/compose/install/)
sudo curl -L "https://github.com/docker/compose/releases/download/1.29.1/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
# just (https://github.com/casey/just#pre-built-binaries)
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/local/bin

## Setup GitHub SSH key.
# Set the local variables
private_key_path="$HOME/.ssh/github_id_rsa"
public_key_path="$private_key_path.pub"
email="admin@launchpad.rs"

# Generate the key
ssh-keygen -t ed25519 -C "$email" -f "$private_key_path"

# Add the key to the ssh-agent
eval "$(ssh-agent -s)"
ssh-add $private_key_path

# Add the key to GitHub
cat $public_key_path
read -p "Upload your new public key to GitHub (https://github.com/settings/keys). Once that is complete, press any key to continue." unused_user_input

## Setup Application
# Clone the folder into the root
git clone git@github.com:cbzehner/launchpad.git
cd launchpad

# Create a directory to store the database volume
mkdir -p ./.data/postgres-data

read -p "Create a dotenv file (.env) with the necessary environmental variables. Once that is complete, press any key to continue." unused_user_input

# Start the application
just production launch