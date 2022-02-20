import type Launcher from '../../Launcher';

import { Debug, fs, path } from '../../../empathize';

import constants from '../../Constants';
import Patch from "../../Patch";
import Locales from '../Locales';

declare const Neutralino;

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        const gameDir = await constants.paths.gameDir;

        Neutralino.filesystem.readFile(`${gameDir}/pkg_version`)
            .then(async (files) => {
                let mismatchedFiles = [];

                files = files.split(/\r\n|\r|\n/).filter((file) => file != '');

                if (files.length > 0)
                {
                    const patch = await Patch.latest;
                    
                    launcher.progressBar?.init({
                        label: Locales.translate('launcher.progress.game.integrity_check') as string,
                        showSpeed: false,
                        showEta: true,
                        showPercents: true,
                        showTotals: false
                    });

                    launcher.progressBar?.show();

                    let current = 0, total = files.length;

                    for (const file of files)
                    {
                        // {"remoteName": "AnAnimeGame_Data/StreamingAssets/AssetBundles/blocks/00/16567284.blk", "md5": "79ab71cfff894edeaaef025ef1152b77", "fileSize": 3232361}
                        const fileCheckInfo = JSON.parse(file) as { remoteName: string, md5: string, fileSize: number };

                        // If this file exists, it's not UnityPlayer.dll,
                        // or if it's UnityPlayer.dll, but patch wasn't applied
                        if (await fs.exists(`${gameDir}/${fileCheckInfo.remoteName}`) &&
                            (!fileCheckInfo.remoteName.includes('UnityPlayer.dll') || !patch.applied))
                        {
                            const process = await Neutralino.os.execCommand(`md5sum "${path.addSlashes(`${gameDir}/${fileCheckInfo.remoteName}`)}"`);
                            const fileHash = (process.stdOut || process.stdErr).split(' ')[0];

                            if (fileHash != fileCheckInfo.md5)
                                mismatchedFiles.push(fileCheckInfo);
                        }

                        launcher.progressBar?.update(++current, total, 1);
                    }

                    Debug.log({
                        function: 'Launcher/States/Integrity',
                        message: mismatchedFiles.length == 0 ?
                            `Checked ${total} files with ${mismatchedFiles.length} mismatches` :
                            [
                                `Checked ${total} files with ${mismatchedFiles.length} mismatches:`,
                                ...mismatchedFiles
                            ]
                    });
                }

                launcher.progressBar?.hide();

                resolve();
            })
            .catch(() => resolve());
    })
}
