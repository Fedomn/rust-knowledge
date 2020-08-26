default: run

run:
	cargo run

build:
	cargo build

check:
	cargo check

clean:
	cargo clean

fix:
	@cargo fix --allow-dirty --allow-staged
	@cargo fmt

release:
	cargo build --release