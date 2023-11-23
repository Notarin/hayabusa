.PHONY: build install

BINARY_NAME=hayabusa
INSTALL_PATH=/usr/local/bin
SERVICE_FILE=src/daemon/hayabusa.service
SYSTEMD_PATH=/etc/systemd/system

build:
	cargo build --release

install:
	@echo "Installing binary and service..."
	@cp target/release/$(BINARY_NAME) $(INSTALL_PATH)/$(BINARY_NAME)
	@cp $(SERVICE_FILE) $(SYSTEMD_PATH)/$(BINARY_NAME).service
	@systemctl enable $(BINARY_NAME).service
	@systemctl start $(BINARY_NAME).service
