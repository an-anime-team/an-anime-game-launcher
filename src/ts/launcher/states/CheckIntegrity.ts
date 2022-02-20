import type Launcher from '../../Launcher';

import { fs, path, Downloader } from '../../../empathize';
import { DebugThread } from '@empathize/framework/dist/meta/Debug';

import constants from '../../Constants';
import Patch from '../../Patch';
import Locales from '../Locales';
import Voice from '../../Voice';
import Game from '../../Game';

declare const Neutralino;

type FileInfo = {
    remoteName: string;
    md5: string;
    fileSize: number;
};

/**
 * Try to the repair game's file
 */
function repairFile(fileInfo: FileInfo): Promise<boolean>
{
    return new Promise(async (resolve) => {
        const gameDir = await constants.paths.gameDir;
        const fileUri = `${(await Game.getLatestData()).game.latest.decompressed_path}/${fileInfo.remoteName}`;

        Downloader.download(fileUri, `${gameDir}/${fileInfo.remoteName}.new`).then((stream) => {
            stream.finish(async () => {
                const process = await Neutralino.os.execCommand(`md5sum "${path.addSlashes(`${gameDir}/${fileInfo.remoteName}.new`)}"`);

                if ((process.stdOut || process.stdErr).split(' ')[0] == fileInfo.md5)
                {
                    await fs.remove(`${gameDir}/${fileInfo.remoteName}`);
                    await fs.move(`${gameDir}/${fileInfo.remoteName}.new`, `${gameDir}/${fileInfo.remoteName}`);

                    resolve(true);
                }

                else
                {
                    await fs.remove(`${gameDir}/${fileInfo.remoteName}.new`);

                    resolve(false);
                }
            });
        });
    });
}

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        const gameDir = await constants.paths.gameDir;

        const debugThread = new DebugThread('State/IntegrityCheck', 'Checking files integrity...');

        // Check Game and Voice Pack integrity
        Neutralino.filesystem.readFile(`${gameDir}/pkg_version`)
            .then(async (files) => {
                files = files.split(/\r\n|\r|\n/).filter((file) => file != '');

                // Add voice packages integrity info
                for (const voice of await Voice.installed)
                    Neutralino.filesystem.readFile(`${gameDir}/Audio_${Voice.langs[voice.lang]}_pkg_version`)
                        .then(async (voiceFiles) => {
                            files.push(...voiceFiles.split(/\r\n|\r|\n/).filter((file) => file != ''));
                        });

                if (files.length > 0)
                {
                    const patch = await Patch.latest;
                    
                    launcher.progressBar?.init({
                        label: Locales.translate<string>('launcher.progress.game.integrity_check'),
                        showSpeed: false,
                        showEta: true,
                        showPercents: true,
                        showTotals: false
                    });

                    launcher.progressBar?.show();

                    let current = 0, total = files.length;
                    let mismatchedFiles = new Array();

                    debugThread.log(`Verifying ${total} files...`);

                    for (const file of files)
                    {
                        // {"remoteName": "AnAnimeGame_Data/StreamingAssets/AssetBundles/blocks/00/16567284.blk", "md5": "79ab71cfff894edeaaef025ef1152b77", "fileSize": 3232361}
                        const fileCheckInfo: FileInfo = JSON.parse(file);

                        // If the file exists and it's not UnityPlayer.dll
                        // or if it's UnityPlayer.dll but the patch wasn't applied
                        if (await fs.exists(`${gameDir}/${fileCheckInfo.remoteName}`) &&
                            (!fileCheckInfo.remoteName.includes('UnityPlayer.dll') || !patch.applied))
                        {
                            const process = await Neutralino.os.execCommand(`md5sum "${path.addSlashes(`${gameDir}/${fileCheckInfo.remoteName}`)}"`);
                            const fileHash = (process.stdOut || process.stdErr).split(' ')[0];

                            if (fileHash != fileCheckInfo.md5)
                            {
                                mismatchedFiles.push(fileCheckInfo);

                                debugThread.log({
                                    message: [
                                        'Wrong file hash found',
                                        `[path] ${fileCheckInfo.remoteName}`,
                                        `[hash] ${fileHash}`,
                                        `[remote hash] ${fileCheckInfo.md5}`
                                    ]
                                });
                            }
                        }

                        launcher.progressBar?.update(++current, total, 1);
                    }

                    debugThread.log({
                        message: mismatchedFiles.length == 0 ?
                            `Checked ${total} files with ${mismatchedFiles.length} mismatches` :
                            [
                                `Checked ${total} files with ${mismatchedFiles.length} mismatch(es):`,
                                ...mismatchedFiles.map((mismatch) => `[${mismatch.md5}] ${mismatch.remoteName}`)
                            ]
                    });

                    // Replace mismatched files

                    if (mismatchedFiles.length > 0)
                    {
                        launcher.progressBar?.init({
                            label: Locales.translate<string>('launcher.progress.game.download_mismatch_files'),
                            showSpeed: false,
                            showEta: false,
                            showPercents: true,
                            showTotals: false
                        });

                        let current = 0, total = mismatchedFiles.length;

                        for (const fileInfo of mismatchedFiles)
                        {
                            await repairFile(fileInfo).then((success) => {
                                if (!success)
                                    debugThread.log(`Repair failed: ${fileInfo.remoteName}`);
                            });

                            launcher.progressBar?.update(++current, total, 1);
                        }
                    }

                    launcher.progressBar?.hide();
                }

                resolve();
            })
            .catch(() => {
                debugThread.log('No pkg_version file provided');

                resolve();
            });
    });
}
