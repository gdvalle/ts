all: build
build:
	cargo build --release --target=x86_64-unknown-linux-musl
bootstrap:
	rustup.sh --channel=nightly
	rustup.sh --add-target=x86_64-unknown-linux-musl
