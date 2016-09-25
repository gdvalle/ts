all: build

PREFIX?=/usr/local

build:
	cargo build --release --target=x86_64-unknown-linux-musl

bootstrap:
	rustup.sh --channel=nightly
	rustup.sh --add-target=x86_64-unknown-linux-musl

install:
	install -D -m 0755 -t $(DESTDIR)$(PREFIX)/bin target/x86_64-unknown-linux-musl/release/ts
