class Stream
{
    /**
     * The interval in ms between progress event calls
     */
    public progressInterval: number = 200;

    protected uri: string;
    protected total: number;
    protected previous: number = 0;

    protected onProgress?: (current: number, total: number, difference: number) => void;
    protected onFinish?: () => void;

    protected finished: boolean = false;

    public constructor(uri: string, output: string, total: number)
    {
        this.uri = uri;
        this.total = total;

        // @ts-expect-error
        Neutralino.os.execCommand(`curl -s -L -N -o "${output}" "${uri}"`, {
            background: true
        });

        const updateProgress = () => {
            // @ts-expect-error
            Neutralino.filesystem.getStats(output).then((stats) => {
                if (this.onProgress)
                    this.onProgress(stats.size, this.total, this.previous - stats.size);

                this.previous = stats.size;

                if (stats.size >= this.total)
                {
                    this.finished = true;

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
     * Specify event that will be called every [this.progressInterval] ms during the file downloading
     * 
     * @param callback
     */
    public progress(callback: (current: number, total: number, difference: number) => void)
    {
        this.onProgress = callback;
    }

    /**
     * Specify event that will be called after the file will be downloaded
     * 
     * @param callback
     */
    public finish(callback: () => void)
    {
        this.onFinish = callback;

        if (this.finished)
            callback();
    }
}

export default class Downloader
{
    /**
     * Download file
     * 
     * @param uri
     * @param output
     * 
     * @returns Promise<Stream>
     */
    public static async download(uri: string, output: string|null = null): Promise<Stream>
    {
        return new Promise(async (resolve) => {
            // @ts-expect-error
            let statsRaw = await Neutralino.os.execCommand(`curl -s -I -L "${uri}"`);

            if (statsRaw.stdOut == '')
                statsRaw = statsRaw.stdErr;

            else statsRaw = statsRaw.stdOut;

            const length = parseInt(/content-length: ([\d]+)/i.exec(statsRaw)[1]);

            resolve(new Stream(uri, output ?? this.fileFromUri(uri), length));
        });
    }

    protected static fileFromUri(uri: string): string
    {
        const file = uri.split('/').pop().split('#')[0].split('?')[0];

        if (file === '')
            return 'index.html';

        else if (`https://${file}` != uri && `http://${file}` != uri)
            return file;

        else 'index.html';
    }
}

export { Stream };
