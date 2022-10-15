import type Launcher from '../../Launcher';

import { Debug, fs } from '../../../empathize';
import { DebugThread } from '@empathize/framework/dist/meta/Debug';

import constants from '../../Constants';
import Locales from '../Locales';
import HDiffPatch from '../../core/HDiffPatch';
import { FilesVerifier, FilesRepairer } from './CheckIntegrity';
import Voice from '../../Voice';
import Patch from '../../Patch';


export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        const debugThread = new DebugThread('Launcher/State/ApplyChanges', 'Applying hdiff patches...');

        const gameDir = await constants.paths.gameDir;

        Neutralino.filesystem.readFile(`${gameDir}/hdifffiles.txt`)
            .then(async (filesString) => {
                let patchFails: string[] = [];

                const files = filesString.split(/\r\n|\r|\n/).filter((file) => file != '');

                if (files.length > 0)
                {
                    debugThread.log('Listing hdiff patches...');

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

                            debugThread.log(`Patching ${filePatchInfo.remoteName}: ${patchResult ? 'success' : 'failure'}`);

                            if (patchResult)
                            {
                                await Neutralino.filesystem.removeFile(`${gameDir}/${filePatchInfo.remoteName}`);
                                await Neutralino.filesystem.removeFile(`${gameDir}/${filePatchInfo.remoteName}.hdiff`);

                                await Neutralino.filesystem.moveFile(
                                    `${gameDir}/${filePatchInfo.remoteName}.hdiff_patched`,
                                    `${gameDir}/${filePatchInfo.remoteName}`
                                );
                            }

                            else patchFails.push(filePatchInfo.remoteName);
                        }

                        launcher.progressBar?.update(++current, total, 1);
                    }

                    // If we have some files failed to be patched
                    // then we'll try to repair them
                    if (patchFails.length > 0)
                    {
                        let files = await FilesVerifier.getIntegrityFiles((await Voice.installed).map((lang) => lang.lang));

                        files = files.filter((file) => patchFails.includes(file.remoteName));

                        const verifier = new FilesVerifier(files, gameDir, await Patch.latest, launcher, debugThread);
                        const repairer = new FilesRepairer(await verifier.process(), launcher, debugThread);

                        launcher.progressBar?.init({
                            label: Locales.translate<string>('launcher.progress.game.download_mismatch_files'),
                            showSpeed: false,
                            showEta: true,
                            showPercents: true,
                            showTotals: false
                        });

                        repairer.progress((current, total) => {
                            launcher.progressBar?.update(current, total, 1);
                        });

                        // Repair broken files
                        // TODO: notify about errors
                        await repairer.process();

                        // Remove hdiff files
                        for (const file of files)
                            await Neutralino.filesystem.removeFile(`${gameDir}/${file.remoteName}.hdiff`);
                    }

                    Debug.log({
                        function: 'Launcher/States/Install',
                        message: [
                            `Applied changes${patchFails.length > 0 ? ` (${patchFails.length} errors)` : ''}:`,
                            ...files
                        ]
                    });
                }

                await Neutralino.filesystem.removeFile(`${gameDir}/hdifffiles.txt`);

                launcher.progressBar?.hide();

                resolve();
            })
            .catch(() => resolve());
    });
};
