# Chromia Neural Interface

This repository demonstrates using Chromia to store AI Agent short-term and long-term memories on chain. It also demonstrates a simple neural interface for interacting with the AI Agent.

[Chromia Demo using oyster](https://www.youtube.com/watch?v=YfWzTeK1LaY)


## Prerequisites
- Install [Docker](https://docs.docker.com/engine/install/ubuntu/)
- Fetch [Groq API key](https://console.groq.com/keys)
  
## For amd64 local run
1. Install dependencies and setup
   ```sh
   docker-compose up --build
   ```
   Update `XAI_API_KEY` in `.env`

2. Wait for the chromia node to start. You should be able to see logs:
   ![Logs](image.png)

3. Navigate to http://host-ip-or-localhost:6060 and start chatting!

## For arm64 local run
1. Replace docker images with corresponding arm64 versions in the `docker-compose.yml` file

2. Install dependencies and setup
   ```sh
   docker-compose up --build
   ```
   Update `XAI_API_KEY` in `.env`

3. Wait for the chromia node to start. You should be able to see logs:
   ![Logs](image.png)

4. Navigate to http://host-ip-or-localhost:6060 and start chatting!
