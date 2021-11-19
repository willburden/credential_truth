#!/usr/bin/env bash
echo -e "\e[1m=== run.sh ===\e[0m"

# Install docker-credential-truth
echo -e "\e[1mInstalling docker-credential-truth...\e[0m"
wget -O - $INSTALL_SCRIPT | bash

key_id=$(cat key_id)

# Integration test
echo -e "\e[1m-  Integration test start   -\e[0m"

# Init
echo -e "\e[1mInitialising password store...\e[0m"
docker-credential-truth init $key_id

echo -e "\e[1m- Integration test complete -\e[0m"
rm -- "$0"
