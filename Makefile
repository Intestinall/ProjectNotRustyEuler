run:
	cargo run --release

watch:
	cargo watch -c -x "run --release"

format:
	cargo fmt

check:
	cargo clippy
