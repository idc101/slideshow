let settings = null;
let currentImageNum = -1;
let timeOffset = 0;

function log(msg) {
    console.log(`[${new Date().toISOString()}] ${msg}`);
}

// --- Face Detection Setup ---
let facefinder_classify_region = function (r, c, s, pixels, ldim) { return -1.0; };
fetch('/facefinder').then(function (response) {
    if (!response.ok) {
        console.error('Failed to fetch facefinder cascade file:', response.status);
        return;
    }
    response.arrayBuffer().then(function (buffer) {
        let bytes = new Int8Array(buffer);
        facefinder_classify_region = pico.unpack_cascade(bytes);
        console.log('* cascade loaded');
    })
});

function rgba_to_grayscale(rgba, nrows, ncols) {
    var gray = new Uint8Array(nrows * ncols);
    for (var r = 0; r < nrows; ++r)
        for (var c = 0; c < ncols; ++c)
            gray[r * ncols + c] = (2 * rgba[r * 4 * ncols + 4 * c + 0] + 7 * rgba[r * 4 * ncols + 4 * c + 1] + 1 * rgba[r * 4 * ncols + 4 * c + 2]) / 10;
    return gray;
}

function findFaces(img) {
    const maxDim = 640;
    let scale = Math.min(maxDim / img.naturalWidth, maxDim / img.naturalHeight);
    if (scale > 1.0) scale = 1.0;

    const width = Math.floor(img.naturalWidth * scale);
    const height = Math.floor(img.naturalHeight * scale);

    const canvas = document.createElement('canvas');
    canvas.width = width;
    canvas.height = height;
    const ctx = canvas.getContext('2d', { willReadFrequently: true });
    ctx.drawImage(img, 0, 0, width, height);

    const rgba = ctx.getImageData(0, 0, width, height).data;
    const image = {
        "pixels": rgba_to_grayscale(rgba, height, width),
        "nrows": height,
        "ncols": width,
        "ldim": width
    }

    const params = {
        "shiftfactor": 0.1,
        "minsize": 20,
        "maxsize": 1000,
        "scalefactor": 1.1
    }

    let dets = pico.run_cascade(image, facefinder_classify_region, params);
    dets = pico.cluster_detections(dets, 0.2);

    const faces = [];
    for (let i = 0; i < dets.length; ++i) {
        if (dets[i][3] > 50.0) {
            faces.push({
                r: dets[i][0] / scale,
                c: dets[i][1] / scale,
                s: dets[i][2] / scale
            });
        }
    }
    log(`Detected ${faces.length} faces from pico.js on ${img.src}`);
    return faces;
}

// --- Ken Burns Logic ---

function generateKenBurnsState(imgWidth, imgHeight, vw, vh, faces) {
    const sCover = Math.max(vw / imgWidth, vh / imgHeight);
    // Add 10% to 30% extra zoom for panning freedom
    const S = sCover * (1.1 + Math.random() * 0.2);

    const minTxImg = vw - imgWidth * S;
    const maxTxImg = 0;
    const minTyImg = vh - imgHeight * S;
    const maxTyImg = 0;

    let tx, ty;

    if (faces && faces.length > 0) {
        let fLeft = imgWidth, fRight = 0, fTop = imgHeight, fBottom = 0;
        for (const face of faces) {
            fLeft = Math.min(fLeft, face.c - face.s / 2);
            fRight = Math.max(fRight, face.c + face.s / 2);
            fTop = Math.min(fTop, face.r - face.s / 2);
            fBottom = Math.max(fBottom, face.r + face.s / 2);
        }

        const padding = 50;
        const minTxFace = vw - padding - fRight * S;
        const maxTxFace = padding - fLeft * S;

        let validMinTx = Math.max(minTxImg, minTxFace);
        let validMaxTx = Math.min(maxTxImg, maxTxFace);

        if (validMinTx > validMaxTx) {
            let idealTx = vw / 2 - (fLeft + fRight) / 2 * S;
            tx = Math.max(minTxImg, Math.min(maxTxImg, idealTx));
        } else {
            tx = validMinTx + Math.random() * (validMaxTx - validMinTx);
        }

        const minTyFace = vh - padding - fBottom * S;
        const maxTyFace = padding - fTop * S;

        let validMinTy = Math.max(minTyImg, minTyFace);
        let validMaxTy = Math.min(maxTyImg, maxTyFace);

        if (validMinTy > validMaxTy) {
            let idealTy = vh / 2 - (fTop + fBottom) / 2 * S;
            ty = Math.max(minTyImg, Math.min(maxTyImg, idealTy));
        } else {
            ty = validMinTy + Math.random() * (validMaxTy - validMinTy);
        }

    } else {
        tx = minTxImg + Math.random() * (maxTxImg - minTxImg);
        ty = minTyImg + Math.random() * (maxTyImg - minTyImg);
    }

    return { S, tx, ty };
}

let currentAnimation = null;

function applyKenBurns(imgElement, imgObj, durationMs) {
    const vw = window.innerWidth;
    const vh = window.innerHeight;

    // Find faces first
    const faces = findFaces(imgObj);

    // Apply necessary styles
    imgElement.style.position = 'absolute';
    imgElement.style.transformOrigin = '0 0';
    imgElement.style.width = imgObj.naturalWidth + 'px';
    imgElement.style.height = imgObj.naturalHeight + 'px';
    imgElement.style.objectFit = 'fill';

    // Generate two states
    const state1 = generateKenBurnsState(imgObj.naturalWidth, imgObj.naturalHeight, vw, vh, faces);
    const state2 = generateKenBurnsState(imgObj.naturalWidth, imgObj.naturalHeight, vw, vh, faces);

    if (currentAnimation) {
        currentAnimation.cancel();
    }

    currentAnimation = imgElement.animate([
        { transform: `translate(${state1.tx}px, ${state1.ty}px) scale(${state1.S})` },
        { transform: `translate(${state2.tx}px, ${state2.ty}px) scale(${state2.S})` }
    ], {
        duration: durationMs + 2000, // extend slightly past the transition
        fill: 'forwards',
        easing: 'ease-in-out'
    });
}

function disableKenBurns(imgElement) {
    if (currentAnimation) {
        currentAnimation.cancel();
        currentAnimation = null;
    }
    imgElement.style.position = '';
    imgElement.style.transformOrigin = '';
    imgElement.style.transform = '';
    imgElement.style.width = '100%';
    imgElement.style.height = '100%';
    imgElement.style.objectFit = 'contain';
}

// --- Main App Logic ---

function updateClock() {
    const now = new Date();
    const hours = now.getHours().toString().padStart(2, '0');
    const minutes = now.getMinutes().toString().padStart(2, '0');
    document.getElementById('clock').innerText = `${hours}:${minutes}`;
}

async function fetchSettings() {
    try {
        const response = await fetch('/api/settings');
        settings = await response.json();
    } catch (e) {
        console.error("Failed to fetch settings", e);
    }
}

async function updateImage() {
    if (!settings) {
        await fetchSettings();
        if (settings) {
            log(`Loaded settings. Ken Burns effect is: ${settings.kenBurns ? 'ON' : 'OFF'}`);
        }
    }
    if (!settings) return;

    const ts = Math.floor(Date.now() / 1000) + timeOffset;
    const newImageNum = Math.floor(ts / settings.interval);

    if (newImageNum !== currentImageNum) {
        currentImageNum = newImageNum;
        const imgElement = document.getElementById('currentImage');
        const nextSrc = `/api/image/${currentImageNum}`;

        // Fetch metadata concurrently
        fetch(`/api/image/${currentImageNum}/metadata`).then(r => r.json()).then(metadata => {
            document.getElementById('metaDate').innerText = metadata.date || "";
            document.getElementById('metaDescription').innerText = metadata.description || "";
        }).catch(e => {
            document.getElementById('metaDate').innerText = "";
            document.getElementById('metaDescription').innerText = "";
        });

        if (settings.kenBurns) {
            // Load image in background first for processing
            log(`Loading new image for Ken Burns: ${nextSrc}`);
            const img = new Image();
            img.onload = () => {
                log(`Image loaded completely: ${nextSrc}. Applying Ken Burns math.`);
                imgElement.src = nextSrc;
                applyKenBurns(imgElement, img, settings.interval * 1000);
            };
            img.src = nextSrc;
        } else {
            disableKenBurns(imgElement);
            imgElement.src = nextSrc;
        }
    }
}

// Keyboard controls
window.addEventListener('keydown', (e) => {
    if (!settings) return;
    if (e.key === 'ArrowRight') {
        timeOffset += settings.interval;
        updateImage();
    } else if (e.key === 'ArrowLeft') {
        timeOffset -= settings.interval;
        updateImage();
    } else if (e.key === '0') {
        timeOffset = 0;
        updateImage();
    }
});

// Initialization
setInterval(updateClock, 1000);
updateClock();

setInterval(updateImage, 1000);
updateImage();
