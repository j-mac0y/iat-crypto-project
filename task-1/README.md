# IAT Project - Task 1
## Description
This directory contains a simple client, server and MITM:
- Server listens for connections and prints any messages it receives to the console.
- Client binds to the "server" port (which is actually the MITM port) and sends the message it reads from the file `message.txt`.
- MITM listens for connections and forwards any data it receives to the server, as if it is the client.

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
