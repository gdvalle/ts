PREFIX?=/usr/local
TARGET?=x86_64-unknown-linux-musl
TS_BIN_PATH=target/$(TARGET)/release/ts
RUST_DISTRIBUTION?=nightly

.PHONY: build compress bootstrap install clean

all: build compress

build:
	cargo build --release --target=$(TARGET)

compress:
	strip -s $(TS_BIN_PATH)
	upx --brute $(TS_BIN_PATH)

bootstrap:
	curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=$(RUST_DISTRIBUTION)
	$(HOME)/.cargo/bin/rustup target add $(TARGET)

install:
	install -D -m 0755 -t $(DESTDIR)$(PREFIX)/bin $(TS_BIN_PATH)

clean:
	rm -r target
