.PHONY: build install

BINARY_NAME=hayabusa
INSTALL_PATH=/usr/local/bin
SERVICE_FILE=distribution/hayabusa.service
SYSTEMD_PATH=/etc/systemd/system

build:
	cargo build --release

install:
	@echo "Installing binary and service..."
	@rm -f $(INSTALL_PATH)/$(BINARY_NAME)
	@cp target/release/$(BINARY_NAME) $(INSTALL_PATH)/$(BINARY_NAME)
	@rm -f $(SYSTEMD_PATH)/$(BINARY_NAME).service
	@cp $(SERVICE_FILE) $(SYSTEMD_PATH)/$(BINARY_NAME).service
	@systemctl enable $(BINARY_NAME).service
	@systemctl restart $(BINARY_NAME).service

uninstall:
	@echo "Uninstalling binary and service..."
	@rm -f $(INSTALL_PATH)/$(BINARY_NAME)
	@systemctl disable $(BINARY_NAME).service
	@systemctl stop $(BINARY_NAME).service
	@rm -f $(SYSTEMD_PATH)/$(BINARY_NAME).service
	@systemctl daemon-reload