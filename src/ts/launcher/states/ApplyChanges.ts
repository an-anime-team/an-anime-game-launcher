import type Launcher from '../../Launcher';

import { Debug, Notification, fs } from '../../../empathize';

import constants from '../../Constants';
import Locales from '../Locales';
import HDiffPatch from '../../core/HDiffPatch';

declare const Neutralino;

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        const gameDir = await constants.paths.gameDir;

        Neutralino.filesystem.readFile(`${gameDir}/hdifffiles.txt`)
            .then(async (files) => {
                let patchErrors = 0;

                files = files.split(/\r\n|\r|\n/).filter((file) => file != '');

                if (files.length > 0)
                {
                    launcher.progressBar?.init({
                        label: Locales.translate<string>('launcher.progress.game.applying_changes'),
                        showSpeed: false,
                        showEta: true,
                        showPercents: true,
                        showTotals: false
                    });

                    launcher.progressBar?.show();

                    let current = 0, total = files.length;

                    for (const file of files)
                    {
                        // {"remoteName": "AnAnimeGame_Data/StreamingAssets/Audio/GeneratedSoundBanks/Windows/Banks0.pck"}
                        const filePatchInfo = JSON.parse(file) as { remoteName: string };

                        if (await fs.exists(`${gameDir}/${filePatchInfo.remoteName}.hdiff`))
                        {
                            const patchResult = await HDiffPatch.patch(
                                `${gameDir}/${filePatchInfo.remoteName}`,
                                `${gameDir}/${filePatchInfo.remoteName}.hdiff`,
                                `${gameDir}/${filePatchInfo.remoteName}.hdiff_patched`
                            );

                            if (patchResult)
                            {
                                await Neutralino.filesystem.removeFile(`${gameDir}/${filePatchInfo.remoteName}`);
                                await Neutralino.filesystem.removeFile(`${gameDir}/${filePatchInfo.remoteName}.hdiff`);

                                await Neutralino.filesystem.moveFile(
                                    `${gameDir}/${filePatchInfo.remoteName}.hdiff_patched`,
                                    `${gameDir}/${filePatchInfo.remoteName}`
                                );
                            }

                            else ++patchErrors;
                        }

                        launcher.progressBar?.update(++current, total, 1);
                    }

                    if (patchErrors > 0)
                    {
                        const locale = Locales.translate<{ title: string, body: string }>('notifications.game_changes_applying_error');
                        
                        Notification.show({
                            title: locale.title,
                            body: locale.body.replace('{files}', patchErrors.toString()),
                            icon: `${constants.paths.appDir}/public/images/baal64-transparent.png`,
                            importance: 'critical'
                        });
                    }

                    Debug.log({
                        function: 'Launcher/States/Install',
                        message: [
                            `Applied changes${patchErrors > 0 ? ` (${patchErrors} errors)` : ''}:`,
                            ...files
                        ]
                    });
                }

                if (patchErrors === 0)
                    await Neutralino.filesystem.removeFile(`${gameDir}/hdifffiles.txt`);

                launcher.progressBar?.hide();

                resolve();
            })
            .catch(() => resolve());
    });
};
