# IAT Project - Step 1

## Description
This directory contains a simple client, server and MITM:
- Server listens for connections and prints any messages it receives to the console.
- Client binds to the "server" port (which is actually the MITM port) and sends the message it reads from the file `message.txt`.
- MITM listens for connections and forwards any data it receives to the server, as if it is the client.

## Instructions
To run the scenario, open three terminals:
1. Run `cargo run --bin server` in the first terminal.
2. Run `cargo run --bin mitm` in the second terminal.
3. Run `cargo run --bin client` in the third terminal.
4. Repeat step 3 if you want to see the result again.
