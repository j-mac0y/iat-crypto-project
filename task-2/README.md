# IAT Project - Task 2
## Pre-requisites: *Rust*

## Description
This directory contains a client, server and MITM.

In addition to the last task, `client_message.rs` defines the struct `ClientMessage` which includes a function for hashing the message data, alongside some extra utilites like encoding/decoding. 

Client uses `ClientMessage` to hash the message data before sending it, MITM then replaces the message data with its own from `malicious_message.txt`, and then forwards to the server.

The server validates that the plaintext data has not been altered by using the hash, but does not know the active MITM is the one who created the data. 

## Instructions
To run the scenario, first run `cargo build`, then open three terminals:
1. Run `cargo run --bin server` in the first terminal.
2. Run `cargo run --bin mitm` in the second terminal.
3. Run `cargo run --bin client` in the third terminal.
4. Repeat step 3 if you want to see the result again.
