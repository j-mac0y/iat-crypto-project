# IAT Project - Task 3
## Pre-requisites: *Rust*

## Description
This directory contains a client, server and MITM.

In addition the last task, `client_message.rs` adds signing capability to the `ClientMessage` struct using the `openssl` library. 

Client signs the data using a keypair generated using RSA, but MITM can still replace the message data with its own from `malicious_message.txt` by simply overwriting the keys and generating its own signature.

The server validates that the plaintext data has not been altered and authenticates that it has been signed by the public key provided with the message.

However the server still has no way of knowing the active MITM is the one who created the data and keys.

## Instructions
To run the scenario, first run `cargo build`, then open three terminals:
1. Run `cargo run --bin server` in the first terminal.
2. Run `cargo run --bin mitm` in the second terminal.
3. Run `cargo run --bin client` in the third terminal.
4. Repeat step 3 if you want to see the result again.
