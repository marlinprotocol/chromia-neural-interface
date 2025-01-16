#!/bin/bash

# Install required dependencies
echo "Installing dependencies"
apt-get update
apt-get install -y apt-transport-https ca-certificates curl software-properties-common

# Install docker
# Install docker
curl -fsSL https://get.docker.com -o get-docker.sh \
    && sh get-docker.sh

# daemon 
systemctl start docker
systemctl enable docker

# check version
docker --version

# verify installation
docker run hello-world

# Start app
npm run start 

