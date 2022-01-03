import type Launcher from '../../Launcher';

import constants from '../../Constants';
import Prefix from '../../core/Prefix';

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        const prefixDir = await constants.paths.prefix.current;

        Prefix.exists(prefixDir).then((exists) => {
            if (exists)
                resolve();

            else
            {
                let progressLabel = 'Creating prefix...';

                launcher.progressBar!.init({
                    label: () => progressLabel,
                    showSpeed: false,
                    showEta: false,
                    showPercents: false,
                    showTotals: false
                });

                launcher.progressBar!.show();

                Prefix.create(prefixDir, (output, current, total) => {
                    progressLabel = output;

                    if (progressLabel.length > 70)
                        progressLabel = progressLabel.substring(0, 70) + '...';

                    launcher.progressBar!.update(current, total, 1);
                })
                .then(() => resolve());
            }
        });
    });
};
