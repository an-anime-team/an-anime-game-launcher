import type Launcher from '../../Launcher';

import Game from '../../Game';
import constants from '../../Constants';
import Runners from '../../core/Runners';

declare const Neutralino;

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        const prefixDir = await constants.paths.prefix.current;

        Neutralino.filesystem.getStats(prefixDir)
            .then(() => updateGame())
            .catch(() => {
                Runners.createPrefix(prefixDir).then((result) => {
                    if (result === true)
                        updateGame();

                    else
                    {
                        // TODO
                        console.error('There\'s no wine version installed to use to create the prefix');

                        resolve();
                    }
                });
            });

        const updateGame = async () => {
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
        };
    });
};
