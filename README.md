# Node.js Slideshow Application

A lightweight web-based slideshow application to randomly display images from a directory.
Migrated from Rust to TypeScript Node.js, specifically optimized for easier setup on Raspberry Pi.

## Why?

I run it on a Raspberry pi, hooked up to a TV to have a slideshow of my favorite photos.

## Project Structure

- `src/`: Express backend API and file scanner.
- `public/`: Vanilla JS and HTML frontend.

## Prerequisites

- Node.js (v18+)
- PM2 (for running in background: `npm install -g pm2`)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/idc101/slideshow.git
cd slideshow
```

2. Install dependencies and build:
```bash
npm install
npm run build
```

3. Run the application:
```bash
# To run normally
PICTURES_BASE=/path/to/photos npm start

# Or to run with PM2 in the background
PICTURES_BASE=/path/to/photos npm run pm2
```

4. The application will be available at `http://localhost:8000`

## Configuration

The application can be configured through the following environment variables:

- `PICTURES_BASE`: Slideshow directory path
- `SLIDESHOW_INTERVAL`: Interval in seconds to wait before changing image (default 300)
- `PORT`: Port to run the Express server on (default 8000)

## Raspberry Pi and TV Setup

This setup runs the slideshow with PM2 and launches a fullscreen Chromium browser with the slideshow URL to display on HDMI.

1. Install Chromium browser:
```bash
sudo apt-get update
sudo apt-get install chromium
```

2. Install PM2 and save it to startup:
```bash
sudo npm install -g pm2
pm2 startup
# Follow the command provided by pm2 startup
```

3. Start the app:
```bash
PICTURES_BASE=/path/to/photos pm2 start dist/index.js --name "slideshow"
pm2 save
```

4. Create `~/bin/start-chromium.sh`:
```bash
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

5. Launch Chromium on startup:
```bash
echo "@sh $HOME/bin/start-chromium.sh" >> ~/.config/lxsession/LXDE-pi/autostart
```

### TV Control

If you have a TV connected via HDMI, you can turn it on and off with [CEC Control](https://gist.github.com/rmtsrc/dc35cd1458cd995631a4f041ab11ff74).

## API Endpoints

The backend provides the following API endpoints:

- `GET /api/settings` - Get interval settings
- `GET /api/image/<num>` - Get a specific image
- `GET /api/image/<num>/metadata` - Get metadata for a specific image
