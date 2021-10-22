import GIJSON from './GIJSON';

const https = require('follow-redirects').https;

const fs = require('fs');
const path = require('path');
const os = require('os');
const { spawn, exec } = require('child_process');

type Runner = {
    name: string,          // Runner title which will be showed in the list
    version: string,       // Runner version
    uri: string            // Downloading URI
    archive: 'tar' | 'zip' // Archive type
    folder: string,        // Folder name where it will be downloaded
    makeFolder: boolean,   // Do we need to create folder or it is included in archive
    executable: string     // Path to wine executable inside folder
};

type Config = {
    lang: {
        launcher: 'en-us' | 'ru-ru' | 'fr-fr' | 'id-id' | 'de-de' | 'es-es' | 'pt-pt' | 'th-th' | 'vi-vn' | 'ko-kr' | 'ja-jp' | 'zh-tw' | 'zh-cn',
        voice: 'en-us' | 'ko-kr' | 'ja-jp' | 'zh-cn'
    },
    background: {
        time: string|null,
        file: string|null
    },
    version: string|null,
    patch: {
        version: string|null,
        state: 'testing' | 'stable'
    },
    runner: null | {
        name: string,
        folder: string,
        executable: string
    }
};

export class Genshinlib
{
    public static readonly patchDir: string = path.join(path.dirname(__dirname), 'patch');
    public static readonly patchJson: string = path.join(this.patchDir, 'patch.json');
    public static readonly patchSh = path.join(this.patchDir, 'patch.sh');
    public static readonly patchAntiCrashSh = path.join(this.patchDir, 'patch_anti_logincrash.sh');

    public static readonly launcherDir: string = path.join(os.homedir(), 'genshin-impact-launcher');
    public static readonly launcherJson: string = path.join(this.launcherDir, 'launcher.json');

    public static readonly tmpPatchDir: string = path.join(this.launcherDir, 'gi-on-linux');

    public static readonly prefixDir: string = path.join(this.launcherDir, 'game');
    public static readonly gameDir: string = path.join(this.prefixDir, 'drive_c', 'Program Files', 'Genshin Impact');
    public static readonly runnersDir: string = path.join(this.launcherDir, 'runners');

    protected static readonly versionsUri: string = 'https://sdk-os-static.mihoyo.com/hk4e_global/mdk/launcher/api/resource?key=gcStgarh&launcher_id=10';
    protected static readonly backgroundUri: string = 'https://sdk-os-static.mihoyo.com/hk4e_global/mdk/launcher/api/content?filter_adv=true&launcher_id=10&language=';
    protected static readonly patchUri: string = 'https://notabug.org/Krock/GI-on-Linux/archive/master.zip';
    protected static readonly runnersUri: string = 'https://notabug.org/nobody/an-anime-game-launcher/raw/main/runners.json';

    public static get version(): Config['version']
    {
        return this.getConfig().version;
    }

    public static get lang(): Config['lang']
    {
        return this.getConfig().lang;
    }

    public static getRunners (): Promise<[{ title: string, runners: Runner[] }]>
    {
        return new Promise((resolve, reject) => {
            fetch(this.runnersUri)
                .then(response => response.json())
                .then(runners => resolve(runners));
        });

        // return JSON.parse(fs.readFileSync(path.join(__dirname, '..', '..', 'runners.json')));
    }

    public static getConfig (): Config
    {
        if (!fs.existsSync(this.launcherJson))
            fs.writeFileSync(this.launcherJson, JSON.stringify({
                lang: {
                    launcher: 'en-us',
                    voice: 'en-us'
                },
                background: {
                    time: null,
                    name: null
                },
                version: null,
                patch: null,
                runner: null
            }, null, 4));
        
        return JSON.parse(fs.readFileSync(this.launcherJson));
    }

    public static setConfig (info: Config): Genshinlib
    {
        fs.writeFileSync(this.launcherJson, JSON.stringify(info, null, 4));

        return this;
    }

    public static updateConfig (config: any): Genshinlib
    {
        return this.setConfig({
            ...this.getConfig(),
            ...config
        });
    }

    public static async getData (): Promise<any>
    {
        return new Promise((resolve, reject) => {
            https.get(this.versionsUri, (response: any) => {
                let data = '';
    
                response.on('data', (chunk: any) => data += chunk);

                response.on('end', () => {
                    let jsondata: GIJSON = JSON.parse(data);

                    return jsondata.message === 'OK' ? resolve(jsondata.data) : reject(null);
                });
            }).on('error', (err: Error) => reject(err));
        });
    }

    public static async getBackgroundUri (): Promise<string>
    {
        let background = '';
        
        if (!this.getConfig().background.time || new Date(new Date().setHours(0,0,0,0)).setDate(new Date(new Date().setHours(0,0,0,0)).getDate()).toString() >= this.getConfig().background.time!)
        {
            await fetch(this.backgroundUri + this.lang.launcher)
                .then(res => res.json())
                .then(async resdone => {
                    let prevBackground = this.getConfig().background.file;

                    this.updateConfig({
                        background: {
                            time: new Date(new Date().setHours(0,0,0,0)).setDate(new Date(new Date().setHours(0,0,0,0)).getDate() + 7).toString(),
                            file: resdone.data.adv.background.replace(/.*\//, '')
                        }
                    });

                    if (fs.existsSync(path.join(this.launcherDir, this.getConfig().background.file)))
                        background = path.join(this.launcherDir, this.getConfig().background.file);
                    
                    else
                    {
                        await this.downloadFile(resdone.data.adv.background, path.join(this.launcherDir, this.getConfig().background.file), (current: number, total: number, difference: number) => null).then(() => {
                            !prevBackground ?
                                console.log('No old background found') :
                                fs.unlinkSync(path.join(this.launcherDir, prevBackground));

                            background = path.join(this.launcherDir, this.getConfig().background.file);
                        });
                    };
                });
        }

        else background = path.join(this.launcherDir, this.getConfig().background.file);
        
        return background;
    }

    public static getPatchInfo (): { version: string, state: 'stable' | 'testing' }
    {
        return JSON.parse(fs.readFileSync(this.patchJson));
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
            }).on('error', (err: Error) => reject(err));
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

    public static async untar (tarPath: string, unpackedPath: string, progress: (current: number, total: number, difference: number) => void): Promise<void|Error>
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

                    // TODO: compressedSize?
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

                                progress(current, total, file.uncompressedSize); // compressedSize
                            }
                        });
                    });
                });

                unpackerProcess.on('close', () => resolve());
            });
        });
    }

    // WINEPREFIX='/home/observer/genshin-impact-launcher/wineprefix' winetricks corefonts usetakefocus=n
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
            'Executing load_webdings',
            'Executing load_usetakefocus n'
        ];

        return new Promise((resolve) => {
            let installationProgress = 0;

            let installerProcess = spawn('winetricks', ['corefonts', 'usetakefocus=n'], {
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

    public static patchGame (version: string, onFinish: () => void, onData: (data: string) => void)
    {
        this.downloadFile(this.patchUri, path.join(this.launcherDir, 'krock.zip'), (current: number, total: number, difference: number) => null).then(() => {
            this.unzip(path.join(this.launcherDir, 'krock.zip'), this.launcherDir, (current: number, total: number, difference: number) => null).then(() => {
                // Delete zip file and assign patch directory.
                fs.unlinkSync(path.join(this.launcherDir, 'krock.zip'));

                let patchDir: string = path.join(this.tmpPatchDir, version.replace(/\./g, ''));

                // Patch out the testing phase content from the shell files if active and make sure the shell files are executable.
                exec(`cd ${patchDir} && sed -i '/^echo "If you would like to test this patch, modify this script and remove the line below this one."/,+5d' patch.sh`);
                exec(`cd ${patchDir} && sed -i '/^echo "       necessary afterwards (Friday?). If that's the case, comment the line below."/,+2d' patch_anti_logincrash.sh`);
                exec(`chmod +x ${path.join(patchDir, 'patch.sh')}`);
                exec(`chmod +x ${path.join(patchDir, 'patch_anti_logincrash.sh')}`);

                // Execute the patch file with "yes yes" in the beginning to agree to the choices.
                let patcherProcess = exec(`yes yes | ${path.join(patchDir, 'patch.sh')}`, {
                    cwd: this.gameDir,
                    env: {
                        ...process.env,
                        WINEPREFIX: this.prefixDir
                    }
                });

                patcherProcess.stdout.on('data', (data: string) => onData(data));

                patcherProcess.on('close', () => {
                    // Execute the patch file with "yes" in the beginning to agree to the choice.
                    let patcherAntiCrashProcess = exec(`yes | ${path.join(patchDir, 'patch_anti_logincrash.sh')}`, {
                        cwd: this.gameDir,
                        env: {
                            ...process.env,
                            WINEPREFIX: this.prefixDir
                        }
                    });
    
                    patcherAntiCrashProcess.stdout.on('data', (data: string) => onData(data));
    
                    patcherAntiCrashProcess.on('close', () => {
                        fs.rmSync(this.tmpPatchDir, { recursive: true });

                        onFinish();
                    });
                });
            });
        });
    }

    /*public static applyPatch (onFinish: () => void, onData: (data: string) => void)
    {
        let patcherProcess = spawn('bash', [Genshinlib.patchSh], {
            cwd: Genshinlib.gameDir,
            env: {
                ...process.env,
                WINEPREFIX: Genshinlib.prefixDir
            }
        });

        patcherProcess.stdout.on('data', (data: string) => onData(data));

        patcherProcess.on('close', () => {
            let patcherAntiCrashProcess = spawn('bash', [Genshinlib.patchAntiCrashSh], {
                cwd: Genshinlib.gameDir,
                env: {
                    ...process.env,
                    WINEPREFIX: Genshinlib.prefixDir
                }
            });
    
            patcherAntiCrashProcess.stdout.on('data', (data: string) => onData(data));
    
            patcherAntiCrashProcess.on('close', () => {
                Genshinlib.setConfig({
                    ...Genshinlib.getConfig(),
                    patch: Genshinlib.getPatchInfo()
                });
    
                onFinish();
            });
        });
    }*/
}