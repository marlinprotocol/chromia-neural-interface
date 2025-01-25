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

2. Navigate to http://host-ip-or-localhost:6060 and start chatting!

curl --location --request POST 'http://localhost:3030/api/set_key' \
--header 'Content-Type: application/json' \
--data-raw '
{ 
"xai_api_key": [80, 120, 234, 98, 148, 204, 198, 109, 92, 52, 103, 196, 117, 10, 247, 129, 235, 211, 218, 14, 197, 4, 191, 39, 108, 103, 158, 188, 101, 243, 185, 231, 248, 150, 9, 134, 202, 157, 74, 127, 119, 113, 45, 0, 225, 46, 247, 12, 130, 55, 222, 216, 17, 184, 41, 137, 38, 148, 76, 239, 224, 126, 137, 219, 65, 160, 136, 128, 10, 17, 7, 249, 82, 227, 105, 57, 106, 164, 164, 95, 236, 176, 64, 255, 67, 159, 173, 115, 172, 235, 191, 34, 63, 225, 213, 140, 254, 215, 219, 109, 217, 122, 219, 139, 208, 44, 209, 233, 216, 106, 202, 246], 
"encryption_key": [66, 108, 97, 104, 66, 108, 97, 104, 49, 50, 51] 
 }'
