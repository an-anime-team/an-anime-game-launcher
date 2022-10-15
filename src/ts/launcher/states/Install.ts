import type Launcher from '../../Launcher';

import { promisify, Configs } from '../../../empathize';

import Game from '../../Game';
import Prefix from '../../core/Prefix';
import constants from '../../Constants';
import Locales from '../Locales';

declare const Neutralino;

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        const prefixDir = await constants.paths.prefix.current;
        
        Prefix.exists(prefixDir).then((exists) => {
            if (!exists)
            {
                import('./CreatePrefix').then((module) => {
                    module.default(launcher).then(() => updateGame());
                });
            }

            else updateGame();
        });

        const updateGame = async () => {
            const prevGameVersion = await Game.current;

            Game.update(prevGameVersion).then((stream) => {
                launcher.progressBar?.init({
                    label: Locales.translate('launcher.progress.game.downloading'),
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

                        launcher.state!.pauseButton.textContent = Locales.translate('launcher.progress.resume');
                    }

                    else
                    {
                        stream?.resumeDownload();

                        launcher.state!.pauseButton.textContent = Locales.translate('launcher.progress.pause');
                    }

                    paused = !paused;
                };
    
                stream?.downloadStart(() => launcher.progressBar?.show());
    
                stream?.downloadProgress((current: number, total: number, difference: number) => {
                    launcher.progressBar?.update(current, total, difference);
                });
    
                stream?.unpackStart(() => {
                    launcher.progressBar?.init({
                        label: Locales.translate('launcher.progress.game.unpacking'),
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
    
                stream?.unpackFinish(async () => {
                    // Hide pause/resume button
                    launcher.state!.pauseButton.style['display'] = 'none';

                    await promisify({
                        callbacks: [
                            // Applying game's files changes
                            (): Promise<void> => new Promise((resolve) => {
                                import('./ApplyChanges').then((module) => {
                                    module.default(launcher).then(() => resolve());
                                });
                            }),

                            // Deleting outdated files
                            (): Promise<void> => new Promise((resolve) => {
                                import('./RemoveOutdated').then((module) => {
                                    module.default(launcher).then(() => resolve());
                                });
                            })
                        ]
                    });

                    if (!await Configs.get('skip_analytics'))
                        await Neutralino.filesystem.writeFile(`${await constants.paths.launcherDir}/.analytics`, '');

                    // Download voice package when the game itself has been installed
                    import('./InstallVoice').then((module) => {
                        module.default(launcher).then(() => resolve());
                    });
                });
            });
        };
    });
};
