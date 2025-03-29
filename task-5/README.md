# IAT Project - Task 5

## Description
This directory contains a client, server and a Certificate Authority.

Compared to the last task, this task removes the MITM, but adds a Certificate Authority, which fixes the trust problem of the previous tasks by allowing the client to be confident of the public key of the server.

This task plays out in the following steps:
1. Server: Request certificate from CA
2. CA: Signs the certificate request and issues a `Certificate` to Server
3. Client: Gets certificate from server
4. Client: Gets CA public key from CA
5. Client: Validates the Certificate using the signature of the CA and the public key

## Instructions
### Executables
I have placed executables built for an x86 Windows machine in the `executables` folder for each task.

Execute the programs in the following order (in separate terminals):
1. Run `./ca` in the first terminal.
2. Run `./server` in the second terminal.
3. Run `./client` in the third terminal.

If you want to see it again, restart the process from step 1.

### Compiling from source code
Pre-requisites:
1. Rust
2. [Rust OpenSSL crate setup commands](https://docs.rs/openssl/latest/openssl/#automatic)

If the executable doesn't work for some reason, you can run `cargo build` and then run each program with `cargo run --bin programName`, using the same order as for the executables.
