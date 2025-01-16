# Chromia Neural Interface

This repository demonstrates using Chromia to store AI Agent short-term and long-term memories on chain. It also demonstrates a simple neural interface for interacting with the AI Agent.

## How to Run

### Prerequisites
- Install [Docker](https://docs.docker.com/engine/install/ubuntu/)
- Install [npm](https://www.digitalocean.com/community/tutorials/how-to-install-node-js-on-ubuntu-20-04)
- Fetch [Groq API key](https://console.groq.com/keys)

### Steps
1. Install dependencies and setup
   ```sh
   bash build.sh
   ```
   Update `XAI_API_KEY` in `.env`.

2. Start the interactive terminal browser (in a new terminal) 
    ```sh
    cd terminal
    npm run start
    ```

3. Navigate to http://host-machine-ip-or-localhost:6060 and start bun dev server
   ```sh
   bash start.sh
   ```

nix build --impure --expr '((builtins.getFlake "github:marlinprotocol/oyster-monorepo?rev=f27cbe66bef2f0749ff3afee3aeaff232e933ec0").packages.x86_64-linux.musl.sdks.docker-enclave.override { 
  compose = ./docker-compose.yml;
  dockerImages = [
    ./docker-images/custom-memcached.tar
  ];
}).default' -vL
