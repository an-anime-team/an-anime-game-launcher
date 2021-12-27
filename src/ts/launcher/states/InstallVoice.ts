import type Launcher from '../../Launcher';

import Voice from '../../Voice';

export default (launcher: Launcher, prevGameVersion: string|null = null): Promise<void> => {
    return new Promise(async (resolve) => {
        Voice.update(await Voice.selected, prevGameVersion).then((stream) => {
            launcher.progressBar?.init({
                label: 'Downloading voice package...',
                showSpeed: true,
                showEta: true,
                showPercents: true,
                showTotals: true
            });

            stream?.downloadStart(() => launcher.progressBar?.show());

            stream?.downloadProgress((current: number, total: number, difference: number) => {
                launcher.progressBar?.update(current, total, difference);
            });

            stream?.unpackStart(() => {
                launcher.progressBar?.init({
                    label: 'Unpacking voice package...',
                    showSpeed: true,
                    showEta: true,
                    showPercents: true,
                    showTotals: true
                });
            });

            stream?.unpackProgress((current: number, total: number, difference: number) => {
                launcher.progressBar?.update(current, total, difference);
            });

            stream?.unpackFinish(() => {
                launcher.progressBar?.hide();

                resolve();
            });
        });
    });
};
