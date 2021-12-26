import type Launcher from '../../Launcher';

import Game from '../../Game';

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        Game.update(await Game.current).then((stream) => {
            launcher.progressBar?.init({
                label: 'Downloading game...',
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
                    label: 'Unpacking game...',
                    showSpeed: true,
                    showEta: true,
                    showPercents: true,
                    showTotals: true
                });
            });

            stream?.unpackProgress((current: number, total: number, difference: number) => {
                launcher.progressBar?.update(current, total, difference);
            });

            stream?.unpackFinish(() => resolve());
        });
    });
};
