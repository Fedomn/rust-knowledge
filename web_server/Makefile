doc:
	open https://doc.rust-lang.org/stable/book/ch20-01-single-threaded.html

run:
	cargo run

test:
	curl http://127.0.0.1:7878/
	curl http://127.0.0.1:7878/

fix:
	cargo fix --allow-dirty --allow-staged
	cargo fmt
	cargo clippy