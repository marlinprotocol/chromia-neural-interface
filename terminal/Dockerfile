FROM ubuntu:22.04

# Copy files
COPY ./ /app

#Change Workdir
WORKDIR /app


RUN apt-get update && apt-get install -y wget build-essential curl && apt-get clean

# Install Node dependencies.
RUN wget https://nodejs.org/dist/v18.17.0/node-v18.17.0-linux-x64.tar.xz \
    && mkdir -p /node && tar -xf node-v18.17.0-linux-x64.tar.xz -C /node \
    && chmod +x -R /node/node-v18.17.0-linux-x64

# Install docker
RUN curl -fsSL https://get.docker.com -o get-docker.sh \
    && sh get-docker.sh

# Setup Path
ENV PATH="/node/node-v18.17.0-linux-x64/bin:${PATH}"
RUN npm --version
RUN node --version 
RUN npm install

EXPOSE 6060

#Start App
ENTRYPOINT [ "node", "./src/server.js" ]