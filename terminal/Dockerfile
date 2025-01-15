# Dockerfile for frontend
FROM ubuntu:20.04

# Set environment variables to avoid prompts
ENV DEBIAN_FRONTEND=noninteractive

# Install nodejs and npm
RUN apt-get update && apt-get install -y --no-install-recommends \
    nodejs \
    npm 

# Set working directory
WORKDIR /app/terminal

# Copy the package files
COPY package.json package.json

RUN npm install

# Copy the source files

# Expose port 6060
EXPOSE 6060

# Command to run the terminal server
CMD ["npm", "run", "start"]
