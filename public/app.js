let settings = null;
let currentImageNum = -1;

function updateClock() {
    const now = new Date();
    // Format to HH:MM like Rust backend did
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
    }
    if (!settings) return;

    // Use same logic as Rust frontend: 
    // image_num.set((chrono::Local::now().timestamp() as i32) / settings.interval);
    const ts = Math.floor(Date.now() / 1000);
    const newImageNum = Math.floor(ts / settings.interval);

    if (newImageNum !== currentImageNum) {
        currentImageNum = newImageNum;
        const imgElement = document.getElementById('currentImage');
        imgElement.src = `/api/image/${currentImageNum}`;

        try {
            const metaResponse = await fetch(`/api/image/${currentImageNum}/metadata`);
            const metadata = await metaResponse.json();

            document.getElementById('metaDate').innerText = metadata.date || "";
            document.getElementById('metaDescription').innerText = metadata.description || "";
        } catch (e) {
            console.error("Failed to fetch metadata", e);
            document.getElementById('metaDate').innerText = "";
            document.getElementById('metaDescription').innerText = "";
        }
    }
}

// Initialization
setInterval(updateClock, 1000);
updateClock();

setInterval(updateImage, 1000); // Check every second if the interval has passed
updateImage();
