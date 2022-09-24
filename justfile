all: test lint build-binary

test:
	cargo test

lint:
	cargo clippy
	cargo clippy --bin hb --features "build-binary"

build-binary:
	cargo build --release --features 'build-binary fast' --bin hb
