# IAT Project - Task 2
## Description
This directory contains a client, server and MITM.

In addition to the last task, `client_message.rs` defines the struct `ClientMessage` which includes a function for hashing the message data, alongside some extra utilites like encoding/decoding. 

Client uses `ClientMessage` to hash the message data before sending it, MITM then replaces the message data with its own from `malicious_message.txt`, and then forwards to the server.

The server validates that the plaintext data has not been altered by using the hash, but does not know the active MITM is the one who created the data. 

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