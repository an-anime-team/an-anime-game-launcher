import type Launcher from '../../Launcher';

import Patch from '../../Patch';
import Notifications from '../../core/Notifications';
import constants from '../../Constants';

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        Patch.latest.then((patch) => {
            if (patch.applied)
                resolve();

            else
            {
                launcher.progressBar?.init({
                    label: 'Downloading patch...',
                    showSpeed: false,
                    showEta: false,
                    showPercents: true,
                    showTotals: true
                });

                Patch.install(patch).then((stream) => {
                    if (stream === null)
                        resolve();

                    else
                    {
                        stream.downloadStart(() => launcher.progressBar?.show());

                        stream.downloadProgress((current: number, total: number, difference: number) => {
                            launcher.progressBar?.update(current, total, difference);
                        });
            
                        stream.unpackStart(() => {
                            launcher.progressBar?.init({
                                label: 'Unpacking patch...',
                                showSpeed: false,
                                showEta: false,
                                showPercents: true,
                                showTotals: true
                            });
        
                            launcher.progressBar?.show();
                        });
            
                        stream.unpackProgress((current: number, total: number, difference: number) => {
                            launcher.progressBar?.update(current, total, difference);
                        });

                        stream.unpackFinish(() => {
                            launcher.progressBar?.init({
                                label: 'Applying patch...',
                                showSpeed: false,
                                showEta: false,
                                showPercents: false,
                                showTotals: false
                            });
                        });

                        stream.patchFinish((result) => {
                            launcher.progressBar?.hide();

                            // If for some reasons patch wasn't applied successfully
                            if (!result)
                            {
                                Notifications.show({
                                    title: 'An Anime Game Launcher',
                                    body: 'Patch wasn\'t applied successfully. Please, check your log file to find a reason of it, or ask someone in our discord server',
                                    icon: `${constants.paths.appDir}/public/images/baal64-transparent.png`
                                });
                            }

                            resolve();
                        });
                    }
                });
            }
        });
    });
};
