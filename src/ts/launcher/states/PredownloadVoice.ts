import type Launcher from '../../Launcher';

import Voice from '../../Voice';

export default (launcher: Launcher, prevGameVersion: string|null = null): Promise<void> => {
    return new Promise(async (resolve) => {
        Voice.predownloadUpdate(await Voice.selected, prevGameVersion).then((stream) => {
            launcher.progressBar?.init({
                label: 'Downloading voice package...',
                showSpeed: true,
                showEta: true,
                showPercents: true,
                showTotals: true
            });

            stream?.start(() => launcher.progressBar?.show());

            stream?.progress((current: number, total: number, difference: number) => {
                launcher.progressBar?.update(current, total, difference);
            });

            stream?.finish(() => {
                launcher.progressBar?.hide();

                resolve();
            });
        });
    });
};
