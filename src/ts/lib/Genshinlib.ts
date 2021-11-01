import GIJSON from '../types/GIJSON';
import { Tools } from './Tools';

const store = require('electron-store');
const https = require('follow-redirects').https;

const fs = require('fs');
const path = require('path');
const os = require('os');
const { spawn, exec } = require('child_process');
const dns = require('dns');

const config = new store ({
    cwd: path.join(os.homedir(), '.local', 'share', 'anime-game-launcher'),
    defaults: {
        lang: {
            launcher: 'en-us',
            voice: 'en-us'
        },
        background: {
            time: null,
            file: null
        },
        version: null, // Installed game version
        patch: null, // Installed patch info ({ version, state } - related game's version and patch's state)
        runner: null, // Selected runner ({ folder, executable })
        rpc: false,

        // Version of the game we asked about analytics last time,
        // or null if user said don't ask him again
        analytics: '0',
        
        // Environement variables
        env: {
            DXVK_ASYNC: '1',
            WINEESYNC: '1',
            WINE_FULLSCREEN_FSR: '1',
            WINE_FULLSCREEN_FSR_STRENGTH: '3'
        }
    }
});

type Runner = {
    name: string,          // Runner title which will be showed in the list
    version: string,       // Runner version
    uri: string            // Downloading URI
    archive: 'tar' | 'zip' // Archive type
    folder: string,        // Folder name where it will be downloaded
    makeFolder: boolean,   // Do we need to create folder or it is included in archive
    executable: string     // Path to wine executable inside folder
};

type DXVK = {
    version: string,
    uri: string
};

export class Genshinlib
{
    public static readonly launcherDir: string = path.join(os.homedir(), '.local', 'share', 'anime-game-launcher');

    public static readonly prefixDir: string = path.join(this.launcherDir, 'game'); // TODO: rename every game's name entry to something like below
    public static readonly gameDir: string = path.join(this.prefixDir, 'drive_c', 'Program Files', Buffer.from('R2Vuc2hpbiBJbXBhY3Q=', 'base64').toString('utf-8'));
    public static readonly runnersDir: string = path.join(this.launcherDir, 'runners');
    public static readonly dxvksDir: string = path.join(this.launcherDir, 'dxvks');

    protected static readonly versionsUri: string = 'https://sdk-os-static.mihoyo.com/hk4e_global/mdk/launcher/api/resource?key=gcStgarh&launcher_id=10';
    protected static readonly backgroundUri: string = 'https://sdk-os-static.mihoyo.com/hk4e_global/mdk/launcher/api/content?filter_adv=true&launcher_id=10&language=';
    protected static readonly patchUri: string = 'https://notabug.org/Krock/GI-on-Linux/archive/master.zip';
    protected static readonly runnersUri: string = 'https://notabug.org/nobody/an-anime-game-launcher/raw/main/runners.json';
    protected static readonly dxvksUri: string = 'https://notabug.org/nobody/an-anime-game-launcher/raw/main/dxvks.json';

    public static get version(): string|null
    {
        return this.getConfig('version');
    }

    public static getRunners (): Promise<[{ title: string, runners: Runner[] }]>
    {
        /*return new Promise((resolve, reject) => {
            fetch(this.runnersUri)
                .then(response => response.json())
                .then(runners => resolve(runners));
        });*/

        return new Promise(resolve => resolve(JSON.parse(fs.readFileSync(path.join(path.dirname(__dirname), '..', 'runners.json')))));
    }

    public static getDXVKs (): Promise<DXVK[]>
    {
        /*return new Promise((resolve, reject) => {
            fetch(this.dxvksUri)
                .then(response => response.json())
                .then(dxvks => resolve(dxvks));
        });*/

        return new Promise(resolve => resolve(JSON.parse(fs.readFileSync(path.join(path.dirname(__dirname), '..', 'dxvks.json')))));
    }

    public static getConfig (property: string|null = null): any
    {
        if (property === null)
            return config;

        return config.get(property);
    }

    public static updateConfig (property: string, value: string|boolean|null|number)
    {
        return config.set(property, value);
    }

    public static deleteConfig (property: string)
    {
        return config.delete(property);
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
        
        if (!this.getConfig('background.time') || new Date(new Date().setHours(0,0,0,0)).setDate(new Date(new Date().setHours(0,0,0,0)).getDate()).toString() >= this.getConfig('background.time')!)
        {
            await fetch(this.backgroundUri + this.getConfig('lang.launcher'))
                .then(res => res.json())
                .then(async resdone => {
                    let prevBackground = this.getConfig('background.file');

                    this.updateConfig('background.time', new Date(new Date().setHours(0,0,0,0)).setDate(new Date(new Date().setHours(0,0,0,0)).getDate() + 7).toString());
                    this.updateConfig('background.file', resdone.data.adv.background.replace(/.*\//, ''));

                    if (fs.existsSync(path.join(this.launcherDir, this.getConfig('background.file'))))
                        background = path.join(this.launcherDir, this.getConfig('background.file'));
                    
                    else
                    {
                        await Tools.downloadFile(resdone.data.adv.background, path.join(this.launcherDir, this.getConfig('background.file')), (current: number, total: number, difference: number) => null).then(() => {
                            !prevBackground ?
                                console.log('No old background found') :
                                fs.unlinkSync(path.join(this.launcherDir, prevBackground));

                            background = path.join(this.launcherDir, this.getConfig('background.file'));
                        });
                    };
                });
        }

        else background = path.join(this.launcherDir, this.getConfig('background.file'));
        
        return background;
    }

    public static async getPatchInfo (): Promise<{ version: string, state: 'stable' | 'testing' }>
    {
        return new Promise(resolve => {
            this.getData().then(data => {
                let gameLatest: string = data.game.latest.version;

                fetch(`https://notabug.org/Krock/GI-on-Linux/raw/master/${gameLatest.replaceAll('.', '')}/patch.sh`)
                    .then(response => response.text())
                    .then((patch: string) => {
                        // patch.sh exists so patch in testing, stable or it's just a preparation
                        fetch(`https://notabug.org/Krock/GI-on-Linux/raw/master/${gameLatest.replaceAll('.', '')}/patch_files/unityplayer_patch.vcdiff`)
                            .then(response => response.text())
                            .then((unityPatch: string) => {
                                // unityplayer_patch exists so it's testing or stable
                                resolve({
                                    version: gameLatest,
                                    state: patch.includes('#echo "If you would like to test this patch, modify this script and remove the line below this one."') ?
                                        'stable' : 'testing'
                                });
                            })
                            .catch(() => {
                                // unityplayer_patch doesn't exist so it's just a preparation
                                // TODO: add preparation state
                                resolve({
                                    version: data.game.diffs[0].version,
                                    state: 'stable'
                                });
                            });
                    })
                    .catch(() => {
                        // patch.sh doesn't exist so patch is not available
                        resolve({
                            version: data.game.diffs[0].version,
                            state: 'stable'
                        });
                    });
            });
        });
    }

    /**
     * 0.0.0.0 log-upload-os.mihoyo.com
     * 0.0.0.0 overseauspider.yuanshen.com
     */
    public static isTelemetryDisabled (): Promise<boolean>
    {
        return new Promise((resolve, reject) => {
            dns.lookup('log-upload-os.mihoyo.com', (error: any, address: string, family: any) => {
                if (error)
                    reject(error);
                
                else
                {
                    if (address != '0.0.0.0')
                        resolve(false);

                    else
                    {
                        dns.lookup('log-upload-os.mihoyo.com', (error: any, address: string, family: any) => {
                            if (error)
                                reject(error);
                            
                            else resolve(address == '0.0.0.0');
                        });
                    }
                }
            });
        });
    }

    // WINEPREFIX='/home/observer/genshin-impact-launcher/wineprefix' winetricks corefonts usetakefocus=n
    public static async installPrefix (prefixpath: string, progress: (output: string, current: number, total: number) => void): Promise<void>
    {
        let installationSteps = [
            // corefonts
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

            // usetakefocus=n (fullscreen input issues fix)
            'Executing load_usetakefocus n'
        ];

        return new Promise((resolve) => {
            let installationProgress = 0;

            let installerProcess = spawn('winetricks', ['corefonts', 'usetakefocus=n'], {
                env: {
                    ...process.env,
                    WINEPREFIX: prefixpath
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

    public static patchGame (onFinish: () => void, onData: (data: string) => void)
    {
        Genshinlib.getPatchInfo().then(pathInfo => {
            Tools.downloadFile(this.patchUri, path.join(this.launcherDir, 'patch.zip'), (current: number, total: number, difference: number) => null).then(() => {
                Tools.unzip(path.join(this.launcherDir, 'patch.zip'), this.launcherDir, (current: number, total: number, difference: number) => null).then(() => {
                    // Delete zip file and assign patch directory.
                    fs.unlinkSync(path.join(this.launcherDir, 'patch.zip'));

                    let patchDir = path.join(this.launcherDir, 'gi-on-linux', pathInfo.version.replaceAll('.', ''));

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
                        // Make sure that launcher.bat exists if not run patch.sh again.
                        if (!path.join(this.gameDir, 'launcher.bat'))
                            exec(`yes yes | ${path.join(patchDir, 'patch.sh')}`, {
                                cwd: this.gameDir,
                                env: {
                                    ...process.env,
                                    WINEPREFIX: this.prefixDir
                                }
                            });

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
                            Genshinlib.updateConfig('patch.version', pathInfo.version);
                            Genshinlib.updateConfig('patch.state', pathInfo.state);

                            fs.rmSync(path.join(this.launcherDir, 'gi-on-linux'), { recursive: true });

                            onFinish();
                        });
                    });
                });
            })
        });
    }
}
