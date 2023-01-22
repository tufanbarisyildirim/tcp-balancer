# Dummy TCP MITM

This is a simple dummy TCP man-in-the-middle (MITM) proxy that listens on a specific port and forwards data between a
client and a destination server. It is intended for debugging purposes only, and should not be used in production
environments.

To use the proxy, you need to have rust installed on your machine.

1. Clone the repository
2. Run `cargo build` to build the project
3. Run `cargo run -- --destinations=<destinations> --listen-port=<listen_port>`

For example, if you want to listen on port `8000` and forward data to destination IPs `127.0.0.1:8001` and `127.0.0.1:8002`, you would run the following command:

```bash
cargo run --  --destinations=127.0.0.1:8001,127.0.0.1:8002 --listen-port=8000
```