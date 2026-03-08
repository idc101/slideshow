import express from 'express';
import path from 'path';
import fs from 'fs';
import { execFileSync } from 'child_process';
import { AppState } from './slideshow';

const app = express();
const port = process.env.PORT ? parseInt(process.env.PORT, 10) : 8000;

// Initialize app state
const state = new AppState();
const picturesBase = process.env.PICTURES_BASE || '.';
state.setPath(picturesBase);

app.use(express.static(path.join(__dirname, '../public')));

app.get('/api/settings', (req, res) => {
    console.log(`[${new Date().toISOString()}] GET /api/settings - serving settings`);
    res.json(state.settings);
});

app.get('/api/image/:num', (req, res) => {
    const num = parseInt(req.params.num, 10);
    console.log(`[${new Date().toISOString()}] GET /api/image/${num} - serving image`);
    if (isNaN(num)) {
        return res.status(400).send('Invalid image number');
    }
    const imagePath = state.getImage(num);
    if (!imagePath) {
        return res.status(404).send('Image not found');
    }

    const ext = path.extname(imagePath).toLowerCase();
    if (ext === '.heic' || ext === '.heif') {
        const cacheDir = path.join(__dirname, '../.cache');
        if (!fs.existsSync(cacheDir)) {
            fs.mkdirSync(cacheDir, { recursive: true });
        }

        // Use a simple hash of the path to avoid filename collisions
        const cacheFile = path.join(cacheDir, Buffer.from(imagePath).toString('base64').replace(/[/+=]/g, '_') + '.jpg');

        if (!fs.existsSync(cacheFile)) {
            console.log(`[${new Date().toISOString()}] Converting HEIC to JPEG: ${imagePath}`);
            try {
                // Requires sips on macOS or ImageMagick/heif-convert on Linux
                // Assuming sips for macOS (USER OS is mac) or fall back to native if possible, but the prompt says 
                // "runs on a Raspberry Pi". So sips is not there. Let's use `heif-convert` or just standard ImageMagick.
                // Actually the cleanest way without knowing the system is a lightweight npm module or just failing gracefully if OS tools missing.
                // Let's assume standard ImageMagick is installed or they can install it: `sudo apt install libheif-examples` -> heif-convert.
                execFileSync('heif-convert', ['-q', '85', imagePath, cacheFile]);
            } catch (e) {
                console.error("Failed to convert HEIC", e);
                return res.status(500).send('Failed to convert HEIC image');
            }
        }
        return res.sendFile(cacheFile);
    }

    // Set caching headers similarly to standard static serving or as needed
    res.sendFile(imagePath);
});

app.get('/api/image/:num/metadata', async (req, res) => {
    const num = parseInt(req.params.num, 10);
    console.log(`[${new Date().toISOString()}] GET /api/image/${num}/metadata - serving metadata`);
    if (isNaN(num)) {
        return res.status(400).send('Invalid image number');
    }
    const metadata = await state.getImageMetadata(num);
    res.json(metadata);
});

app.listen(port, () => {
    console.log(`Slideshow listening on port ${port}`);
});
