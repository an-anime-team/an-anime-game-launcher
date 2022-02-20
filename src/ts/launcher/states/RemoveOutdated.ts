import type Launcher from '../../Launcher';

import { Debug } from '../../../empathize';

import constants from '../../Constants';
import Locales from '../Locales';

declare const Neutralino;

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        const gameDir = await constants.paths.gameDir;

        Neutralino.filesystem.readFile(`${gameDir}/deletefiles.txt`)
            .then(async (files) => {
                files = files.split(/\r\n|\r|\n/).filter((file) => file != '');

                if (files.length > 0)
                {
                    launcher.progressBar?.init({
                        label: Locales.translate<string>('launcher.progress.game.deleting_outdated'),
                        showSpeed: false,
                        showEta: true,
                        showPercents: true,
                        showTotals: false
                    });

                    launcher.progressBar?.show();

                    let current = 0, total = files.length;

                    for (const file of files)
                    {
                        await Neutralino.filesystem.removeFile(`${gameDir}/${file}`);

                        launcher.progressBar?.update(++current, total, 1);
                    }
                    
                    Debug.log({
                        function: 'Launcher/States/RemoveOutdated',
                        message: [
                            'Deleted outdated files:',
                            ...files
                        ]
                    });
                }

                await Neutralino.filesystem.removeFile(`${gameDir}/deletefiles.txt`);

                launcher.progressBar?.hide();

                resolve();
            })
            .catch(() => resolve());
    });
};
