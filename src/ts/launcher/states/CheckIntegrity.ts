import type Launcher from '../../Launcher';
import type { PatchInfo } from '../../types/Patch';

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

class FilesVerifier
{
    protected files: string[];
    protected gameDir: string;
    protected patch: PatchInfo;
    protected launcher: Launcher;
    protected debugThread: DebugThread;

    protected current: number = 0;
    protected total: number;

    protected mismatches: FileInfo[] = [];

    protected paused: boolean = false;

    protected onProgress: null|((current: number, total: number) => void) = null;
    protected onFinished: null|((mismatches: FileInfo[]) => void) = null;

    protected ignoringFiles = [
        'crashreport.exe',
        'upload_crash.exe'
    ];

    public constructor(files: string[], gameDir: string, patch: PatchInfo, launcher: Launcher, debugThread: DebugThread)
    {
        this.files = files;
        this.gameDir = gameDir;
        this.patch = patch;
        this.launcher = launcher;
        this.debugThread = debugThread;
        this.total = files.length;

        // Show pause/resume button
        launcher.state!.pauseButton.style['display'] = 'block';

        launcher.state!.pauseButton.onclick = () => {
            if (this.paused)
            {
                launcher.state!.pauseButton.textContent = Locales.translate('launcher.progress.pause');

                this.resume();
            }

            else
            {
                launcher.state!.pauseButton.textContent = Locales.translate('launcher.progress.resume');

                this.pause();
            }
        };

        this.process();
    }

    protected async process()
    {
        const file = this.files[this.current++];

        try
        {
            // {"remoteName": "AnAnimeGame_Data/StreamingAssets/AssetBundles/blocks/00/16567284.blk", "md5": "79ab71cfff894edeaaef025ef1152b77", "fileSize": 3232361}
            const fileCheckInfo: FileInfo = JSON.parse(file);

            let skipping = false;

            for (const ignoringFile of this.ignoringFiles)
                if (fileCheckInfo.remoteName.includes(ignoringFile))
                {
                    skipping = true;

                    break;
                }

            if (!skipping)
            {
                if (!await fs.exists(`${this.gameDir}/${fileCheckInfo.remoteName}`))
                {
                    this.mismatches.push(fileCheckInfo);

                    this.debugThread.log({
                        message: [
                            'File is missing',
                            `[path] ${fileCheckInfo.remoteName}`,
                            `[hash] ${fileCheckInfo.md5}`
                        ]
                    });
                }

                else if (!fileCheckInfo.remoteName.includes('UnityPlayer.dll') || !this.patch.applied)
                {
                    const process = await Neutralino.os.execCommand(`md5sum "${path.addSlashes(`${this.gameDir}/${fileCheckInfo.remoteName}`)}"`);
                    const fileHash = (process.stdOut || process.stdErr).split(' ')[0];

                    if (fileHash != fileCheckInfo.md5)
                    {
                        this.mismatches.push(fileCheckInfo);

                        this.debugThread.log({
                            message: [
                                'Wrong file hash found',
                                `[path] ${fileCheckInfo.remoteName}`,
                                `[hash] ${fileHash}`,
                                `[remote hash] ${fileCheckInfo.md5}`
                            ]
                        });
                    }
                }
            }
        }

        catch {}

        if (this.onProgress)
            this.onProgress(this.current, this.total);

        if (this.current == this.total)
        {
            if (this.onFinished)
                this.onFinished(this.mismatches);

            // Hide pause/resume button
            this.launcher.state!.pauseButton.style['display'] = 'none';
        }

        else if (!this.paused)
            this.process();
    }

    public pause()
    {
        this.paused = true;
    }

    public resume()
    {
        this.paused = false;

        this.process();
    }

    public progress(callback: (current: number, total: number) => void)
    {
        this.onProgress = callback;
    }

    public finish(callback: (mismatches: FileInfo[]) => void): void
    {
        this.onFinished = callback;
    }
}

class FilesRepairer
{
    protected mismatches: FileInfo[];
    protected launcher: Launcher;
    protected debugThread: DebugThread;

    protected current: number = 0;
    protected total: number;

    protected paused: boolean = false;

    protected onProgress: null|((current: number, total: number) => void) = null;
    protected onFinished: null|(() => void) = null;

    public constructor(mismatches: FileInfo[], launcher: Launcher, debugThread: DebugThread)
    {
        this.mismatches = mismatches;
        this.launcher = launcher;
        this.debugThread = debugThread;
        this.total = mismatches.length;

        // Show pause/resume button
        launcher.state!.pauseButton.style['display'] = 'block';

        launcher.state!.pauseButton.onclick = () => {
            if (this.paused)
            {
                launcher.state!.pauseButton.textContent = Locales.translate('launcher.progress.pause');

                this.resume();
            }

            else
            {
                launcher.state!.pauseButton.textContent = Locales.translate('launcher.progress.resume');

                this.pause();
            }
        };

        this.process();
    }

    /**
     * Try to the repair game's file
     */
    protected repairFile(fileInfo: FileInfo): Promise<boolean>
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

    protected async process()
    {
        if (this.mismatches[this.current] === undefined)
            return;

        const fileInfo = this.mismatches[this.current++];

        await this.repairFile(fileInfo).then((success) => {
            if (!success)
                this.debugThread.log(`Repair failed: ${fileInfo.remoteName}`);
        });

        if (this.onProgress)
            this.onProgress(this.current, this.total);

        if (this.current == this.total)
        {
            if (this.onFinished)
                this.onFinished();

            // Hide pause/resume button
            this.launcher.state!.pauseButton.style['display'] = 'none';
        }

        else if (!this.paused)
            this.process();
    }

    public pause()
    {
        this.paused = true;
    }

    public resume()
    {
        this.paused = false;

        this.process();
    }

    public progress(callback: (current: number, total: number) => void)
    {
        this.onProgress = callback;
    }

    public finish(callback: () => void): void
    {
        this.onFinished = callback;
    }
}

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        const gameDir = await constants.paths.gameDir;

        const debugThread = new DebugThread('Launcher/State/CheckIntegrity', 'Checking files integrity...');

        // Check Game and Voice Pack integrity
        Neutralino.filesystem.readFile(`${gameDir}/pkg_version`)
            .then(async (files) => {
                files = files.split(/\r\n|\r|\n/);

                // Add voice packages integrity info
                for (const voice of await Voice.installed)
                    Neutralino.filesystem.readFile(`${gameDir}/Audio_${Voice.langs[voice.lang]}_pkg_version`)
                        .then(async (voiceFiles) => {
                            files.push(...voiceFiles.split(/\r\n|\r|\n/));
                        });

                files = files
                    .map((file) => file.trim())
                    .filter((file: string) => file.length > 30);

                if (files.length > 0)
                {
                    launcher.progressBar?.init({
                        label: Locales.translate<string>('launcher.progress.game.integrity_check'),
                        showSpeed: false,
                        showEta: true,
                        showPercents: true,
                        showTotals: false
                    });

                    launcher.progressBar?.show();

                    debugThread.log(`Verifying ${files.length} files...`);

                    // Find broken files
                    const verifier = new FilesVerifier(files, gameDir, await Patch.latest, launcher, debugThread);

                    verifier.progress((current, total) => launcher.progressBar?.update(current, total, 1));

                    verifier.finish(async (mismatches) => {
                        debugThread.log({
                            message: mismatches.length == 0 ?
                                `Checked ${files.length} files with ${mismatches.length} mismatches` :
                                [
                                    `Checked ${files.length} files with ${mismatches.length} mismatch(es):`,
                                    ...mismatches.map((mismatch) => `[${mismatch.md5}] ${mismatch.remoteName}`)
                                ]
                        });
    
                        // And then repair them
                        if (mismatches.length > 0)
                        {
                            launcher.progressBar?.init({
                                label: Locales.translate<string>('launcher.progress.game.download_mismatch_files'),
                                showSpeed: false,
                                showEta: false,
                                showPercents: true,
                                showTotals: false
                            });

                            const repairer = new FilesRepairer(mismatches, launcher, debugThread);

                            repairer.progress((current, total) => launcher.progressBar?.update(current, total, 1));

                            repairer.finish(() => {
                                launcher.progressBar?.hide();

                                resolve();
                            });
                        }

                        else resolve();
                    });
                }

                else resolve();
            })
            .catch(() => {
                debugThread.log('No pkg_version file provided');

                resolve();
            });
    });
}
