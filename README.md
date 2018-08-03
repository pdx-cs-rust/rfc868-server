# rfc868-server
Copyright (c) 2018 Bart Massey

This software implements the server side of RFC 868, the
old-school protocol for getting a remote system's time.
This server responds to both TCP and UDP requests.

The server as it stands is solely a demo of Rust
programming. Each service will stop on error, including
malformed requests. The server serves localhost
(`127.0.0.1`) only.

To build and run this server, say "cargo run" as root.

# License

This work is licensed under the "MIT License". Please see
the file `LICENSE` in this distribution for license terms.
