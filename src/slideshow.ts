import ExifReader from 'exifreader';
import fs from 'fs';
import path from 'path';

export interface ImageMetadata {
    date: string | null;
    description: string | null;
}

export class AppState {
    allImages: string[] = [];
    counter: number = 0;
    settings: { slideshow: string; interval: number; kenBurns: boolean };
    picturesBase: string;

    constructor() {
        const intervalStr = process.env.SLIDESHOW_INTERVAL || '300';
        const interval = parseInt(intervalStr, 10);
        const kenBurnsStr = process.env.KEN_BURNS || 'false';
        const kenBurns = kenBurnsStr.toLowerCase() === 'true' || kenBurnsStr === '1';
        this.settings = { slideshow: 'slideshow', interval: isNaN(interval) ? 300 : interval, kenBurns };
        this.picturesBase = '';
    }

    setPath(picturesBase: string) {
        this.picturesBase = path.resolve(picturesBase);
        this.allImages = [];
        console.log(`Setting pictures_base to: ${this.picturesBase}`);
        this.scanDirectory(this.picturesBase);

        // Match Rust's shuffle using a simple Fisher-Yates shuffle
        this.shuffle(this.allImages);
        console.log(`Found ${this.allImages.length} images`);
    }

    private scanDirectory(dir: string) {
        let files;
        try {
            files = fs.readdirSync(dir, { withFileTypes: true });
        } catch (e) {
            return;
        }

        for (const file of files) {
            const res = path.resolve(dir, file.name);
            if (file.isDirectory()) {
                this.scanDirectory(res);
            } else {
                const ext = path.extname(res).toLowerCase();
                if (ext === '.jpg' || ext === '.jpeg' || ext === '.png' || ext === '.webp') {
                    this.allImages.push(res);
                }
            }
        }
    }

    private shuffle(array: string[]) {
        for (let i = array.length - 1; i > 0; i--) {
            // Seeded random is omitted, simple random is fine
            const j = Math.floor(Math.random() * (i + 1));
            [array[i], array[j]] = [array[j], array[i]];
        }
    }

    getCurrentImage(): string {
        if (this.allImages.length === 0) return '';
        const MathRandomIndex = Math.floor(Math.random() * this.allImages.length);
        return this.allImages[MathRandomIndex];
    }

    getImage(num: number): string {
        if (this.allImages.length === 0) return '';
        // handle negative modulo properly
        let index = num % this.allImages.length;
        if (index < 0) {
            index = index + this.allImages.length;
        }
        return this.allImages[index];
    }

    increment(): void {
        this.counter++;
    }

    get(): number {
        return this.counter;
    }

    async getImageMetadata(num: number): Promise<ImageMetadata> {
        const filepath = this.getImage(num);
        if (!filepath) return { date: null, description: null };

        const description = AppState.filenameToDescription(filepath, this.picturesBase);
        let date: string | null = null;
        try {
            const fileBuffer = fs.readFileSync(filepath);
            const tags = ExifReader.load(fileBuffer);

            let dateStr = tags['DateTimeOriginal']?.description || tags['DateTime']?.description;
            if (dateStr) {
                // e.g., "2024:10:28 10:08:33"
                // Extract parts to bypass timezone locale issues
                const match = dateStr.match(/^(\d{4})[:\-](\d{2})[:\-](\d{2}) (\d{2}):(\d{2}):(\d{2})/);
                if (match) {
                    const d = new Date(
                        parseInt(match[1], 10),
                        parseInt(match[2], 10) - 1,
                        parseInt(match[3], 10),
                        parseInt(match[4], 10),
                        parseInt(match[5], 10),
                        parseInt(match[6], 10)
                    );
                    const days = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
                    const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
                    date = `${days[d.getDay()]} ${match[3]} ${months[d.getMonth()]} ${match[1]}`;
                } else {
                    date = dateStr;
                }
            }
        } catch (err) {
            // Exif format not supported or no exif
        }

        return { date, description };
    }

    static filenameToDescription(filename: string, picturesBase: string): string | null {
        // Handle differences in forward/backward slashes on different OSes if necessary
        const normalizedFilename = filename.split(path.sep).join('/');
        const normalizedBase = picturesBase.split(path.sep).join('/');

        let baseName = normalizedFilename.startsWith(normalizedBase)
            ? normalizedFilename.substring(normalizedBase.length)
            : null;

        if (baseName === null) {
            // Check if we passed a relative file path that starts with the same base name
            if (filename.startsWith(picturesBase) && picturesBase !== "") {
                baseName = filename.substring(picturesBase.length);
            } else {
                return null;
            }
        }

        if (baseName.startsWith('/')) baseName = baseName.substring(1);

        let current = baseName;
        current = current.replace(/\..+$/g, "");
        current = current.replace(/^\d{4}\//g, "");
        current = current.replace(/^\d{4}-\d{2}-\d{2}[ -]/g, "");
        current = current.replace(/^\d{4}-\d{2}[ -]/g, "");
        current = current.replace(/\d{8}T\d{6}[-]/g, "");
        current = current.replace(/-\d{3}$/g, "");
        current = current.replace(/IMG_\d{4}/g, "");
        current = current.replace(/DCS_\d{4}/g, "");

        // alnum_regex replace_all
        current = current.replace(/^[^a-zA-Z0-9]+|[^a-zA-Z0-9]+$/g, "");

        return current;
    }
}
