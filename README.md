# Chromia Neural Interface

This repository demonstrates using Chromia to store AI Agent short-term and long-term memories on chain. It also demonstrates a simple neural interface for interacting with the AI Agent.

[Chromia Demo using oyster](https://www.youtube.com/watch?v=YfWzTeK1LaY)


## Run using docker compose

### Prerequisites
- Install [Docker](https://docs.docker.com/engine/install/ubuntu/)
- Fetch [Groq API key](https://console.groq.com/keys)

### Steps
1. Install dependencies and setup
   ```sh
   docker-compose up --build
   ```
   Update `XAI_API_KEY` in `.env`

2. Wait for the chromia node to start. You should be able to see logs:
   ![Logs](image.png)

3. Navigate to http://host-ip-or-localhost:6060 and start chatting!
