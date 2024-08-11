# Scroll Prover Integration with Gevulot

This project demonstrates the integration of the Scroll prover with the Gevulot platform, leveraging the Risc0 zkVM. The integration was completed during .

## Overview

This repository contains the steps required to build, deploy, and test a Scroll prover on the Gevulot network. The prover can be used to generate zero-knowledge proofs on-chain, showcasing the power and flexibility of integrating ZK-rollups with the Gevulot project.

## Quick Start

### Prover Deployment

- **Prover Hash**: `34948922b107eaa8672b3315c830638fc98ade760780dd2f5d9406a14ef6c10a`
- **Transaction Hash**: `50a1f17c966020b4f85cab002515f367473127372f8e777be4c4e09487d09cc6`

### Example Transaction

The Scroll prover was successfully deployed and executed on the Gevulot Devnet. Below is an example transaction hash:

- **Transaction Hash**: `3c90fd79ea28803f2d826d23b7c332c1c7be50c95e73fa85472a706ac979784a`

## Deployment Steps

### Requirements

- A Linux machine with x86_64 architecture.
- `gevulot-cli`, `Ops`, and `Rust` installed.
- A valid Gevulot API key.

### Building the Prover

1. **Compile the Prover:**
    ```bash
    cargo build --release -p prover
    cp ./target/release/prover_gevulot ./prover_gevulot
    ```

### Preparing the Unikernel Image

1. **Build the Image:**
    ```bash
    ops build ./prover_gevulot -c manifest_prover.json
    ```

    The image file location will be printed by Ops:
    ```
    Bootable image file: /home/username/.ops/images/prover_gevulot
    ```

### Calculating the Image Hash

1. **Compute the Hash:**
    ```bash
    gevulot-cli calculate-hash --file ~/.ops/images/prover_gevulot
    ```

    The resulting hash will be:
    ```
    The hash of the file is: f37b73d5b9c1108a4ee5276a880cdba0cd97c71997cd37884c4c7ab87f340517
    ```

### Uploading the Image

1. **Upload to HTTP Service:**
   Upload the image to a service like Google Cloud Storage to make it accessible via HTTP.

### Deploying on Gevulot

1. **Deploy the Prover:**
    ```bash
    gevulot-cli --jsonurl "http://api.devnet.gevulot.com:9944" --keyfile /tmp/localkey.pki \
    deploy \
    --name "Scroll Prover Integration" \
    --prover f37b73d5b9c1108a4ee5276a880cdba0cd97c71997cd37884c4c7ab87f340517 \
    --provername 'scrollprover' \
    --proverimgurl 'https://storage.googleapis.com/gevulot-test/scroll/prover_gevulot'
    ```

### Testing the Deployment

1. **Run Example Workload:**

   After successful deployment, you can initiate a sample run using the following commands:

   - **Setup Guest:**
     ```bash
     cargo build --release -p example-workload-input
     cp ./target/riscv-guest/riscv32im-risc0-zkvm-elf/release/square_check_guest /tmp/workload-guest.bin
     ```

   - **Prepare Input:**
     ```bash
     cargo run --release -p example-workload-input > /tmp/workload-input.json
     ```

   - **Run the Prover:**
     ```bash
     cargo run -p prover --bin prover -- --guest /tmp/workload-guest.bin --input /tmp/workload-input.json --output /tmp/workload-receipt.bin
     ```

   - **Verify Locally:**
     ```bash
     cargo run -p verifier --bin verifier -- --guest /tmp/workload-guest.bin --receipt /tmp/workload-receipt.bin
     ```

   This should output a valid proof.

### Executing on Gevulot

1. **Execute Proof Generation:**
    ```bash
    gevulot-cli --jsonurl "http://api.devnet.gevulot.com:9944" --keyfile /tmp/localkey.pki \
    exec --tasks '[{"program":"34948922b107eaa8672b3315c830638fc98ade760780dd2f5d9406a14ef6c10a","cmd_args":[{"name":"--guest","value":"/workspace/workload-guest.bin"},{"name":"--input","value":"/workspace/workload-input.json"},{"name":"--output","value":"/workspace/workload-receipt.bin"}],"inputs":[{"Input":{"local_path":"1e7d80754b7f9f8cf0bc5b423feb03baacd4e2a533333581f0ab713a75e52afb","vm_path":"/workspace/workload-guest.bin","file_url":"https://storage.googleapis.com/gevulot-test/workload-guest.bin"}},{"Input":{"local_path":"e51bf918d5d85b49283a096ccb25afb0d2089fec2701b5d9f79437b58cd39660","vm_path":"/workspace/workload-input.json","file_url":"https://storage.googleapis.com/gevulot-test/workload-input.json"}}]}]'
    ```

## Conclusion

This integration demonstrates the capability of the Scroll prover running on the Gevulot platform, paving the way for advanced zero-knowledge proof systems to operate seamlessly on-chain.
