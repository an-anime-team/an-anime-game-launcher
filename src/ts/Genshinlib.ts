const https = require('https');
const fs = require('fs');
const path = require('path');
const os = require('os');
const { spawn } = require('child_process');

type Config = {
    lang: {
        launcher: 'en-us' | 'ru-ru',
        voice: 'en-us' | 'ru-ru'
    },
    version: string|null
};

export class Genshinlib
{
    public static readonly launcherDir: string = path.join(os.homedir(), 'genshin-impact-launcher');
    public static readonly launcherJson: string = path.join(this.launcherDir, 'launcher.json');

    public static readonly prefixDir: string = path.join(this.launcherDir, 'game');
    public static readonly gameDir: string = path.join(this.prefixDir, 'drive_c', 'Program Files', 'Genshin Impact');

    protected static uri: string = 'https://sdk-os-static.mihoyo.com/hk4e_global/mdk/launcher/api/resource?key=gcStgarh&launcher_id=10';

    public static get version(): string|null
    {
        return this.getLauncherInfo().version;
    }

    public static get lang(): { launcher: string, voice: string }
    {
        return this.getLauncherInfo().lang;
    }

    public static getLauncherInfo (): Config
    {
        if (!fs.existsSync(this.launcherJson))
            fs.writeFileSync(this.launcherJson, JSON.stringify({
                lang: {
                    launcher: 'en-us',
                    voice: 'en-us'
                },
                version: null
            }));
        
        return JSON.parse(fs.readFileSync(this.launcherJson));
    }

    public static setLauncherInfo (info: Config): Genshinlib
    {
        fs.writeFileSync(this.launcherJson, JSON.stringify(info));

        return this;
    }

    public static async getData (): Promise<any>
    {
        return new Promise((resolve, reject) => {
            https.get(this.uri, (response: any) => {
                let data = '';
    
                response.on('data', (chunk: any) => data += chunk);

                response.on('end', () => {
                    data = JSON.parse(data);

                    // @ts-expect-error
                    return data.message === 'OK' ? resolve(data.data) : reject(null);
                });
            }).on('error', (err: Error) => {
                reject(err);
            });
        });
    }

    public static getBackgroundUri (): string
    {
        return path.join(__dirname, '..', 'images', 'backgrounds', this.lang.launcher + '.png');
    }

    public static async downloadFile (uri: string, savePath: string, progress: (current: number, total: number, difference: number) => void): Promise<void|Error>
    {
        return new Promise((resolve, reject) => {
            https.get(uri, (response: any) => {
                let length = parseInt(response.headers['content-length'], 10),
                    total  = 0;

                response.on('data', (chunk: any) => {
                    total += chunk.length;

                    progress(total, length, chunk.length);

                    fs.appendFileSync(savePath, chunk);
                });

                response.on('end', () => resolve());
            }).on('error', (err: Error) => {
                reject(err);
            });
        });
    }

    public static async unzip (zipPath: string, unpackedPath: string, progress: (current: number, total: number, difference: number) => void): Promise<void|Error>
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
                
                    return {
                        // @ts-expect-error
                        path: matches[3],

                        // @ts-expect-error
                        compressedSize: parseInt(matches[2]),

                        // @ts-expect-error
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
                                if (file.path == items[1])
                                {
                                    current += file.compressedSize;

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

    // WINEPREFIX='/home/observer/genshin-impact-launcher/wineprefix' winetricks corefonts
    public static async installPrefix (path: string, progress: (output: string, current: number, total: number) => void): Promise<void>
    {
        let installationSteps = [
            'Executing w_do_call corefonts',
            'Executing load_corefonts',
            'Executing load_andale',
            'Executing load_arial',
            'Executing load_comicsans',
            'Executing load_courier',
            'Executing load_georgia',
            'Executing load_impact',
            'Executing load_times',
            'Executing load_trebuchet',
            'Executing load_verdana',
            'Executing load_webdings'
        ];

        return new Promise((resolve) => {
            let installationProgress = 0;

            let installerProcess = spawn('winetricks', ['corefonts'], {
                env: {
                    ...process.env,
                    WINEPREFIX: path
                }
            });

            installerProcess.stdout.on('data', (data: string) => {
                let str = data.toString();

                for (let i = 0; i < installationSteps.length; ++i)
                    if (str.includes(installationSteps[i]))
                    {
                        installationProgress = i + 1;

                        break;
                    }

                progress(str, installationProgress, installationSteps.length);
            });
    
            installerProcess.on('close', () => resolve());
        });
    }

    public static isPrefixInstalled (prefixPath: string): boolean
    {
        return fs.existsSync(path.join(prefixPath, 'drive_c'));
    }
}