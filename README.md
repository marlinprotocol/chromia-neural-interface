# Chromia Neural Interface

This repository showcases how Chromia can store both short-term and long-term AI Agent memories on-chain. Additionally, it features a streamlined neural interface for seamless interaction with the AI Agent. The user interface is deployed on a decentralized cloud, secured within Trusted Execution Environments for enhanced privacy and security.

[![Chromia AI agent on Oyster](https://res.cloudinary.com/marcomontalbano/image/upload/v1742238254/video_to_markdown/images/google-drive--1gPEI9PtRptOj7zSK4d__MnPtUqpCpGXZ-c05b58ac6eb4c4700831b2b3070cd403.jpg)](https://drive.google.com/file/d/1gPEI9PtRptOj7zSK4d__MnPtUqpCpGXZ/view?usp=sharing "Chromia AI agent on Oyster")

## Prerequisites
- For docker desktop, enable [host networking](https://docs.docker.com/engine/network/drivers/host/#docker-desktop)
- Fetch [Groq API key](https://console.groq.com/keys)
- [Generate key pair and lease a container](https://docs.chromia.com/intro/getting-started/testnet/getting-started#step-1-obtain-a-container-for-your-dapp)
- [Deploy the dapp & obtain the dapp brid](https://docs.chromia.com/intro/getting-started/testnet/getting-started#step-2-deploy-your-dapp)

**Note:** Update the following docker images according to your system's architecture in the `docker-compose.yml`:
  ```sh
   # Bun service
    bun:
      image: kalpita888/bun:0.0.1                     # For arm64 system use kalpita888/bun_arm64:0.0.1 and for amd64 system use kalpita888/bun:0.0.1
  ```
  
## Steps for local run
1. Update the variables in the `.env` file.

2. Install dependencies and setup
   ```sh
   docker-compose up --build
   ```

3. Navigate to http://localhost:6060 and start chatting!

## Steps for using Marlin's TEE
1. Update the variables in the `.env` file.

2. Uncomment the `# Production` section and comment out the `# Local testing` section in the `docker-compose.yml` file.

3. [Check your system requirements](https://docs.marlin.org/oyster/build-cvm/tutorials/)

4. Install oyster-cvm and [Setup the development environment](https://docs.marlin.org/oyster/build-cvm/tutorials/setup#run-the-doctor-command)
   ```sh 
   # for amd64
   sudo wget https://artifacts.marlin.org/oyster/binaries/oyster-cvm_v2.0.0_linux_amd64 -O /usr/local/bin/oyster-cvm

   # for arm64
   sudo wget https://artifacts.marlin.org/oyster/binaries/oyster-cvm_v2.0.0_linux_arm64 -O /usr/local/bin/oyster-cvm

   # for darwin
   sudo wget https://artifacts.marlin.org/oyster/binaries/oyster-cvm_v2.0.1_darwin_arm64 -O /usr/local/bin/oyster-cvm
   ```
   Make it executable by running the following:
   ```sh
   sudo chmod +x /usr/local/bin/oyster-cvm
   oyster-cvm
   ```

5. Set up a wallet where you can export the private key. Deposit 0.001 ETH and 1 USDC to the wallet on the Arbitrum One network.

6. Deploy the enclave image 
   ```sh
   # for amd64
   # replace <key> with private key of the wallet
   oyster-cvm deploy --wallet-private-key <key> --docker-compose ./docker-compose.yml --instance-type c6a.2xlarge --region ap-south-1 --operator 0xe10Fa12f580e660Ecd593Ea4119ceBC90509D642 --duration-in-minutes 20 --pcr-preset base/blue/v1.0.0/amd64 --init-params 'bun/.env:1:1:file:./.env' --image-url https://artifacts.marlin.org/oyster/eifs/base-blue_v1.0.0_linux_amd64.eif

   # for arm64
   # replace <key> with private key of the wallet
   oyster-cvm deploy --wallet-private-key <key> --docker-compose ./docker-compose.yml --instance-type c6g.2xlarge --region ap-south-1 --operator 0xe10Fa12f580e660Ecd593Ea4119ceBC90509D642 --duration-in-minutes 20 --pcr-preset base/blue/v1.0.0/arm64 --init-params 'bun/.env:1:1:file:./.env' --image-url https://artifacts.marlin.org/oyster/eifs/base-blue_v1.0.0_linux_arm64.eif
   ```
   Make a note of the IP from the output and navigate to http://IP:6060 and start chatting!

7. Verify a remote attestation (recommended)
   ```sh
   # Replace <ip> with the IP you obtained above
   oyster-cvm verify --enclave-ip <ip> 
   ```
   You should see `Verification successful` along with some attestation fields printed out.

Head over to [Oyster Confidential VM tutorials](https://docs.marlin.org/oyster/build-cvm/tutorials/) for more details.
