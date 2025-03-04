# IAT Project - Step 1

## Description
This directory contains a simple client, server and MITM:
- Server listens for connections and prints any messages it receives to the console.
- Client binds to the "server" port (which is actually the MITM port) and sends the message it reads from the file `message.txt`.
- MITM listens for connections and forwards any data it receives to the server, as if it is the client.