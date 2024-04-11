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

cmd-copy: build-arm64
	scp $(TARGET_DIR)/$(BINARY) $(SSH_USER)@$(SSH_HOST):$(BIN_PATH)/$(BINARY)

config-copy:
	scp $(PWD)/$(CONFIG_FILE) $(SSH_USER)@$(SSH_HOST):$(CONFIG_PATH)/$(CONFIG_FILE)

service-copy:
	scp $(PWD)/$(SERVICE_FILE) $(SSH_USER)@$(SSH_HOST):$(SERVICE_PATH)/$(BINARY)

service-deploy: service-stop service-disable service-copy service-enable service-start

service-enable:
	ssh $(SSH_USER)@$(SSH_HOST) "/etc/init.d/$(BINARY) enable"

service-disable:
	ssh $(SSH_USER)@$(SSH_HOST) "/etc/init.d/$(BINARY) enable"

service-stop:
	ssh $(SSH_USER)@$(SSH_HOST) "/etc/init.d/$(BINARY) stop"

service-start:
	ssh $(SSH_USER)@$(SSH_HOST) "/etc/init.d/$(BINARY) start"

deploy: service-stop cmd-copy service-deploy

.PHONY: build build-arm64 clean run deploy service-deploy service-enable service-disable service-stop service-start