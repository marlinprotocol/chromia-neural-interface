# Chromia Neural Interface

This repository demonstrates using Chromia to store AI Agent short-term and long-term memories on chain. It also demonstrates a simple neural interface for interacting with the AI Agent.

## Run using docker compose

### Prerequisites
- Install [Docker](https://docs.docker.com/engine/install/ubuntu/)

### Steps
1. Install dependencies and setup
   ```sh
   bash build.sh
   ```

2. Navigate to http://host-ip-or-localhost:6060 and run:
    ```sh
    chr node start
    ```

3. Open a new tab & navigate to http://host-ip-or-localhost:6060 and run:
   ```sh
   bun run dev
   ```
