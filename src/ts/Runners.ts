import {
    Runner,
    RunnerFamily
} from './types/Runners';

declare const Neutralino;

import Constants from './Constants';
import Downloader from './Downloader';
import Archive from './Archive';

class Stream
{
    /**
     * The interval in ms between progress event calls
     */
    public downloadProgressInterval: number = 200;

    /**
     * The interval in ms between progress event calls
     */
    public unpackProgressInterval: number = 500;

    protected onDownloadStart?: () => void;
    protected onUnpackStart?: () => void;

    protected onDownloadProgress?: (current: number, total: number, difference: number) => void;
    protected onUnpackProgress?: (current: number, total: number, difference: number) => void;

    protected onDownloadFinish?: () => void;
    protected onUnpackFinish?: () => void;

    protected downloadStarted: boolean = false;
    protected unpackStarted: boolean = false;

    protected downloadFinished: boolean = false;
    protected unpackFinished: boolean = false;

    public constructor(runner: Runner)
    {
        Constants.paths.launcher.then((launcherDir) => {
            const archivePath = `${launcherDir}/${Downloader.fileFromUri(runner.uri)}`;

            // Download archive
            Downloader.download(runner.uri, archivePath).then((stream) => {
                stream.progressInterval = this.downloadProgressInterval;

                stream.start(() => {
                    this.downloadStarted = true;

                    if (this.onDownloadStart)
                        this.onDownloadStart();
                });

                stream.progress((current, total, difference) => {
                    if (this.onDownloadProgress)
                        this.onDownloadProgress(current, total, difference);
                });

                stream.finish(() => {
                    this.downloadFinished = true;

                    if (this.onDownloadFinish)
                        this.onDownloadFinish();

                    // And then unpack it
                    Constants.paths.runners.then((runners) => {
                        Archive.unpack(archivePath, runners).then((stream) => {
                            stream.progressInterval = this.unpackProgressInterval;

                            stream.start(() => {
                                this.unpackStarted = true;
            
                                if (this.onUnpackStart)
                                    this.onUnpackStart();
                            });

                            stream.progress((current, total, difference) => {
                                if (this.onUnpackProgress)
                                    this.onUnpackProgress(current, total, difference);
                            });

                            stream.finish(() => {
                                this.unpackFinished = true;

                                if (this.onUnpackFinish)
                                    this.onUnpackFinish();
                            });
                        });
                    });
                });
            });
        });
    }

    /**
     * Specify event that will be called after the runner will begin downloading
     * 
     * @param callback
     */
    public downloadStart(callback: () => void)
    {
        this.onDownloadStart = callback;

        if (this.downloadStarted)
            callback();
    }

    /**
     * Specify event that will be called after the runner will begin unpacking
     * 
     * @param callback
     */
    public unpackStart(callback: () => void)
    {
        this.onUnpackStart = callback;

        if (this.unpackStarted)
            callback();
    }

    /**
     * Specify event that will be called every [this.downloadProgressInterval] ms during the runner downloading
     * 
     * @param callback
     */
    public downloadProgress(callback: (current: number, total: number, difference: number) => void)
    {
        this.onDownloadProgress = callback;
    }

    /**
     * Specify event that will be called every [this.unpackProgressInterval] ms during the runner unpacking
     * 
     * @param callback
     */
    public unpackProgress(callback: (current: number, total: number, difference: number) => void)
    {
        this.onUnpackProgress = callback;
    }

    /**
     * Specify event that will be called after the runner will be downloaded
     * 
     * @param callback
     */
    public downloadFinish(callback: () => void)
    {
        this.onDownloadFinish = callback;

        if (this.downloadFinished)
            callback();
    }

    /**
     * Specify event that will be called after the runner will be unpacked
     * 
     * @param callback
     */
    public unpackFinish(callback: () => void)
    {
        this.onUnpackFinish = callback;

        if (this.unpackFinished)
            callback();
    }
}

class Runners
{
    /**
     * Get runners list
     * 
     * @returns Promise<Runner[]>
     */
    public static get(): Promise<RunnerFamily[]>
    {
        return new Promise((resolve) => {
            Constants.paths.runners.then(async (runnersDir: string) => {
                let list: RunnerFamily[] = JSON.parse(await Neutralino.filesystem.readFile(`${Constants.paths.app}/public/runners.json`));

                const installed: { entry: string, type: string }[] = await Neutralino.filesystem.readDirectory(runnersDir);

                let runners: RunnerFamily[] = [];

                list.forEach((family) => {
                    let newFamily: RunnerFamily = {
                        title: family.title,
                        runners: []
                    };

                    family.runners.forEach((runner) => {
                        let inst = false;

                        for (let dir of installed)
                            inst ||= dir.entry == runner.name;

                        newFamily.runners.push({
                            ...runner,

                            installed: inst
                        });
                    });

                    runners.push(newFamily);
                });

                resolve(runners);
            });
        });
    }

    public static download(runner: Runner|Runner['name']): Promise<null|Stream>
    {
        return new Promise(async (resolve) => {
            // If we provided runner property as a name of the runner
            // then we should find this runner and call this method from it
            if (typeof runner == 'string')
            {
                let foundRunner = null;

                (await this.get()).forEach((family) => {
                    family.runners.forEach((familyRunner) => {
                        if (familyRunner.name == runner)
                            foundRunner = familyRunner;
                    });
                });

                resolve(foundRunner === null ? null : new Stream(foundRunner));
            }

            // Otherwise we can use runner.uri and so on to download runner
            else resolve(new Stream(runner));
        });
    }
}

export default Runners;

export { Stream };

export type {
    Runner,
    RunnerFamily
};
