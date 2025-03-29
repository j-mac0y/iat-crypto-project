# IAT Project - Task 4
## Description
This directory contains a client, server and MITM.

In addition the last task, `client_message.rs` adds encryption/decryption capability to the `ClientMessage` struct using the `openssl` library. 

Client encrypts the data using a keypair generated using RSA, but MITM can still replace the message data with its own from `malicious_message.txt` by simply overwriting the keys and encrypting the data itself.

The server decrypts the data, in theory validating the authenticity and integrity of the data, but it still has has no way of knowing the active MITM is the one who created the data and keys.

The main benefit of upgrading to encryption from a signature last time is that the client preserves confidentiality of its initial message (MITM can't read it).

NB to marker on lack of decryption at the MITM: Task 4 Step 2 asked for MITM to decrypt the data, but this did not make sense since the MITM would need access to the private key of the server to decrypt the data. I have assumed that this was a typo in the assingment and so not implemented decryption at the MITM.
> "Your solution should include the middleman decrypting the client's communications before modifying the data to ensure that the server still receives the modified data in an encrypted format."  

## Instructions
### Executables
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