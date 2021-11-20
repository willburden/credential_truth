echo "Downloading..."
wget https://github.com/willburden/credential_truth/releases/latest/download/docker-credential-truth -O /tmp/docker-credential-truth 2> /dev/null
echo "Installing..."
chmod +x /tmp/docker-credential-truth > /dev/null
mv -f /tmp/docker-credential-truth /usr/bin > /dev/null
echo "Finished."
