import type Launcher from '../../Launcher';
import { Debug, Notification, fs, path } from '../../../empathize';

import constants from '../../Constants';
import Patch from "../../Patch";
import Locales from '../Locales';

declare const Neutralino;

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        const gameDir = await constants.paths.gameDir;
        Neutralino.filesystem.readFile(`${gameDir}/pkg_version`)
            .then(async (files) => {
                let checkErrors = 0;

                files = files.split(/\r\n|\r|\n/).filter((file) => file != '');

                const patch = await Patch.latest;

                if (files.length > 0)
                {
                    launcher.progressBar?.init({
                        label: Locales.translate('launcher.progress.game.integrity_check') as string,
                        showSpeed: false,
                        showEta: true,
                        showPercents: true,
                        showTotals: false
                    });

                    launcher.progressBar?.show();

                    let current = 0, total = files.length;
                    const mismatchedFiles = new Array();

                    for (const file of files)
                    {
                        // {"remoteName": "GenshinImpact_Data/StreamingAssets/AssetBundles/blocks/00/16567284.blk", "md5": "79ab71cfff894edeaaef025ef1152b77", "fileSize": 3232361}
                        const fileCheckInfo = JSON.parse(file) as { remoteName: string, md5: string, fileSize: number };

                        if (await fs.exists(`${gameDir}/${fileCheckInfo.remoteName}`))
                        {
                            const process = await Neutralino.os.execCommand(`md5sum "${path.addSlashes(`${gameDir}/${fileCheckInfo.remoteName}`)}" | awk '{ print $1 }'`);
                            const md5 = process.stdOut || process.stdErr;

                            if (md5.substring(0, md5.length - 1) != fileCheckInfo.md5)
                            {
                                if (fileCheckInfo.remoteName.includes('UnityPlayer.dll') && patch.applied)
                                    console.log('UnityPlayer patched. Skipping check...'); 
                                else
                                {
                                    ++checkErrors;
                                    mismatchedFiles.push(fileCheckInfo);
                                }
                            }

                        }

                        launcher.progressBar?.update(++current, total, 1);
                    }

                    Debug.log({
                        function: 'Launcher/States/Integrity',
                        message: `Checked ${total} files${checkErrors > 0 ? `, there were ${checkErrors} mismatch(es):\n${JSON.stringify(mismatchedFiles, null, 4)}` : ', there were no mismatches'}`
                    });

                    mismatchedFiles.length = 0;
                }

                launcher.progressBar?.hide();
                resolve();
            })
            .catch(() => resolve());
    })
}