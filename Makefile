all: build

PREFIX?=/usr/local
TARGET=x86_64-unknown-linux-musl
TS_BIN_PATH=target/$(TARGET)/release/ts

build:
	cargo build --release --target=$(TARGET)
	strip $(TS_BIN_PATH)
	upx --brute $(TS_BIN_PATH)

bootstrap:
	rustup.sh --channel=nightly
	rustup.sh --add-target=$(TARGET)

install:
	install -D -m 0755 -t $(DESTDIR)$(PREFIX)/bin $(TS_BIN_PATH)

clean:
	rm -r target
