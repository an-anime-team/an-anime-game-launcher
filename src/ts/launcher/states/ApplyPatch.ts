import type Launcher from '../../Launcher';

import Patch from '../../Patch';

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        Patch.latest.then((patch) => {
            if (patch.applied)
                resolve();

            else
            {
                launcher.progressBar?.init({
                    label: 'Applying patch...',
                    showSpeed: false,
                    showEta: false,
                    showPercents: false,
                    showTotals: false
                });

                Patch.install().then((stream) => {
                    if (stream === null)
                        resolve();

                    else
                    {
                        stream.downloadStart(() => launcher.progressBar?.show());

                        stream.patchFinish(() => {
                            launcher.progressBar?.hide();

                            resolve();
                        });
                    }
                });
            }
        });
    });
};
