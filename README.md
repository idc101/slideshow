# Rust Slideshow Application

A web-based slideshow application to randomly display images from a directory.
It is built with Rust, featuring a Rocket backend server and a Yew frontend framework.

## Why?

I run it on a Raspberry pi, hooked up to a TV to have a slideshow of my favorite photos.

## Project Structure

The project is divided into two main components:

- `backend/`: Rocket-based server application
- `frontend/`: Yew-based web application

## Prerequisites

- Rust (latest stable version)
- Cargo
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (for frontend development)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/idc101/slideshow.git
cd rust-hw
```

2. Run:
```bash
cargo xtask run
```

3. The application will be available at `http://localhost:8000`

## Configuration

The application can be configured through the following environment variables:

- `PICTURES_BASE`: Slideshow directory path

## Docker

```
docker build -t slideshow .
docker run -p 8000:8000 slideshow
```

## Raspberry Pi and TV Setup

This setup runs slideshow in docker and launches a fullscreen Chromium browser with the slideshow URL to display on HDMI.

1. Install Chromium browser:
```bash
sudo apt-get update
sudo apt-get install chromium-browser
```

2. Install Slideshow:
```
docker build -t slideshow .
```

3. Configure the slideshow to run on startup:
```
sudo nano /etc/rc.local
```
Add the following line before `exit 0`:
```
docker run -d -p 8000:8000 slideshow
```

4. Restart the Raspberry Pi:
```
sudo reboot
```

5. Create `~/bin/start-chromium.sh`:
```
#!/bin/sh

set -e

CHROMIUM_TEMP=~/tmp/chromium
rm -Rf ~/.config/chromium/
rm -Rf $CHROMIUM_TEMP
mkdir -p $CHROMIUM_TEMP

chromium-browser \
        --disable \
        --disable-translate \
        --disable-infobars \
        --disable-suggestions-service \
        --disable-save-password-bubble \
        --disk-cache-dir=$CHROMIUM_TEMP/cache/ \
        --user-data-dir=$CHROMIUM_TEMP/user_data/ \
        --start-maximized \
        --kiosk http://localhost:8000 &
```

6. Launch Chromium on startup:
```
echo "@sh $HOME/bin/start-chromium.sh" >> ~/.config/lxsession/LXDE-pi/autostart
```

### TV Control

If you have a TV connected via HDMI, you can turn it on and off with [CEC Control](https://gist.github.com/rmtsrc/dc35cd1458cd995631a4f041ab11ff74).

## API Endpoints

The backend provides the following API endpoints:

- `GET /api/image/<num>` - Get a specific image
- `GET /api/image/<num>/metadata` - Get metadata for a specific image

## Dependencies

### Backend
- Rocket - Web framework
- Tokio - Async runtime

### Frontend
- Yew - Frontend framework
- yew-router - Routing
- gloo-net - Network requests
- chrono - Time handling
- serde - Serialization/Deserialization
