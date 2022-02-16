import { Notification, Package } from '../../../empathize';

import type Launcher from '../../Launcher';

import Patch from '../../Patch';
import constants from '../../Constants';
import Locales from '../Locales';

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        // Show an error notification if xdelta3 package is not installed
        if (!await Package.exists('xdelta3'))
        {
            Notification.show({
                ...(Locales.translate('notifications.xdelta3_package_required') as { title: string, body: string }),
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
            
                        stream.applyingStart(() => {
                            launcher.progressBar?.init({
                                label: 'Applying patch...',
                                showSpeed: false,
                                showEta: false,
                                showPercents: false,
                                showTotals: false
                            });
        
                            launcher.progressBar?.show();
                        });

                        stream.applyingFinish((result) => {
                            launcher.progressBar?.hide();

                            // If for some reasons patch wasn't applied successfully
                            if (!result)
                            {
                                Notification.show({
                                    ...(Locales.translate('notifications.patch_applying_error') as { title: string, body: string }),
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
                ...(Locales.translate('notifications.patch_repos_unavailable') as { title: string, body: string }),
                icon: `${constants.paths.appDir}/public/images/baal64-transparent.png`,
                importance: 'critical'
            });

            resolve();
        });
    });
};
