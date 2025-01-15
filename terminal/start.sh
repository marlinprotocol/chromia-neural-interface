#!/bin/bash

# Install docker
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh

echo "Start chatting with chromia AI!"
docker exec -it chromia-neural-interface_chr-node_1 sh -c 'bun run dev'
