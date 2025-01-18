FROM ubuntu:20.04

# Set environment variables to avoid prompts
ENV DEBIAN_FRONTEND=noninteractive

# Install dependencies for chr-node
RUN apt-get update && apt-get install -y \
    curl gnupg2 ca-certificates software-properties-common

# Install Chromia CLI
RUN curl -fsSL https://apt.chromia.com/chromia.gpg -o /usr/share/keyrings/chromia.gpg
RUN echo "deb [signed-by=/usr/share/keyrings/chromia.gpg] https://apt.chromia.com stable main" \
    > /etc/apt/sources.list.d/chromia.list
RUN apt-get update && apt-get install -y chr

# Set working directory
WORKDIR /app

# Install Bun
RUN apt-get install -y unzip
RUN curl -fsSL https://bun.sh/install | bash

# Add these lines to ensure bun is in PATH
ENV BUN_INSTALL="/root/.bun"
ENV PATH="/root/.bun/bin:$PATH"

COPY . .

RUN ["bun", "install"]

RUN apt-get install -y build-essential wget xterm

# Install Node dependencies.
RUN wget https://nodejs.org/dist/v18.17.0/node-v18.17.0-linux-x64.tar.xz \
    && mkdir -p /node && tar -xf node-v18.17.0-linux-x64.tar.xz -C /node \
    && chmod +x -R /node/node-v18.17.0-linux-x64

# Setup Path
ENV PATH="/node/node-v18.17.0-linux-x64/bin:${PATH}"

WORKDIR /app/terminal

RUN npm install

EXPOSE 6060

WORKDIR /app

ENTRYPOINT [ "node", "./terminal/src/server.js" ]

