# IAT Project - Task 3
## Description
This directory contains a client, server and MITM.

In addition the last task, `client_message.rs` adds signing capability to the `ClientMessage` struct using the `openssl` library. 

Client signs the data using a keypair generated using RSA, but MITM can still replace the message data with its own from `malicious_message.txt` by simply overwriting the keys and generating its own signature.

The server validates that the plaintext data has not been altered and authenticates that it has been signed by the public key provided with the message.

However the server still has no way of knowing the active MITM is the one who created the data and keys.

## Instructions
I have placed executables built for an x86 Windows machine in the `executables` folder for each task.

Execute the programs in the following order (in separate terminals):
1. Run `./server` in the first terminal.
2. Run `./mitm` in the second terminal.
3. Run `./client` in the third terminal.

If you want to see it again, restart the process from step 1.

### Compiling from source code
Pre-requisites:
1. Rust
2. [Rust OpenSSL crate setup commands](https://docs.rs/openssl/latest/openssl/#automatic)

If the executable doesn't work for some reason, you can run `cargo build` and then run each program with `cargo run --bin programName`, using the same order as for the executables.