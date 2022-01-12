import type Launcher from '../../Launcher';
import type { VoiceLang } from '../../types/Voice';

import Voice from '../../Voice';
import promisify from '../../core/promisify';

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        let packagesVersions = {};

        for (const installedVoice of await Voice.installed)
            packagesVersions[installedVoice.lang] = installedVoice.version;

        Voice.selected.then(async (selected: VoiceLang[]) => {
            const updateVoices = promisify({
                callbacks: selected.map((selectedVoice: VoiceLang) => {
                    return (): Promise<void> => new Promise((resolve) => {
                        Voice.predownloadUpdate(selectedVoice, packagesVersions[selectedVoice] ?? null).then((stream) => {
                            launcher.progressBar?.init({
                                label: `Pre-downloading ${selectedVoice} voice package...`,
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
                }),
                interval: 3000
            });

            updateVoices.then(() => resolve());
        });
    });
};
