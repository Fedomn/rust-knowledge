.PHONY: test run build check clean fix release

default: test

test: fix
	#cargo test -- --nocapture
	cargo test --all-features

run:
	cargo run

build:
	cargo build

check:
	cargo check --all

clean:
	cargo clean

fix:
	cargo fix --allow-dirty --allow-staged
	cargo fmt -- --check
	#cargo clippy
	cargo clippy --all-targets --all-features --tests --benches -- -D warnings

release:
	cargo build --release