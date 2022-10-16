import type Launcher from '../../Launcher';

import DXVK from '../../core/DXVK';
import constants from '../../Constants';

const DEFAULT_DXVK = '1.10.3';

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        // Create prefix if it is not created
        import('./CreatePrefix').then((module) => {
            module.default(launcher).then(() => {
                // And then download the DXVK
                DXVK.download(DEFAULT_DXVK).then((stream) => {
                    launcher.progressBar?.init({
                        label: `Downloading DXVK ${DEFAULT_DXVK}...`,
                        showSpeed: true,
                        showEta: true,
                        showPercents: true,
                        showTotals: true
                    });

                    stream?.downloadStart(() => launcher.progressBar?.show());

                    stream?.downloadProgress((current: number, total: number, difference: number) => {
                        launcher.progressBar?.update(current, total, difference);
                    });

                    let unpacking = true;

                    stream?.unpackStart(() => {
                        launcher.progressBar?.init({
                            label: () => unpacking ? `Unpacking DXVK ${DEFAULT_DXVK}...` : `Applying DXVK ${DEFAULT_DXVK}...`,
                            showSpeed: true,
                            showEta: true,
                            showPercents: true,
                            showTotals: true
                        });
                    });

                    stream?.unpackProgress((current: number, total: number, difference: number) => {
                        launcher.progressBar?.update(current, total, difference);
                    });

                    stream?.unpackFinish(async () => {
                        unpacking = false;

                        // Select this DXVK
                        await DXVK.current(DEFAULT_DXVK);

                        // And apply it
                        DXVK.apply(await constants.paths.prefix.current, DEFAULT_DXVK).then(() => {
                            launcher.progressBar?.hide();

                            resolve();
                        });
                    });
                });
            });
        });
    });
};
