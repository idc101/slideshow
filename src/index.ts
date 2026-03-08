import express from 'express';
import path from 'path';
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
