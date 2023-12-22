.PHONY: build install uninstall

BINARY_NAME=hayabusa
INSTALL_PATH=/usr/local/bin
SERVICE_FILE=distribution/hayabusa.service
SYSTEMD_PATH=/etc/systemd/system
STAGING_DIR := staging

build:
	cargo build --release

clean_stage:
	@echo "Cleaning staging directory..."
	@rm -rf $(STAGING_DIR)/*

stage: clean_stage build
	@echo "Staging binary and service..."
	@mkdir -p $(STAGING_DIR)$(INSTALL_PATH)
	@mkdir -p $(STAGING_DIR)$(SYSTEMD_PATH)
	@install -m755 target/release/$(BINARY_NAME) $(STAGING_DIR)$(INSTALL_PATH)/$(BINARY_NAME)
	@install -m644 $(SERVICE_FILE) $(STAGING_DIR)$(SYSTEMD_PATH)/$(BINARY_NAME).service

install: stage
ifndef PACKAGE_BUILD
	@echo "Installing binary and service..."
	@cp -a $(STAGING_DIR)/. /
	@systemctl daemon-reload
	@systemctl enable $(BINARY_NAME).service
	@systemctl restart $(BINARY_NAME).service
	@echo "Installation complete."
endif

uninstall:
ifndef PACKAGE_BUILD
	@echo "Uninstalling binary and service..."
	-@systemctl stop $(BINARY_NAME).service
	-@systemctl disable $(BINARY_NAME).service
	-@rm -f $(INSTALL_PATH)/$(BINARY_NAME)
	-@rm -f $(SYSTEMD_PATH)/$(BINARY_NAME).service
	-@systemctl daemon-reload
	@echo "Uninstallation complete."
endif
