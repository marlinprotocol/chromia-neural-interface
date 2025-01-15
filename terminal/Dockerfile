# Dockerfile for frontend
FROM ubuntu:20.04

# Set environment variables to avoid prompts
ENV DEBIAN_FRONTEND=noninteractive

# Install nodejs and npm
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    nodejs \
    npm 

# Set working directory
WORKDIR /app/terminal

# Copy the package files
COPY package.json package.json

RUN npm install

# Copy the source files
COPY ./src/client.js /app/terminal/src
COPY ./src/server.js /app/terminal/src
COPY ./src/terminal.js /app/terminal/src
COPY ./src/index.html /app/terminal/src
COPY start.sh /app/terminal

# Expose port 6060
EXPOSE 6060

# Command to run the terminal server
CMD ["npm", "run", "start"]
