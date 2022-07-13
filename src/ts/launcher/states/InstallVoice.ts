import { get as svelteget } from 'svelte/store';
import { _ } from 'svelte-i18n';

import type Launcher from '../../Launcher';
import type { VoiceLang } from '../../types/Voice';

import { promisify } from '../../../empathize';

import Voice from '../../Voice';
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
                    label: svelteget(_)('launcher.progress.voice.deleting'),
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
                                label: svelteget(_)('launcher.progress.voice.downloading', { values: { voice: selectedVoice } }),
                                showSpeed: true,
                                showEta: true,
                                showPercents: true,
                                showTotals: true
                            });

                            // Show pause/resume button
                            launcher.state!.pauseButton.style['display'] = 'block';

                            let paused = false;

                            launcher.state!.pauseButton.onclick = () => {
                                if (!paused)
                                {
                                    stream?.pauseDownload();

                                    launcher.state!.pauseButton.textContent = svelteget(_)('launcher.progress.resume');
                                }

                                else
                                {
                                    stream?.resumeDownload();

                                    launcher.state!.pauseButton.textContent = svelteget(_)('launcher.progress.pause');
                                }

                                paused = !paused;
                            };
                
                            stream?.downloadStart(() => launcher.progressBar?.show());
                
                            stream?.downloadProgress((current: number, total: number, difference: number) => {
                                launcher.progressBar?.update(current, total, difference);
                            });
                
                            stream?.unpackStart(() => {
                                launcher.progressBar?.init({
                                    label: svelteget(_)('launcher.progress.voice.unpacking', { values: { voice: selectedVoice } }),
                                    showSpeed: true,
                                    showEta: true,
                                    showPercents: true,
                                    showTotals: true
                                });

                                // Showing progress bar again
                                // in case if this update was pre-downloaded
                                // and we skipped downloadStart event
                                launcher.progressBar?.show();
                            });
                
                            stream?.unpackProgress((current: number, total: number, difference: number) => {
                                launcher.progressBar?.update(current, total, difference);
                            });
                
                            stream?.unpackFinish(() => {
                                launcher.progressBar?.hide();

                                // Hide pause/resume button
                                launcher.state!.pauseButton.style['display'] = 'none';

                                // Apply hdiff changes
                                import('./ApplyChanges').then((module) => {
                                    module.default(launcher).then(() => {
                                        // Remove outdated files
                                        import('./RemoveOutdated').then((module) => {
                                            module.default(launcher).then(() => resolve());
                                        });
                                    });
                                });
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
