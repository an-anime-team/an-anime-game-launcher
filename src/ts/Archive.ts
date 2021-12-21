import type {
    ArchiveType,
    Size,
    File,
    ArchiveInfo
} from './types/Archive';

declare const Neutralino;
declare const NL_CWD;

class Stream
{
    /**
     * The interval in ms between progress event calls
     */
    public progressInterval: number = 500;

    protected path: string;
    protected unpackDir: string|null;
    protected unpacked: number = 0;

    protected archive: ArchiveInfo;

    protected onStart?: () => void;
    protected onProgress?: (current: number, total: number, difference: number) => void;
    protected onFinish?: () => void;
    protected onError?: () => void;

    protected started: boolean = false;
    protected finished: boolean = false;
    protected throwedError: boolean = false;

    /**
     * @param path path to archive
     * @param unpackDir directory to unpack
     */
    public constructor(path: string, unpackDir: string|null = null)
    {
        this.path = path;
        this.unpackDir = unpackDir;
        this.started = true;

        if (this.onStart)
            this.onStart();

        Archive.getInfo(path).then((info) => {
            if (info === null)
            {
                this.throwedError = true;

                if (this.onError)
                    this.onError();
            }

            else
            {
                this.archive = info;

                const command = {
                    tar: `tar -xvf "${path}"${unpackDir ? ` -C "${unpackDir}"` : ''}`,
                    zip: `unzip -o "${path}"${unpackDir ? ` -d "${unpackDir}"` : ''}`
                }[this.archive.type];

                let remainedFiles = this.archive.files;
                
                const baseDir = unpackDir ?? NL_CWD;

                Neutralino.os.execCommand(command, {
                    background: true
                });

                const updateProgress = async () => {
                    let difference: number = 0;

                    remainedFiles.forEach((file) => {
                        if (file.path != '#unpacked#')
                        {
                            Neutralino.filesystem.getStats(`${baseDir}/${file.path}`)
                                .then(() => {
                                    this.unpacked += file.size.uncompressed;
                                    difference += file.size.uncompressed;

                                    file.path = '#unpacked#';
                                })
                                .catch(() => {});
                        }
                    });

                    remainedFiles = remainedFiles.filter((file) => file.path != '#unpacked#');

                    if (this.onProgress)
                        this.onProgress(this.unpacked, this.archive.size.uncompressed, difference);

                    if (this.unpacked >= this.archive.size.uncompressed)
                    {
                        this.finished = true;
    
                        if (this.onFinish)
                            this.onFinish();
                    }
    
                    if (!this.finished)
                        setTimeout(updateProgress, this.progressInterval);
                };
        
                setTimeout(updateProgress, this.progressInterval);
            }
        });
    }

    /**
     * Specify event that will be called when the extraction has started
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
     * Specify event that will be called every [this.progressInterval] ms while extracting the archive
     * 
     * @param callback
     */
    public progress(callback: (current: number, total: number, difference: number) => void)
    {
        this.onProgress = callback;
    }

    /**
     * Specify event that will be called after the archive has been extracted
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
     * Specify event that will be called if archive can't be extracted
     * 
     * @param callback
     */
    public error(callback: () => void)
    {
        this.onError = callback;

        if (this.throwedError)
            callback();
    }
}

export default class Archive
{
    /**
     * Get type of archive
     * 
     * @param path path to archive
     * @returns supported archive type or null
     */
    public static getType(path: string): ArchiveType|null
    {
        if (path.substring(path.length - 4) == '.zip')
            return 'zip';

        else if (path.substring(path.length - 7, path.length - 2) == '.tar.')
            return 'tar';

        else return null;
    }

    /**
     * Get archive info
     * 
     * @param path path to archive
     * @returns null if the archive has unsupported type. Otherwise - archive info
     */
    public static getInfo(path: string): Promise<ArchiveInfo|null>
    {
        return new Promise(async (resolve) => {
            let archive: ArchiveInfo = {
                size: {
                    compressed: null,
                    uncompressed: null
                },
                type: this.getType(path),
                files: []
            };

            switch (archive.type)
            {
                case 'tar':
                    const tarOutput = await Neutralino.os.execCommand(`tar -tvf "${path}"`);

                    for (const match of tarOutput.stdOut.matchAll(/^[dwxr\-]+ [\w/]+[ ]+(\d+) [0-9\-]+ [0-9\:]+ (.+)/gm))
                    {
                        let fileSize = parseInt(match[1]);

                        archive.size.uncompressed += fileSize;

                        archive.files.push({
                            path: match[2],
                            size: {
                                compressed: null,
                                uncompressed: fileSize
                            }
                        });
                    }

                    resolve(archive);

                    break;

                case 'zip':
                    const zipOutput = await Neutralino.os.execCommand(`unzip -v "${path}"`);

                    for (const match of zipOutput.stdOut.matchAll(/^(\d+)  [a-zA-Z\:]+[ ]+(\d+)[ ]+[0-9\-]+% [0-9\-]+ [0-9\:]+ [a-f0-9]{8}  (.+)/gm))
                    {
                        let uncompressedSize = parseInt(match[1]),
                            compressedSize = parseInt(match[2]);

                        archive.size.compressed   += compressedSize;
                        archive.size.uncompressed += uncompressedSize;

                        archive.files.push({
                            path: match[3],
                            size: {
                                compressed: compressedSize,
                                uncompressed: uncompressedSize
                            }
                        });
                    }

                    resolve(archive);

                    break;

                default:
                    resolve(null);

                    break;
            }
        });
    }

    /**
     * Unpack archive
     * 
     * @param path path to archive
     * @param unpackDir directory to unpack
     */
    public static unpack(path: string, unpackDir: string|null = null): Promise<Stream>
    {
        return new Promise((resolve) => resolve(new Stream(path, unpackDir)));
    }
}

export { Stream };

export type {
    ArchiveType,
    File,
    Size,
    ArchiveInfo
};
