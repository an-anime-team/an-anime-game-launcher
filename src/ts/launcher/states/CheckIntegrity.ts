import type Launcher from '../../Launcher';
import type { PatchInfo } from '../../types/Patch';
import type { VoiceLang } from '../../types/Voice';

import { fs, Downloader, fetch } from '../../../empathize';
import { DebugThread } from '@empathize/framework/dist/meta/Debug';

import constants from '../../Constants';
import Patch from '../../Patch';
import Locales from '../Locales';
import Voice from '../../Voice';
import Game from '../../Game';
import md5 from '../../core/md5';

export type FileInfo = {
    remoteName: string;
    md5: string;
    fileSize: number;
};

export class FilesVerifier
{
    protected files: FileInfo[];
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
        'upload_crash.exe',
        'vulkan-1.dll'
    ];

    protected ignoringPatchedFiles = [
        'UnityPlayer.dll',
        'xlua.dll'
    ];

    public constructor(files: FileInfo[], gameDir: string, patch: PatchInfo, launcher: Launcher, debugThread: DebugThread)
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

    /**
     * Returns same as `finish` event
     */
    public process(): Promise<FileInfo[]>
    {
        return new Promise(async (resolve) => {
            const file = this.files[this.current++];

            try
            {
                let skipping = false;

                for (const ignoringFile of this.ignoringFiles)
                    if (file.remoteName.includes(ignoringFile))
                    {
                        skipping = true;

                        break;
                    }

                if (!skipping && this.patch.applied)
                    for (const ignore of this.ignoringPatchedFiles)
                        if (file.remoteName.includes(ignore))
                            skipping = true;

                if (!skipping)
                {
                    // If the file doesn't exist - we should download it
                    if (!await fs.exists(`${this.gameDir}/${file.remoteName}`))
                    {
                        this.mismatches.push(file);

                        this.debugThread.log({
                            message: [
                                'File is missing',
                                `[path] ${file.remoteName}`,
                                `[hash] ${file.md5}`
                            ]
                        });
                    }

                    else
                    {
                        const fileHash = await md5(`${this.gameDir}/${file.remoteName}`);

                        if (fileHash != file.md5)
                        {
                            this.mismatches.push(file);

                            this.debugThread.log({
                                message: [
                                    'Wrong file hash found',
                                    `[path] ${file.remoteName}`,
                                    `[hash] ${fileHash}`,
                                    `[remote hash] ${file.md5}`
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

                resolve(this.mismatches);
            }

            else if (!this.paused)
                this.process();
        });
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

    /**
     * Get list of integrity files directly from the API
     * 
     * @param includeVoice if false, then no voice packages will be included.
     * If true, then all the packages will be included.
     * Otherwise list of packages that should be included
     */
    public static getIntegrityFiles(includeVoice: VoiceLang[]|boolean = false): Promise<FileInfo[]>
    {
        return new Promise(async (resolve) => {
            const decompressedUri = (await Game.getLatestData()).game.latest.decompressed_path;

            const getFiles = (name: string): Promise<FileInfo[]> => {
                return new Promise(async (resolve) => {
                    const filesRaw = await (await fetch(`${decompressedUri}/${name}`)).body();
                    let files: FileInfo[] = [];

                    for (const line of filesRaw.split(/\r\n|\r|\n/))
                        if (line.length > 30)
                            files.push(JSON.parse(line.trim()));

                    resolve(files);
                });
            };

            let files: FileInfo[] = await getFiles('pkg_version');

            let voices: VoiceLang[] = includeVoice === true ? Object.keys(Voice.langs) as VoiceLang[] :
                includeVoice === false ? [] : includeVoice;

            for (const lang of voices)
                files = [...files, ...await getFiles(`Audio_${Voice.langs[lang]}_pkg_version`)];

            resolve(files);
        });
    }
}

export class FilesRepairer
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
     * Try to repair game's file
     */
    protected repairFile(fileInfo: FileInfo): Promise<boolean>
    {
        return new Promise(async (resolve) => {
            const gameDir = await constants.paths.gameDir;
            const fileUri = `${(await Game.getLatestData()).game.latest.decompressed_path}/${fileInfo.remoteName}`;

            Downloader.download(fileUri, `${gameDir}/${fileInfo.remoteName}.new`).then((stream) => {
                stream.finish(async () => {
                    if (await md5(`${gameDir}/${fileInfo.remoteName}.new`) == fileInfo.md5)
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

    /**
     * Try to download game file
     */
    public static downloadFile(file: string): Promise<boolean>
    {
        return new Promise(async (resolve) => {
            const gameDir = await constants.paths.gameDir;
            const fileUri = `${(await Game.getLatestData()).game.latest.decompressed_path}/${file}`;

            fetch(fileUri).then((header) => {
                if (!header.ok)
                    resolve(false);

                else
                {
                    Downloader.download(fileUri, `${gameDir}/${file}`).then((stream) => {
                        stream.finish(() => resolve(true));
                    });
                }
            });
        });
    }

    public process(): Promise<void>
    {
        return new Promise(async (resolve) => {
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

                resolve();
            }

            else if (!this.paused)
                this.process();
        });
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
        const debugThread = new DebugThread('Launcher/State/CheckIntegrity', 'Checking files integrity...');

        const gameDir = await constants.paths.gameDir;
        const files = await FilesVerifier.getIntegrityFiles((await Voice.installed).map((lang) => lang.lang));

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
                    showEta: true,
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
    });
}
