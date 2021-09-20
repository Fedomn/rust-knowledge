default: run

doc:
	open https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html

run:
	cargo run the poem.txt

test:
	cargo t

fix:
	cargo fix --allow-dirty --allow-staged
	cargo fmt
	cargo clippy
