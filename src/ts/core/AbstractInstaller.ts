import constants from '../Constants';
import Downloader from './Downloader';
import Archive from './Archive';
import { DebugThread } from './Debug';

declare const Neutralino;

export default abstract class Installer
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

    public constructor(uri: string, unpackDir: string|Promise<string>)
    {
        const debugThread = new DebugThread('AbstractInstaller', {
            message: {
                'uri': uri,
                'unpack dir': typeof unpackDir === 'string' ? unpackDir : '<promise>'
            }
        });

        constants.paths.launcherDir.then((launcherDir) => {
            const archivePath = `${launcherDir}/${Downloader.fileFromUri(uri)}`;

            // Download archive
            Downloader.download(uri, archivePath).then((stream) => {
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
                    const unpackArchive = (unpackDir: string) => {
                        Archive.unpack(archivePath, unpackDir).then((stream) => {
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

                                Neutralino.filesystem.removeFile(archivePath);

                                debugThread.log('Installation finished');
    
                                if (this.onUnpackFinish)
                                    this.onUnpackFinish();
                            });
                        });
                    };

                    const shouldResolve = typeof unpackDir !== 'string';

                    Promise.resolve(unpackDir)
                        .then((unpackDir) => {
                            if (shouldResolve)
                                debugThread.log(`Resolved unpack dir: ${unpackDir}`);

                            unpackArchive(unpackDir);
                        });
                });
            });
        });
    }

    /**
     * Specify event that will be called after download has begun
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
     * Specify event that will be called after extraction has begun
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
     * Specify event that will be called every [this.downloadProgressInterval] ms while downloading
     * 
     * @param callback
     */
    public downloadProgress(callback: (current: number, total: number, difference: number) => void)
    {
        this.onDownloadProgress = callback;
    }

    /**
     * Specify event that will be called every [this.unpackProgressInterval] ms while extracting
     * 
     * @param callback
     */
    public unpackProgress(callback: (current: number, total: number, difference: number) => void)
    {
        this.onUnpackProgress = callback;
    }

    /**
     * Specify event that will be called after download has finished
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
     * Specify event that will be called after extraction has finished
     * 
     * @param callback
     */
    public unpackFinish(callback: () => void)
    {
        this.onUnpackFinish = callback;

        if (this.unpackFinished)
            callback();
    }
};
