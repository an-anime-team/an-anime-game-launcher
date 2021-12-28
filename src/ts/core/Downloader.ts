import { DebugThread } from './Debug';
import fetch from './Fetch';

declare const Neutralino;

class Stream
{
    protected _id: number = -1;

    /**
     * ID of the curl process
     */
    public get id(): number
    {
        return this._id;
    }

    /**
     * The interval in ms between progress event calls
     */
    public progressInterval: number = 200;

    protected uri: string;
    protected total: number;
    protected previous: number = 0;

    protected onStart?: () => void;
    protected onProgress?: (current: number, total: number, difference: number) => void;
    protected onFinish?: () => void;

    protected started: boolean = false;
    protected finished: boolean = false;

    public constructor(uri: string, output: string, total: number)
    {
        this.uri = uri;
        this.total = total;
        this.started = true;

        const debugThread = new DebugThread('Downloader/Stream', {
            message: {
                'uri': uri,
                'output file': output,
                'total size': total
            }
        });

        if (this.onStart)
            this.onStart();

        const command = `curl -s -L -N -o "${output}" "${uri}"`;

        Neutralino.os.execCommand(command, {
            background: true
        }).then((result) => {
            this._id = result.pid;
        });

        debugThread.log(`Downloading started with command: ${command}`);

        const updateProgress = () => {
            Neutralino.filesystem.getStats(output).then((stats) => {
                if (this.onProgress)
                    this.onProgress(stats.size, this.total, stats.size - this.previous);

                this.previous = stats.size;

                if (stats.size >= this.total)
                {
                    this.finished = true;

                    debugThread.log('Downloading finished');

                    if (this.onFinish)
                        this.onFinish();
                }

                if (!this.finished)
                    setTimeout(updateProgress, this.progressInterval);
            }).catch(() => {
                if (!this.finished)
                    setTimeout(updateProgress, this.progressInterval);
            });
        };

        setTimeout(updateProgress, this.progressInterval);
    }

    /**
     * Specify event that will be called when the download gets started
     * 
     * @param callback
     */
    public start(callback: () => void)
    {
        this.onStart = callback;

        if (this.started)
            callback();
    }

    /**
     * Specify event that will be called every [this.progressInterval] ms while the file is downloading
     * 
     * @param callback
     */
    public progress(callback: (current: number, total: number, difference: number) => void)
    {
        this.onProgress = callback;
    }

    /**
     * Specify event that will be called after the file is downloaded
     * 
     * @param callback
     */
    public finish(callback: () => void)
    {
        this.onFinish = callback;

        if (this.finished)
            callback();
    }

    /**
     * Close downloading stream
     */
    public close(forced: boolean = false)
    {
        Neutralino.os.execCommand(`kill ${forced ? '-9' : '-15'} ${this._id}`);
    }
}

export default class Downloader
{
    protected static streams: Stream[] = [];

    /**
     * Download file
     * 
     * @param uri file's uri to download
     * @param output relative or absolute path to the file to save it as
     * 
     * @returns downloading stream
     */
    public static async download(uri: string, output: string|null = null): Promise<Stream>
    {
        return new Promise(async (resolve) => {
            fetch(uri).then((response) => {
                const stream = new Stream(uri, output ?? this.fileFromUri(uri), response.length!);

                this.streams.push(stream);

                resolve(stream);
            });
        });
    }

    /**
     * Close every open downloading stream
     */
    public static closeStreams(forced: boolean = false)
    {
        this.streams.forEach((stream) => {
            stream.close(forced);
        });
    }

    /**
     * Get a file name from the URI
     */
    public static fileFromUri(uri: string): string
    {
        const file = uri.split('/').pop()!.split('#')[0].split('?')[0];

        if (file === '')
            return 'index.html';

        else if (`https://${file}` != uri && `http://${file}` != uri)
            return file;

        else return 'index.html';
    }
};

export { Stream };
