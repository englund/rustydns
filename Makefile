include .env

BINARY = ydns
SERVICE_FILE = ydns.service
CONFIG_FILE = ydns.yaml

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

deploy-cmd: build-arm64
	scp $(TARGET_DIR)/ydns $(SSH_USER)@$(SSH_HOST):$(BIN_PATH)/$(BINARY)

deploy-config:
	scp $(PWD)/$(CONFIG_FILE) $(SSH_USER)@$(SSH_HOST):$(CONFIG_PATH)/$(CONFIG_FILE)

deploy-service:
	scp $(PWD)/$(SERVICE_FILE) $(SSH_USER)@$(SSH_HOST):$(SERVICE_PATH)/$(BINARY)

.PHONY: build build-arm64 clean run deploy deploy-service