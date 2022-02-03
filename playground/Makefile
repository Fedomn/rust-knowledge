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
	cargo deny check

clean:
	cargo clean

fix:
	cargo fix --allow-dirty --allow-staged
	cargo fmt --all
	cargo fmt -- --check
	cargo clippy --all-targets --all-features --tests --benches -- -D warnings
	cargo doc --all-features --no-deps

release:
	cargo build --release