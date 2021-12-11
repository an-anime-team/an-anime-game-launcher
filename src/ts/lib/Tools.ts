const fs = require('fs');
const path = require('path');
const dns = require('dns');
const { spawn } = require('child_process');

const https = require('follow-redirects').https;
const getPixels = require('get-pixels');

type GitTag = {
    tag: string,
    commit: string
};

type Pixel = {
    x: number,
    y: number,
    color: {
        r: number,
        g: number,
        b: number,
        a: number|null
    }
};

export default class Tools
{
    public static prettifyBytes (bytes: number): string
    {
        const types = [
            {
                name: 'B',
                multiplier: 1
            },
            {
                name: 'KB',
                multiplier: 1024
            },
            {
                name: 'MB',
                multiplier: 1024 * 1024
            },
            {
                name: 'GB',
                multiplier: 1024 * 1024 * 1024
            }
        ].filter(type => type.multiplier < bytes);

        return types.length == 0 ?
            `${bytes} B` :
            `${(bytes / types[types.length - 1].multiplier).toFixed(2)} ${types[types.length - 1].name}`;
    }

    public static getImagePixels (path: string): Promise<Pixel[]>
    {
        return new Promise(resolve => {
            getPixels(path, (err: void, pixels: any) => {
                let response = [], offset = 0;

                const width  = pixels.shape[0],
                      height = pixels.shape[1],
                      depth  = pixels.shape[2];

                for (let i = 0; i < height; ++i)
                    for (let j = 0; j < width; ++j)
                    {
                        response.push({
                            x: j,
                            y: i,
                            color: {
                                r: pixels.data[offset],
                                g: pixels.data[offset + 1],
                                b: pixels.data[offset + 2],
                                a: depth == 4 ? pixels.data[offset + 3] : null
                            }
                        });

                        offset += depth;
                    }

                resolve(response);
            });
        });
    }

    public static async getGitTags (uri: string): Promise<GitTag[]>
    {
        return new Promise(resolve => {
            let git = spawn('git', ['ls-remote', '--tags', uri]),
                tags: GitTag[] = [];

            git.stdout.on('data', (data: string) => {
                data.toString().split(/\r\n|\r|\n/).forEach((line: string) => {
                    if (line != '')
                    {
                        let matches = /^([0-9a-f]+)\trefs\/tags\/(.*)/.exec (line);

                        if (matches)
                            tags.push({
                                tag: matches[2],
                                commit: matches[1]
                            });
                    }
                });
            });

            git.on('close', () => resolve(tags));
        });
    }

    public static async domainAvailable (uri: string): Promise<boolean>
    {
        return new Promise((resolve, reject) => {
            dns.lookup(uri, (error: any, address: string, family: any) => {
                // console.log(`${uri} -> ${address}`);
                
                if (error.toString().startsWith('Error: getaddrinfo ENOTFOUND')) resolve(true);
                else if (error) reject(error);
                else resolve(address == '0.0.0.0');
            });
        });
    }

    public static async downloadFile (uri: string, savePath: string, progress: null|((current: number, total: number, difference: number) => void) = null): Promise<void|Error>
    {
        return new Promise((resolve, reject) => {
            let rangeStart = 0;

            // Part of the file was already downloaded, resume the download
            if (fs.existsSync(savePath))
                rangeStart = fs.statSync(savePath).size;
            
            https.get(uri, {
                headers: {
                    Range: `bytes=${rangeStart}-`
                }
            }, (response: any) => {
                let length = parseInt(response.headers['content-length'], 10) + rangeStart,
                    total  = rangeStart;

                response.on('data', (chunk: any) => {
                    total += chunk.length;

                    if (progress !== null)
                        progress(total, length, chunk.length);

                    fs.appendFileSync(savePath, chunk);
                });

                response.on('end', () => resolve());
            }).on('error', (err: Error) => reject(err));
        });
    }

    public static async unzip (zipPath: string, unpackedPath: string, progress: null|((current: number, total: number, difference: number) => void) = null): Promise<void|Error>
    {
        return new Promise((resolve, reject) => {
            let listenerProcess = spawn('unzip', ['-v', zipPath]),
                filesList = '';

            listenerProcess.stdout.on('data', (data: string) => filesList += data);

            listenerProcess.on('close', () => {
                let files = filesList.split(/\r\n|\r|\n/).slice(3, -3).map(line => {
                    line = line.trim();

                    if (line.slice(-1) == '/')
                        line = line.slice(0, -1);

                    let matches = /^(\d+)  [a-zA-Z\:]+[ ]+(\d+)[ ]+[0-9\-]+% [0-9\-]+ [0-9\:]+ [a-f0-9]{8}  (.+)/.exec(line);

                    if (matches)
                        return {
                            path: matches[3],
                            compressedSize: parseInt(matches[2]),
                            uncompressedSize: parseInt(matches[1])
                        };
                });

                let total = fs.statSync(zipPath)['size'], current = 0;
                let unpackerProcess = spawn('unzip', ['-o', zipPath, '-d', unpackedPath]);

                unpackerProcess.stdout.on('data', (data: string) => {
                    data.toString().split(/\r\n|\r|\n/).forEach(line => {
                        let items = line.split(': ');

                        if (items[1] !== undefined)
                        {
                            items[1] = path.relative(unpackedPath, items[1].trim());

                            files.forEach(file => {
                                if (file?.path == items[1])
                                {
                                    current += file.compressedSize;

                                    if (progress !== null)
                                        progress(current, total, file.compressedSize);
                                }
                            });
                        }
                    });
                });

                unpackerProcess.on('close', () => resolve());
            });
        });
    }

    public static async untar (tarPath: string, unpackedPath: string, progress: null|((current: number, total: number, difference: number) => void) = null): Promise<void|Error>
    {
        return new Promise((resolve, reject) => {
            let listenerProcess = spawn('tar', ['-tvf', tarPath]),
                filesList = '', total = 0;

            listenerProcess.stdout.on('data', (data: string) => filesList += data);

            listenerProcess.on('close', () => {
                let files = filesList.split(/\r\n|\r|\n/).slice(3, -3).map(line => {
                    line = line.trim();

                    if (line.slice(-1) == '/')
                        line = line.slice(0, -1);

                    let matches = /^[dwxr\-]+ [\w/]+[ ]+(\d+) [0-9\-]+ [0-9\:]+ (.+)/.exec(line);

                    if (matches)
                    {
                        total += parseInt(matches[1]);

                        return {
                            path: matches[2],
                            uncompressedSize: parseInt(matches[1])
                        };
                    }
                });

                let current = 0;
                let unpackerProcess = spawn('tar', ['-xvf', tarPath, '-C', unpackedPath]);

                unpackerProcess.stdout.on('data', (data: string) => {
                    data.toString().split(/\r\n|\r|\n/).forEach(line => {
                        line = line.trim();

                        files.forEach(file => {
                            if (file?.path == line)
                            {
                                current += file.uncompressedSize; // compressedSize

                                if (progress !== null)
                                    progress(current, total, file.uncompressedSize); // compressedSize
                            }
                        });
                    });
                });

                unpackerProcess.on('close', () => resolve());
            });
        });
    }
}
