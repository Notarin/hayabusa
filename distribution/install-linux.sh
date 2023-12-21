#!/usr/bin/env sh
# Stop on error
set -e
: "${UNINSTALL=false}"
: "${ROOTLESS=false}"
for arg in "$@"
do
  if [ "$arg" = "uninstall" ]; then
    UNINSTALL=true
  fi
  if [ "$arg" = "--rootless" ]; then
    ROOTLESS=true
  fi
done

# check if we're sudo, if not exit, however if we're rootless, we don't need sudo
is_rootless="$ROOTLESS"
is_not_root=$(( EUID != 0 ))

if [ "$is_rootless" = false ] && [ $is_not_root -eq 1 ]; then
    echo "Please run as root or with sudo, or if you're absolutely sure you know what you're doing, run with --rootless"
    exit
fi


: "${RELEASE_URL=https://api.github.com/repos/notarin/hayabusa/releases/latest}"
: "${INSTALL_PATH=/usr/local/bin}"
: "${SYSTEMD_PATH=/etc/systemd/system}"

GIT_RELEASE="$(curl -s $RELEASE_URL)"
URL_LIST="$(echo "$GIT_RELEASE" | grep browser_download_url)"
URL_LIST="$(echo "$URL_LIST" | cut -d\" -f4)"
LINUX_BINARY_URL="$(echo "$URL_LIST" | grep hayabusa-linux)"
SYSTEMD_SERVICE_URL="$(echo "$URL_LIST" | grep hayabusa.service)"

install() {
  echo "Downloading binary from $LINUX_BINARY_URL to $INSTALL_PATH/hayabusa"
  curl -sL "$LINUX_BINARY_URL" -o $INSTALL_PATH/hayabusa
  echo "Setting permissions for $INSTALL_PATH/hayabusa"
  chmod +x $INSTALL_PATH/hayabusa
  echo "Downloading systemd service from $SYSTEMD_SERVICE_URL to $SYSTEMD_PATH/hayabusa.service"
  curl -sL "$SYSTEMD_SERVICE_URL" -o $SYSTEMD_PATH/hayabusa.service
  echo "Enabling systemd service"
  systemctl enable hayabusa.service
  echo "(Re)Starting systemd service"
  systemctl restart hayabusa.service
  echo "Installation complete!"
}

uninstall() {
  echo "Stopping systemd service"
  systemctl stop hayabusa.service
  echo "Disabling systemd service"
  systemctl disable hayabusa.service
  echo "Removing systemd service"
  rm -f $SYSTEMD_PATH/hayabusa.service
  echo "Removing binary"
  rm -f $INSTALL_PATH/hayabusa
  echo "Uninstallation complete!"
}

if [ "$UNINSTALL" = true ]; then
  uninstall
else
  install
fi
