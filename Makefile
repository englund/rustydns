include .env

BINARY = ydns-updater

PWD = $(shell pwd)
BIN_DIR= $(PWD)/target

TARGET = aarch64-unknown-linux-musl
TARGET_DIR = $(BIN_DIR)/$(TARGET)/release

build:
	cargo build --release

build-arm64:
	cargo build --release --target $(TARGET)

clean:
	cargo clean

run: build
	cargo run

deploy: build-arm64
	scp $(TARGET_DIR)/$(BINARY) $(SSH_USER)@$(SSH_HOST):$(SSH_PATH)/$(BINARY)

.PHONY: build build-arm64 clean run deploy