#!/usr/bin/env bash
echo -e "\e[1m=== setup.sh ===\e[0m"

# Import GPG keys
echo -e "\e[1mImporting GPG keys...\e[0m"
gpg --import keys/public.key
gpg --import keys/private.key
key_id=$( \
    gpg --list-keys --with-colons \
    | awk -F: '/fpr/ {print $10}' \
    | head -n 1 \
)
echo -e "5\ny\n" \
    | gpg --command-fd 0 --no-tty --edit-key \
        $key_id trust

# Tidy up no-longer-useful files
echo -e "\e[1mTidying up...\e[0m"
echo $key_id > key_id
rm -r keys
rm -- "$0"
