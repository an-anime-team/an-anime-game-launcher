import type Launcher from '../../Launcher';
import type { VoiceLang } from '../../types/Voice';

import Voice from '../../Voice';
import promisify from '../../core/promisify';
import Game from '../../Game';

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        Voice.selected.then(async (selected: VoiceLang[]) => {
            const installedVoices = await Voice.installed;
            const currentVersion = await Game.current;

            let packagesToDelete: VoiceLang[] = [],
                packagesVersions = {};

            for (const installedVoice of installedVoices)
            {
                packagesVersions[installedVoice.lang] = installedVoice.version;

                if (!selected.includes(installedVoice.lang))
                    packagesToDelete.push(installedVoice.lang);
            }

            if (packagesToDelete.length > 0)
            {
                launcher.progressBar?.init({
                    label: `Deleting voice packages...`,
                    showSpeed: false,
                    showEta: false,
                    showPercents: true,
                    showTotals: false
                });

                launcher.progressBar?.show();

                for (let i = 0; i < packagesToDelete.length; ++i)
                {
                    await Voice.delete(packagesToDelete[i]);

                    launcher.progressBar?.update(i + 1, packagesToDelete.length, 1);
                }
            }

            const updateVoices = promisify({
                callbacks: selected.map((selectedVoice: VoiceLang) => {
                    return (): Promise<void> => new Promise((resolve) => {
                        if (packagesVersions[selectedVoice] === currentVersion)
                            resolve();
                        
                        else Voice.update(selectedVoice, packagesVersions[selectedVoice] ?? null).then((stream) => {
                            launcher.progressBar?.init({
                                label: `Downloading ${selectedVoice} voice package...`,
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
                                    label: `Unpacking ${selectedVoice} voice package...`,
                                    showSpeed: true,
                                    showEta: true,
                                    showPercents: true,
                                    showTotals: true
                                });

                                launcher.progressBar?.show();
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
                }),
                interval: 3000
            });

            updateVoices.then(() => resolve());
        });
    });
};
