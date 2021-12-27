import type Launcher from '../../Launcher';

import Game from '../../Game';
import Prefix from '../../core/Prefix';

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        Prefix.exists().then((exists) => {
            if (!exists)
            {
                import('./CreatePrefix').then((module) => {
                    module.default(launcher).then(() => updateGame());
                });
            }
        });

        const updateGame = async () => {
            const prevGameVersion = await Game.current;

            Game.update(prevGameVersion).then((stream) => {
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
    
                stream?.unpackFinish(() => {
                    // Download voice package when the game itself was installed
                    import('./InstallVoice').then((module) => {
                        module.default(launcher, prevGameVersion).then(() => resolve());
                    });
                });
            });
        };
    });
};
