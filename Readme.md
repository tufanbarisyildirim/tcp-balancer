# TCP Balancer

It listens on a specified port and forwards incoming connections to one of the specified destination IP:port pairs in a
round-robin fashion. It also has a feature to periodically check the destination IP:port pairs and mark them as "live"
or "dead" based on their reachability, so that the load balancer will only forward connections to live destinations.

To use the proxy, you need to have rust installed on your machine.

1. Clone the repository
2. Run `cargo build` to build the project
3. Run `cargo run -- --destinations=<destinations> --listen-port=<listen_port>`

For example, if you want to listen on port `8000` and forward data to destination IPs `127.0.0.1:8001`
and `127.0.0.1:8002`, you would run the following command:

```bash
cargo run --  --destinations=127.0.0.1:8001,127.0.0.1:8002 --listen-port=8000
```