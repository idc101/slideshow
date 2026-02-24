import { AppState } from './slideshow';

describe('AppState', () => {
    describe('filenameToDescription', () => {
        const picturesBase = '/base/path';

        it('should extract description correctly', () => {
            expect(AppState.filenameToDescription('/base/path/2025-02 Skiing - La Rosiere/IMG_9969.jpg', picturesBase))
                .toBe('Skiing - La Rosiere');

            expect(AppState.filenameToDescription('/base/path/2025/2025-02 Skiing - La Rosiere/IMG_9969.jpg', picturesBase))
                .toBe('Skiing - La Rosiere');

            expect(AppState.filenameToDescription('/base/path/2024-12 December-20241224T100546-019.jpg', picturesBase))
                .toBe('December');

            expect(AppState.filenameToDescription('/base/path/2024-12-25 Christmas-20241224T100546-019.jpg', picturesBase))
                .toBe('Christmas');
        });
    });
});
