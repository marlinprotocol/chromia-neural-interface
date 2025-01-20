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

oyster-cvm build --platform amd64 --docker-compose ./docker-compose.yml --docker-images ./chromia-node.tar

pushed: https://gateway.pinata.cloud/ipfs/QmXjEoktH1mHPKEGwH3jRPpPjjgzpPKcM2Ek2ifuXinCvB

image urls: https://gateway.pinata.cloud/ipfs/QmXjEoktH1mHPKEGwH3jRPpPjjgzpPKcM2Ek2ifuXinCvB

http worked: https://gateway.pinata.cloud/ipfs/Qmax5c5ACCByqa7V8KZJoFuvo3MTFSPa5r8hwmUdjVfjiX

non-tar-image: oyster-cvm logs --ip 3.111.212.194

tar-image: oyster-cvm logs --ip 13.232.151.136, http://15.207.33.60:6060/


PINATA_API_KEY=b8043ac3ae109dfa16e2 PINATA_API_SECRET=ffecbddea420de9d70e5414c2dcd738ab9ebcef8bf10e6982ae36e497e1d7d01 oyster-cvm upload --file result/image.eif

oyster-cvm deploy --wallet-private-key 6e5aebb822bc44949dc600bf6c32c5a712fda977e3619febc294d1c2ad61a514 --image-url https://gateway.pinata.cloud/ipfs/QmTJQTCNcNvC2HEQ5YSs15MFUTwTzGFFFGDUFWnAqTwLcq --instance-type c6a.4xlarge --region ap-south-1 --operator 0xe10Fa12f580e660Ecd593Ea4119ceBC90509D642 --duration-in-minutes 15 --debug
