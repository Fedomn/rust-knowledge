.PHONY: test run build check clean fix release

default: test

test: fix
	cargo test -- --nocapture

run:
	cargo run

build:
	cargo build

check:
	cargo check

clean:
	cargo clean

fix:
	cargo fix --allow-dirty --allow-staged
	cargo fmt
	cargo clippy

release:
	cargo build --release