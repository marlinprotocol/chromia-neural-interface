# Chromia Neural Interface

This repository demonstrates using Chromia to store AI Agent short-term and long-term memories on chain. It also demonstrates a simple neural interface for interacting with the AI Agent.

![](demo.png)

## How to Run

### Prerequisites
- Install [Docker](https://docs.docker.com/engine/install/ubuntu/)
- Install [npm](https://www.digitalocean.com/community/tutorials/how-to-install-node-js-on-ubuntu-20-04)
- Fetch [Groq API key](https://console.groq.com/keys)

### Steps
1. Install dependencies and setup
   ```sh
   bash build
   ```
   Update `XAI_API_KEY` in `.env`.

2. Start the interactive terminal browser (in a new terminal) 
    ```sh
    cd terminal
    npm run start
    ```

3. Navigate to http://<ip>:6060 and start bun dev server
   ```sh
   bash start.sh
   ```
