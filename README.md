# Scroll Prover Integration with Gevulot

This project demonstrates the integration of the Scroll prover with the Gevulot platform. The integration was completed during ZK Hack Montréal.

## Overview

This repository contains the steps required to build, deploy, and test a Scroll prover on the Gevulot network. The prover can be used to generate zero-knowledge proofs on-chain.

## Quick Start

### Prover Deployment

- **Prover Hash**: `e6d7ba0b6d1ddc7a94b761a3bc8e42cfa2c0368fcd4cba710fde14f366ed447e`
- **Transaction Hash**: `09191bfaeda2a4e65cd4e0c5a5067c55b8a2f65e9631b142c168cf00065b10f3`

### Example Transaction

The Scroll prover was successfully deployed and executed on the Gevulot Devnet. Below is an example transaction hash:

- **Transaction Hash**: `ebaaf7687f3f9ee62bfa38b83b10637b37645e0ec394a84ef9d54aee38218d35`

## Deployment Steps

### Requirements

- A Linux machine with x86_64 architecture.
- `gevulot-cli`, `Ops`, and `Rust` installed.
- A valid Gevulot API key.

### Building the Prover

1. **Compile the Prover:**
    ```
    cargo build --release
    ```

### Preparing the Unikernel Image

1. **Build the Image:**
    ```bash
    ops build ./target/release/trace_prover -c manifest_trace_prover.json
    ```

    The image file location will be printed by Ops:
    ```
    Bootable image file: /home/username/.ops/images/trace_prover
    ```

### Calculating the Image Hash

1. **Compute the Hash:**
    ```bash
    gevulot-cli calculate-hash --file ~/.ops/images/trace_prover
    ```

    The resulting hash will be:
    ```
    The hash of the file is: 6575a685399c4bf0709c395b3631a5c121e0ff6b7c2d3f572ad80c9fc29a039e
    ```

### Uploading the Image

1. **Upload to HTTP Service:**
   Upload the image to a service like Google Cloud Storage to make it accessible via HTTP.

### Deploying on Gevulot

1. **Deploy the Prover:**
    ```bash
    gevulot-cli --jsonurl "http://api.devnet.gevulot.com:9944" --keyfile ./localkey.pki \
deploy \
--name "Simple Gevulot test prover & verifier A-Z" \
--prover 6575a685399c4bf0709c395b3631a5c121e0ff6b7c2d3f572ad80c9fc29a039e \
--provername '#testprover' \
--proverimgurl 'https://github.com/aysha8665/Test/releases/download/v1.0.0/trace_prover' \
--verifier c8d90de9418502c30b30a0284f804cc5fc95509482e5c4143f26b7539812c1c1 \
--verifiername '#testverifier' \
--verifierimgurl 'https://storage.googleapis.com/gevulot-devnet-deployments/test-1/verifier'
Start prover / verifier deployment
Prover / Verifier deployed correctly.
Prover hash:e6d7ba0b6d1ddc7a94b761a3bc8e42cfa2c0368fcd4cba710fde14f366ed447e
Verifier hash:09191bfaeda2a4e65cd4e0c5a5067c55b8a2f65e9631b142c168cf00065b10f3.
Tx Hash:ebaaf7687f3f9ee62bfa38b83b10637b37645e0ec394a84ef9d54aee38218d35
    ```

### Testing the Deployment

1. **Run Example Workload:**

   After successful deployment, initiate a sample run using the following commands:


### Executing on Gevulot

1. **Execute Proof Generation:**
    ```bash

    ```


