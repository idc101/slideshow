#!/bin/bash

set -e

SERVICE_NAME="slideshow"
BINARY_SOURCE="./slideshow"
BINARY_DEST="/usr/local/bin/slideshow"
SERVICE_FILE="/etc/systemd/system/${SERVICE_NAME}.service"

if [[ $EUID -ne 0 ]]; then
   echo "Please run as root (use sudo)"
   exit 1
fi

if [[ ! -f "$BINARY_SOURCE" ]]; then
    echo "Error: $BINARY_SOURCE not found"
    exit 1
fi

echo "Installing binary..."
install -m 755 "$BINARY_SOURCE" "$BINARY_DEST"

echo "Creating systemd service..."
cat > "$SERVICE_FILE" <<EOF
[Unit]
Description=Slideshow Service
After=network.target

[Service]
Type=simple
ExecStart=$BINARY_DEST
Restart=always
RestartSec=5
User=root
WorkingDirectory=/usr/local/bin
Environment="PICTURES_BASE=/media/iain/Data/slideshow-all"

[Install]
WantedBy=multi-user.target
EOF

echo "Reloading systemd..."
systemctl daemon-reload

echo "Enabling service..."
systemctl enable ${SERVICE_NAME}.service

echo "Starting service..."
systemctl start ${SERVICE_NAME}.service

echo "Done!"
echo "Check status with: systemctl status ${SERVICE_NAME}.service"
