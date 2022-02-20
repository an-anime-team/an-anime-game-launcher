import type Launcher from '../../Launcher';

import { Debug, fs, path } from '../../../empathize';

import constants from '../../Constants';
import Patch from "../../Patch";
import Locales from '../Locales';
import Voice from "../../Voice"

declare const Neutralino;

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        const gameDir = await constants.paths.gameDir;

        Neutralino.filesystem.readFile(`${gameDir}/pkg_version`)
            .then(async (files) => {
                // Check Game and Voice Pack Integrity

                let mismatchedFiles = new Array();

                files = files.split(/\r\n|\r|\n/).filter((file) => file != '');

                const InstalledVoices = await Voice.installed;

                for (const voice of InstalledVoices)
                {
                    Neutralino.filesystem.readFile(`${gameDir}/Audio_${Voice.langs[voice.lang]}_pkg_version`)
                        .then(async (vfiles) => {
                            vfiles = vfiles.split(/\r\n|\r|\n/).filter((file) => file != '');
                            files.push(...vfiles);
                        })
                }

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

                        // If the file exists and it's not UnityPlayer.dll
                        // or if it's UnityPlayer.dll but the patch wasn't applied
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
                                `Checked ${total} files with ${mismatchedFiles.length} mismatch(es):`,
                                ...mismatchedFiles
                            ]
                    });
                }

                launcher.progressBar?.hide();

                // Replace mismatched files

                for (const file of mismatchedFiles)
                {
                    const fileInfo = JSON.parse(file) as { remoteName: string, md5: string, fileSize: number };
                }

                resolve();
            })
            .catch(() => resolve());
    })
}
