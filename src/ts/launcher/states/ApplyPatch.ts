import { Notification } from '../../../empathize';

import Launcher from '../../Launcher';
import Patch from '../../Patch';
import constants from '../../Constants';

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        // Show an error notification if xdelta3 package is not installed
        if (!await Launcher.isPackageAvailable('xdelta3'))
        {
            Notification.show({
                title: 'An Anime Game Launcher',
                body: 'You must download xdelta3 package to apply the patch',
                icon: `${constants.paths.appDir}/public/images/baal64-transparent.png`,
                importance: 'critical'
            });

            resolve();
        }

        else Patch.latest.then((patch) => {
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
                                Notification.show({
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
        }).catch(() => {
            Notification.show({
                title: 'An Anime Game Launcher',
                body: 'All the patch repositories are not available. You\'ll be able to run the game, but launcher can\'t be sure is it patched properly',
                icon: `${constants.paths.appDir}/public/images/baal64-transparent.png`,
                importance: 'critical'
            });

            resolve();
        });
    });
};
