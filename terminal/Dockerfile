# FROM ubuntu:22.04
FROM node:18.17.0

# Copy files
COPY ./ /app

#Change Workdir
WORKDIR /app

RUN apt-get update \
    && apt-get install -y \
    # wget \
    # build-essential \
    curl \
    && apt-get clean

# Install docker
RUN curl -fsSL https://get.docker.com -o get-docker.sh \
    && sh get-docker.sh

# Verify installation
# RUN docker run hello-world

# RUN usermod -aG docker ubuntu

RUN npm install

EXPOSE 6060

#Start App
ENTRYPOINT [ "node", "./src/server.js" ]