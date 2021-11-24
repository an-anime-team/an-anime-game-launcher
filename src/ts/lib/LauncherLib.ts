import GIJSON from '../types/GIJSON';
import constants from './constants';
import Tools from './Tools';

const fs = require('fs');
const path = require('path');
const os = require('os');
const { spawn, exec } = require('child_process');

const store = require('electron-store');
const https = require('follow-redirects').https;
const got = require('got');

const config = new store ({
    cwd: path.join(os.homedir(), '.local', 'share', 'anime-game-launcher'),
    defaults: {
        lang: {
            launcher: 'en-us',
            voice: {
                installed: null,
                active: 'en-us'
            }
        },
        background: {
            time: null,
            file: null
        },
        version: null, // Installed game version
        patch: null, // Installed patch info ({ version, state } - related game's version and patch's state)
        runner: null, // Selected runner ({ folder, executable })
        rpc: false, // Discord RPC
        playtime: 0, // Number of seconds user spent in game
        hud: 'none', // none / dxvk / mangohud
        shaders: 'none', // none / shader's folder

        // Version of the game we asked about analytics last time,
        // or null if user said don't ask him again
        analytics: '0',
        
        // Environement variables
        env: {
            DXVK_ASYNC: '1',
            WINEESYNC: '1', // ESync

            // FidelityFX Super Resolution
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

export default class LauncherLib
{
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

        return new Promise(resolve => fs.readFile(path.join(constants.appDir, 'public', 'runners.json'), (err: any, data: string) => resolve(JSON.parse(data))));
    }

    public static getDXVKs (): Promise<DXVK[]>
    {
        /*return new Promise((resolve, reject) => {
            fetch(this.dxvksUri)
                .then(response => response.json())
                .then(dxvks => resolve(dxvks));
        });*/

        return new Promise(resolve => fs.readFile(path.join(constants.appDir, 'public', 'dxvks.json'), (err: any, data: string) => resolve(JSON.parse(data))));
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
            https.get(constants.versionsUri, (response: any) => {
                let data = '';
    
                response.on('data', (chunk: any) => data += chunk);

                response.on('end', () => {
                    const jsonData: GIJSON = JSON.parse(data);

                    return jsonData.message === 'OK' ?
                        resolve(jsonData.data) : reject(null);
                });
            }).on('error', (err: Error) => reject(err));
        });
    }

    public static async getBackgroundUri (): Promise<string>
    {
        let background = '';
        
        if (!this.getConfig('background.time') || Date.now() > this.getConfig('background.time')!)
        {
            await fetch(constants.backgroundUri + this.getConfig('lang.launcher'))
                .then(res => res.json())
                .then(async resdone => {
                    let prevBackground = this.getConfig('background.file');

                    this.updateConfig('background.time', Date.now() + 7 * 24 * 60 * 60 * 1000); // 7 days
                    this.updateConfig('background.file', resdone.data.adv.background.replace(/.*\//, ''));

                    if (fs.existsSync(path.join(constants.launcherDir, this.getConfig('background.file'))))
                        background = path.join(constants.launcherDir, this.getConfig('background.file'));
                    
                    else
                    {
                        await Tools.downloadFile(resdone.data.adv.background, path.join(constants.launcherDir, this.getConfig('background.file')), () => null).then(() => {
                            !prevBackground ?
                                console.log('No old background found') :
                                fs.unlinkSync(path.join(constants.launcherDir, prevBackground));

                            background = path.join(constants.launcherDir, this.getConfig('background.file'));
                        });
                    };
                });
        }

        else background = path.join(constants.launcherDir, this.getConfig('background.file'));
        
        return background;
    }

    public static async getPatchInfo (): Promise<{ version: string, state: 'testing' | 'stable' }>
    {
        return new Promise(resolve => {
            this.getData().then(async (data) => {
                let gameLatest: string = data.game.latest.version;

                got(`${constants.uri.patch}/raw/master/${gameLatest.replaceAll('.', '')}/patch.sh`)
                    .then((patch: any) => {
                        /**
                         * [game version]/patch.sh file exists
                         * so it's testing or stable version
                         */
                        got(`${constants.uri.patch}/raw/master/${gameLatest.replaceAll('.', '')}/patch_files/unityplayer_patch.vcdiff`)
                            .then(() => {
                                /**
                                 * [game version]/patch_files/unityplayer_patch
                                 * exists so it's testing or stable
                                 */
                                resolve({
                                    version: gameLatest,
                                    state: patch.body.includes('#echo "If you would like to test this patch, modify this script and remove the line below this one."') ?
                                        'stable' : 'testing'
                                });
                            })
                            .catch(() => {
                                /**
                                 * [game version]/patch_files/unityplayer_patch
                                 * doesn't exist so it's just a preparation
                                 * 
                                 * TODO: add preparation state
                                 */
                                resolve({
                                    version: data.game.diffs[0].version,
                                    state: 'stable'
                                });
                            });
                    })
                    .catch(() => {
                        /**
                         * Otherwise it's definitely preparation
                         */
                        resolve({
                            version: data.game.diffs[0].version,
                            state: 'stable'
                        });
                    });
            });
        });
    }

    public static isTelemetryDisabled (domainNum: number = 0): Promise<boolean>
    {
        return new Promise((resolve, reject) => {
            Tools.domainAvailable(constants.uri.telemetry[domainNum]).then(async (status) => {
                if (constants.uri.telemetry[++domainNum] !== undefined)
                    status &&= await this.isTelemetryDisabled(domainNum);

                resolve(status);
            }).catch(reject);
        });
    }

    // WINEPREFIX='...../wineprefix' winetricks corefonts usetakefocus=n
    public static async installPrefix (prefixpath: string, progress: (output: string, current: number, total: number) => void): Promise<void>
    {
        const installationSteps = [
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
        this.getPatchInfo().then(pathInfo => {
            Tools.downloadFile(constants.patchUri, path.join(constants.launcherDir, 'patch.zip')).then(() => {
                Tools.unzip(path.join(constants.launcherDir, 'patch.zip'), constants.launcherDir).then(() => {
                    // Delete zip file and assign patch directory.
                    fs.unlinkSync(path.join(constants.launcherDir, 'patch.zip'));

                    const patchDir = path.join(constants.launcherDir, 'dawn', pathInfo.version.replaceAll('.', ''));

                    // Patch out the testing phase content from the shell files if active and make sure the shell files are executable.
                    exec(`cd ${patchDir} && sed -i '/^echo "If you would like to test this patch, modify this script and remove the line below this one."/,+5d' patch.sh`);
                    exec(`cd ${patchDir} && sed -i '/^echo "       necessary afterwards (Friday?). If that's the case, comment the line below."/,+2d' patch_anti_logincrash.sh`);
                    exec(`chmod +x ${path.join(patchDir, 'patch.sh')}`);
                    exec(`chmod +x ${path.join(patchDir, 'patch_anti_logincrash.sh')}`);

                    // Execute the patch file with "yes yes" in the beginning to agree to the choices.
                    let patcherProcess = exec(`yes yes | ${path.join(patchDir, 'patch.sh')}`, {
                        cwd: constants.gameDir,
                        env: {
                            ...process.env,
                            WINEPREFIX: constants.prefixDir
                        }
                    });

                    patcherProcess.stdout.on('data', (data: string) => onData(data));

                    patcherProcess.on('close', () => {
                        // Make sure that launcher.bat exists if not run patch.sh again.
                        if (!fs.existsSync(path.join(constants.gameDir, 'launcher.bat')))
                            exec(`yes yes | ${path.join(patchDir, 'patch.sh')}`, {
                                cwd: constants.gameDir,
                                env: {
                                    ...process.env,
                                    WINEPREFIX: constants.prefixDir
                                }
                            });

                        // Execute the patch file with "yes" in the beginning to agree to the choice.
                        let patcherAntiCrashProcess = exec(`yes | ${path.join(patchDir, 'patch_anti_logincrash.sh')}`, {
                            cwd: constants.gameDir,
                            env: {
                                ...process.env,
                                WINEPREFIX: constants.prefixDir
                            }
                        });
        
                        patcherAntiCrashProcess.stdout.on('data', (data: string) => onData(data));
        
                        patcherAntiCrashProcess.on('close', () => {
                            this.updateConfig('patch.version', pathInfo.version);
                            this.updateConfig('patch.state', pathInfo.state);

                            fs.rmSync(path.join(constants.launcherDir, 'dawn'), { recursive: true });

                            onFinish();
                        });
                    });
                });
            })
        });
    }
}
