# Chromia Neural Interface

This repository demonstrates using Chromia to store AI Agent short-term and long-term memories on chain. It also demonstrates a simple neural interface for interacting with the AI Agent.

[Chromia Demo using oyster](https://www.youtube.com/watch?v=YfWzTeK1LaY)


## Prerequisites
- Install [Docker](https://docs.docker.com/engine/install/ubuntu/)
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

## Steps for using Marlin's TEE (only supports Linux systems)
1. Update the variables in the `.env` file.

2. Uncomment the `# Production` section and comment out the `# Local testing` section in the `docker-compose.yml` file.

3. [Check your system requirements](https://docs.marlin.org/oyster/build-cvm/tutorials/)

4. [Setup the development environment](https://docs.marlin.org/oyster/build-cvm/tutorials/setup)

5. Build an enclave image
   ```sh
   # for amd64
   oyster-cvm build --platform amd64 --docker-compose ./docker-compose.yml --commit-ref 5826f66fa1a2dc60d1180465f440c4564f2291fe

   # for arm64
   oyster-cvm build --platform arm64 --docker-compose ./docker-compose.yml --commit-ref 5826f66fa1a2dc60d1180465f440c4564f2291fe
   ```
   You should now have a result folder with the enclave image in image.eif and the PCRs in pcr.json. The PCRs represent a "fingerprint" of the enclave image and will help you verify what is running in a given enclave.

6. Obtain an [API key and secret from Pinata](https://docs.pinata.cloud/account-management/api-keys)

7. Upload your enclave image to Pinata
   ```sh
   # Note the image URL after it finishes
   PINATA_API_KEY=<API key> PINATA_API_SECRET=<API secret> oyster-cvm upload --file result/image.eif
   ```
   Make a note of the image URL from the output.

8. Set up a wallet where you can export the private key. Deposit 0.001 ETH and 1 USDC to the wallet on the Arbitrum One network.

9. Build the init-params-manager. This encoder is expected to be used by users to encode their initialization parameters before deployment.
   ```sh
   cd init-params-manager
   cargo build --release
   ```

10. Encode the `.env` file using the init-params-manager
   ```sh
   cd ..
   ./init-params-manager/target/release/init-params-encoder --kms-endpoint http://v1.kms.box:1101 --pcr0 <pcr0> --pcr1 <pcr1> --pcr2 <pcr2> --init-params 'bun/.env:1:1:file:./.env'
   ```
 Make a note of the base64 encoded data from the outpupt.

11. Deploy the enclave image 
   ```sh
   # for amd64
   # replace <key> with private key of the wallet
   # replace <url> with url from the upload step
   oyster-cvm deploy --wallet-private-key <key> --image-url <url> --instance-type c6a.2xlarge --region ap-south-1 --operator 0xe10Fa12f580e660Ecd593Ea4119ceBC90509D642 --duration-in-minutes 15 --init-params <base64 encoded init params>

   # for arm64
   # replace <key> with private key of the wallet
   # replace <url> with url from the upload step
   oyster-cvm deploy --wallet-private-key <key> --image-url <url> --instance-type c6g.2xlarge --region ap-south-1 --operator 0xe10Fa12f580e660Ecd593Ea4119ceBC90509D642 --duration-in-minutes 15 --init-params <base64 encoded init params>
   ```
   Make a note of the IP from the output and navigate to http://IP:6060 and start chatting!

12. Verify a remote attestation (recommended)
   ```sh
   # Replace <ip> with the IP you obtained above
   # Replace <pcrs> with values from pcr.json
   oyster-cvm verify --enclave-ip <ip> -0 <pcr0> -1 <pcr1> -2 <pcr2>
   ```
   You should see `Verification successful` along with some attestation fields printed out.

13. Head over to [Oyster Confidential VM tutorials](https://docs.marlin.org/oyster/build-cvm/tutorials/) for more details.
