.PHONY: build scout_prerequisites clean_stage stage install uninstall

BINARY_NAME=hayabusa
LOCAL_BINARY=target/release/$(BINARY_NAME)
INSTALL_DIR=/usr/local/bin
SERVICE_FILE=distribution/hayabusa.service
SYSTEMD_PATH=/etc/systemd/system
STAGING_DIR := staging
BINARY_STAGE_PATH=$(STAGING_DIR)$(INSTALL_DIR)/$(BINARY_NAME)
SERVICE_STAGE_PATH=$(STAGING_DIR)$(SYSTEMD_PATH)/$(BINARY_NAME).service

build:
	cargo build --release

scout_prerequisites:
	@echo "Doing some pre-stage checks..."

	@echo "Looking for binary..."
ifeq ("$(wildcard ./$(LOCAL_BINARY))","")
	@echo "There is nothing to stage! try running 'make' before 'make install'."
	@exit 1
endif
	@echo "Found binary! ✓"

	@echo "Looking for service file..."
ifeq ("$(wildcard $(SERVICE_FILE))","")
	@echo "The service file is missing! This shouldn't happen, ensure the full repo is intact."
	@exit 1
endif
	@echo "Found service file! ✓"

	@echo "All checks passed!"

clean_stage:
	@echo "Clearing out old staged files..."
	rm -rf $(STAGING_DIR)/*

stage: scout_prerequisites clean_stage
	@echo "Staging current files..."
	mkdir -p $(STAGING_DIR)$(INSTALL_DIR)
	mkdir -p $(STAGING_DIR)$(SYSTEMD_PATH)
	cp -p $(LOCAL_BINARY) $(BINARY_STAGE_PATH)
	cp -p $(SERVICE_FILE) $(SERVICE_STAGE_PATH)
	chmod -R --reference=$(LOCAL_BINARY) $(STAGING_DIR)
	chown -R --reference=$(LOCAL_BINARY) $(STAGING_DIR)
	chmod 755 $(BINARY_STAGE_PATH)
	chmod 644 $(SERVICE_STAGE_PATH)

check_perms:
	@echo "Checking for permissions before making any changes..."

	@echo "Checking ability to install binary..."
ifneq ("$(shell [ -w $(INSTALL_DIR) ] && echo true)","true")
	@echo "write permissions missing, try running with sudo."
	@exit 1
endif
	@echo "Can install binary!"

	@echo "Checking ability to install service..."
ifneq ("$(shell [ -w $(SYSTEMD_PATH) ] && echo true)","true")
	@echo "write permissions missing, try running with sudo."
	@exit 1
endif
	@echo "Can install service!"

	@echo "Checking ability to start services..."
#	systemd-journald is already running, so "starting" it has no effect,
#	leaving us with the opportunity to check permissions before doing anything.
ifneq ("$(shell systemctl --no-ask-password start systemd-journald && echo true)","true")
	@echo "Unable to start services, permission to do so is required, try running with sudo."
	@exit 1
endif
	@echo "Can start services!"

	@echo "Checking ability to enable services..."
#	systemd-journald is static, it cannot be enabled, but still returns a zero exit,
#	leaving us with the opportunity to check permissions before doing anything.
ifneq ("$(shell systemctl --no-ask-password enable systemd-journald && echo true)","true")
	@echo "Unable to enable services, permission to do so is required, try running with sudo."
	@exit 1
endif
	@echo "Can enable services!"

	@echo "All checks passed!"

install: stage check_perms
ifndef PACKAGE_BUILD
	@echo "Installing binary and service..."
	cp -a $(STAGING_DIR)/* /
	systemctl daemon-reload
	systemctl --now enable $(BINARY_NAME).service
	systemctl restart $(BINARY_NAME).service # In case the previous version is still running.
	@echo "Installation complete."
endif

uninstall:
ifndef PACKAGE_BUILD
	@echo "Uninstalling binary and service..."
	-systemctl --now disable $(BINARY_NAME).service
	-rm -f $(INSTALL_DIR)/$(BINARY_NAME)
	-rm -f $(SYSTEMD_PATH)/$(BINARY_NAME).service
	-systemctl daemon-reload
	@echo "Uninstallation complete."
endif
