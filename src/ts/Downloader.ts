class Stream
{
    protected uri: string;
    protected total: number|null;

    protected onProgress?: (current: number, total: number) => void;
    protected onFinish?: () => void;

    protected finished: boolean = false;

    public constructor(uri: string, output: string, total: number|null = null)
    {
        this.uri = uri;
        this.total = total;

        // @ts-expect-error
        Neutralino.os.execCommand(`wget -O "${output}" -nv "${uri}"`).then(() => {
            this.finished = true;

            this.onFinish();
        });

        if (total !== null)
        {
            const updateProgress = () => {
                // @ts-expect-error
                Neutralino.filesystem.getStats(output).then((stats) => {
                    if (this.onProgress)
                        this.onProgress(stats.size, this.total);

                    if (!this.finished)
                        setTimeout(updateProgress, 100);
                });
            };

            updateProgress();
        }
    }

    /**
     * Specify event that will be called every 100 ms during the file downloading
     * 
     * @param callable
     */
    public progress(callable: (current: number, total: number) => void)
    {
        this.onProgress = callable;
    }

    /**
     * Specify event that will be called after the file will be downloaded
     * 
     * @param callable
     */
    public finish(callable: () => void)
    {
        this.onFinish = callable;

        if (this.finished)
            callable();
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
    public static async download(uri: string, output: string): Promise<Stream>
    {
        return new Promise(async (resolve) => {
            // @ts-expect-error
            let statsRaw = await Neutralino.os.execCommand(`wget --spider "${uri}"`);

            if (statsRaw.stdOut == '')
                statsRaw = statsRaw.stdErr;

            else statsRaw = statsRaw.stdOut;

            let length: any = /Length: ([\d]+)/.exec(statsRaw)[1] ?? null;

            if (length !== null)
                length = parseInt(length);

            resolve(new Stream(uri, output, length));
        });
    }
}

export { Stream };
