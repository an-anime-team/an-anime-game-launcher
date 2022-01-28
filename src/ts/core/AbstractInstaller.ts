import type { Stream as DownloadStream } from '@empathize/framework/dist/network/Downloader';

import { Downloader, Archive } from '../../empathize';
import { DebugThread } from '@empathize/framework/dist/meta/Debug';

import constants from '../Constants';

declare const Neutralino;

export default abstract class Installer
{
    /**
     * The interval in ms between progress event calls
     */
    public downloadProgressInterval: number = 200;

    /**
     * The interval in ms between checking was downloading resumed after pausing
     */
    public downloadPauseInterval: number = 500;

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

    protected downloadStream?: DownloadStream;

    /**
     * @param uri URI to the archive we need to download
     * @param unpackDir path to unpack this archive to
     * 
     * @param alreadyDownloaded specifies whether the archive was already downloaded
     * If true, then URI will be used as a path to the archive. Otherwise stream will download it 
     */
    public constructor(uri: string, unpackDir: string|Promise<string>, alreadyDownloaded: boolean = false)
    {
        const shouldResolve = typeof unpackDir !== 'string';

        const debugThread = new DebugThread('AbstractInstaller', {
            message: {
                'uri': uri,
                'unpack dir': shouldResolve ? '<promise>' : unpackDir,
                'already downloaded': alreadyDownloaded ? 'true' : 'false'
            }
        });

        constants.paths.tempDir.then((tempDir) => {
            const archivePath = alreadyDownloaded ? uri : `${tempDir}/${Downloader.fileFromUri(uri)}`;

            // And then unpack it
            const unpackArchive = () => {
                Promise.resolve(unpackDir)
                    .then((unpackDir) => {
                        if (shouldResolve)
                            debugThread.log(`Resolved unpack dir: ${unpackDir}`);

                        Archive.extract(archivePath, unpackDir).then((stream) => {
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
                    });
            };

            // Download archive
            if (!alreadyDownloaded)
            {
                Downloader.download(uri, archivePath).then((stream) => {
                    this.downloadStream = stream;

                    stream.progressInterval = this.downloadProgressInterval;
                    stream.pauseInterval = this.downloadPauseInterval;

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

                        unpackArchive();
                    });
                });
            }

            else unpackArchive();
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

    /**
     * Pause downloading
     */
    public pauseDownload()
    {
        this.downloadStream?.pause();
    }

    /**
     * Resume downloading
     */
    public resumeDownload()
    {
        this.downloadStream?.resume();
    }
};
