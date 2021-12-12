import GIJSON from '../types/GIJSON';
import constants from './constants';
import Tools from './Tools';

const fs = require('fs');
const path = require('path');
const os = require('os');
const { spawn, exec } = require('child_process');

// TODO: This is an instrument-surprise which will be used later :)
// const crypto = require('crypto');

const store = require('electron-store');
const https = require('follow-redirects').https;
const got = require('got');
const commandExists = require('command-exists').sync;

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
        prefix: path.join(os.homedir(), '.local', 'share', 'anime-game-launcher', 'game'), // Default Prefix
        patch: null, // Installed patch info ({ version, state } - related game's version and patch's state)
        runner: null, // Selected runner ({ folder, executable })
        rpc: false, // Discord RPC
        rpcsettings: {
            launcher: 'Preparing to launch',
            ingame: {
                details: 'In-Game',
                state: null,
                elapsed: true
            }
        },
        playtime: 0, // Number of seconds user spent in game
        hud: 'none', // none / dxvk / mangohud
        shaders: 'none', // none / shader's folder
        gamemode: false, // GameMode
        gpu: 'default', // GPU
        autodelete_dxvk_logs: false, // Auto-delete DXVK logs
        theme: 'system', // light / dark / system

        // Lists filters
        lists_filters: {
            wine: true, // show only recommendable wine versions
            dxvk: true  // show only recommendable dxvk versions
        },

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
    executable: string,    // Path to wine executable inside folder
    recommendable: boolean // Is this runner recommendable to use
};

type DXVK = {
    version: string,
    uri: string,
    recommendable: boolean
};

export default class LauncherLib
{
    public static get version(): string|null
    {
        return this.getConfig('version');
    }

    /*public static getKeypair(): { public: string, private: string }
    {
        const keypairFile = path.join(constants.launcherDir, 'keypair.json');

        if (fs.existsSync(keypairFile))
            return JSON.parse(fs.readFileSync(keypairFile));
        
        else
        {
            const { privateKey, publicKey } = crypto.generateKeyPairSync('ec', {
                namedCurve: 'secp224r1'
            });

            fs.writeFileSync(keypairFile, JSON.stringify({
                public: publicKey.export({
                    type: 'spki',
                    format: 'der'
                }).toString('base64'),

                private: privateKey.export({
                    type: 'sec1',
                    format: 'der'
                }).toString('base64')
            }));

            return {
                public: publicKey,
                private: privateKey
            };
        }
    }*/

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
        let background = this.getConfig('background.file') ?
            path.join(constants.launcherDir, this.getConfig('background.file')) : null;
        
        if (!this.getConfig('background.time') || Date.now() > this.getConfig('background.time')! || background === null || !fs.existsSync(background))
        {
            await fetch(constants.backgroundUri + this.getConfig('lang.launcher'))
                .then(res => res.json())
                .then(async resdone => {
                    const prevBackground = this.getConfig('background.file');

                    this.updateConfig('background.time', Date.now() + 7 * 24 * 60 * 60 * 1000); // 7 days
                    this.updateConfig('background.file', resdone.data.adv.background.replace(/.*\//, ''));

                    background = path.join(constants.launcherDir, this.getConfig('background.file'));

                    if (!fs.existsSync(background))
                    {
                        await Tools.downloadFile(resdone.data.adv.background, background).then(() => {
                            if (prevBackground && prevBackground != this.getConfig('background.file'))
                                fs.unlinkSync(path.join(constants.launcherDir, prevBackground));
                        });
                    };
                });
        }
        
        return background;
    }

    /**
     * Get patch's state and version from the repository
     * @returns information about the patch, or null if repository is not available
     */
    public static async getPatchInfo(source: 'origin' | 'additional' = 'origin'): Promise<{ version: string, state: 'testing' | 'stable' }|null>
    {
        return new Promise(resolve => {
            this.getData().then(async (data) => {
                let gameLatest: string = data.game.latest.version;

                got(`${constants.uri.patch[source]}/raw/master/${gameLatest.replaceAll('.', '')}/patch.sh`, {
                    timeout: {
                        request: 3000
                    }
                }).then((patch: any) => {
                        /**
                         * [game version]/patch.sh file exists
                         * so it's testing or stable version
                         */
                        got(`${constants.uri.patch[source]}/raw/master/${gameLatest.replaceAll('.', '')}/patch_files/unityplayer_patch.vcdiff`, {
                            timeout: {
                                request: 3000
                            }
                        }).then(() => {
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
                            .catch((error: Error) => {
                                console.error(error);

                                /**
                                 * Source is not responding
                                 */
                                if (error.message.includes('Timeout awaiting'))
                                {
                                    // If it was a notabug - then we can try to use
                                    // Maroxy's patch's repo mirror
                                    if (source === 'origin')
                                        this.getPatchInfo('additional').then(resolve);
        
                                    else resolve(null);
                                }
                                
                                /**
                                 * [game version]/patch_files/unityplayer_patch
                                 * doesn't exist so it's just a preparation
                                 * 
                                 * TODO: add preparation state
                                 */
                                else
                                {
                                    resolve({
                                        version: data.game.diffs[0].version,
                                        state: 'stable'
                                    });
                                }
                            });
                    })
                    .catch((error: Error) => {
                        console.error(error);

                        /**
                         * Source is not responding
                         */
                        if (error.message.includes('Timeout awaiting'))
                        {
                            // If it was a notabug - then we can try to use
                            // Maroxy's patch's repo mirror
                            if (source === 'origin')
                                this.getPatchInfo('additional').then(resolve);

                            else resolve(null);
                        }

                        /**
                         * Otherwise it's definitely preparation
                         */
                        else
                        {
                            resolve({
                                version: data.game.diffs[0].version,
                                state: 'stable'
                            });
                        }
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

    /**
     * @param dataLocation path to the [An Anime Game]_Data folder
     */
    public static getGameVersion(dataLocation: string): string|null
    {
        const persistentPath = path.join(dataLocation, 'Persistent');
        const globalGameManagersPath = path.join(dataLocation, 'globalgamemanagers');

        if (fs.existsSync(persistentPath))
            return fs.readFileSync(path.join(persistentPath, 'ScriptVersion'), { encoding: 'UTF-8' }).toString();
        
        else if (fs.existsSync(globalGameManagersPath))
        {
            const config = fs.readFileSync(globalGameManagersPath, { encoding: 'ascii' });
            const version = /([1-9]+\.[0-9]+\.[0-9]+)_[\d]+_[\d]+/.exec(config);
            
            return version !== null ? version[1] : null;
        }
        
        else return null;
    }

    public static async getWinetricks (): Promise<string>
    {
        return new Promise((resolve) => {
            if (!fs.existsSync(path.join(constants.launcherDir, 'winetricks.sh')))
            {
                Tools.downloadFile(constants.uri.winetricks, path.join(constants.launcherDir, 'winetricks.sh')).then(() => {
                    resolve(path.join(constants.launcherDir, 'winetricks.sh'));
                });
            }
            else
            {
                resolve(path.join(constants.launcherDir, 'winetricks.sh'));
            }
        });
    }

    // WINEPREFIX='...../wineprefix' winetricks corefonts usetakefocus=n
    public static async installPrefix (prefixPath: string, progress: (output: string, current: number, total: number) => void): Promise<void>
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
            LauncherLib.getWinetricks().then((winetricksSh) => {
                let installationProgress = 0;

                let env: any = {
                    ...process.env,
                    WINEPREFIX: prefixPath
                };

                if (!commandExists('wine') && LauncherLib.getConfig('runner') !== null)
                {
                    env['WINE'] = path.join(
                        constants.runnersDir,
                        LauncherLib.getConfig('runner.folder'),
                        LauncherLib.getConfig('runner.executable')
                    );

                    env['WINESERVER'] = path.join(path.dirname(env['WINE']), 'wineserver');

                    if (!fs.existsSync(env['WINE']))
                        console.error(`Patcher supposed to use ${LauncherLib.getConfig('runner.name')} runner, but it doesn't installed`);
                }

                const installerProcess = spawn('bash', [winetricksSh, 'corefonts', 'usetakefocus=n'], {
                    env: env
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
        
                installerProcess.on('close', () => {
                    resolve();
                });
            });
        });
    }

    public static isPrefixInstalled(prefixPath: string): boolean
    {
        return fs.existsSync(path.join(prefixPath, 'drive_c'));
    }

    public static patchGame(onData: (data: string) => void): Promise<boolean>
    {
        return new Promise((resolve) => {
            this.getPatchInfo().then(pathInfo => {
                if (pathInfo === null)
                    resolve(false);

                else Tools.downloadFile(constants.patchUri, path.join(constants.launcherDir, 'patch.zip')).then(() => {
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
                                WINEPREFIX: constants.prefixDir.get()
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
                                        WINEPREFIX: constants.prefixDir.get()
                                    }
                                });
    
                            // Execute the patch file with "yes" in the beginning to agree to the choice.
                            let patcherAntiCrashProcess = exec(`yes | ${path.join(patchDir, 'patch_anti_logincrash.sh')}`, {
                                cwd: constants.gameDir,
                                env: {
                                    ...process.env,
                                    WINEPREFIX: constants.prefixDir.get()
                                }
                            });
            
                            patcherAntiCrashProcess.stdout.on('data', (data: string) => onData(data));
            
                            patcherAntiCrashProcess.on('close', () => {
                                this.updateConfig('patch.version', pathInfo.version);
                                this.updateConfig('patch.state', pathInfo.state);
    
                                fs.rmSync(path.join(constants.launcherDir, 'dawn'), { recursive: true });
    
                                resolve(true);
                            });
                        });
                    });
                })
            });
        });
    }
}
