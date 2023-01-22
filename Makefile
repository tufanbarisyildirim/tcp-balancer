run:
	cargo run -- --destinations=127.0.0.1:8001,127.0.0.1:8002 --listen-port=8000

build:
	cargo build

release:
	cargo build --release

test:
	cargo test

.PHONY: run build release test
