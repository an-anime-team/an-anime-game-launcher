import { locale } from 'svelte-i18n';

import {
    Windows, Process, Tray,
    Configs, Debug, IPC, fs, path,
    Cache
} from '../empathize';

import constants from './Constants';
import DiscordRPC from './core/DiscordRPC';
import Locales from './launcher/Locales';

import ProgressBar from './launcher/ProgressBar';
import State from './launcher/State';
import Background from './launcher/Background';
import Game from './Game';

import { version } from '../../package.json';

declare const Neutralino;

export default class Launcher
{
    public state?: State;
    public progressBar?: ProgressBar;
    public rpc?: DiscordRPC;
    public tray: Tray;

    /**
     * Launcher version
     */
    public static readonly version: string = version;

    protected settingsMenu?: Process;
    protected screenshotsWindow?: Process;

    public constructor(onMount)
    {
        Windows.open('splash', {
            title: 'Splash',
            width: 300,
            height: 400,
            borderless: true,
            exitProcessOnClose: false
        });
        
        this.tray = new Tray(`/public/icons/256x256.png`);
        this.tray.update();

        onMount(async () => {
            const launcherDir = await constants.paths.launcherDir;

            // Clear IPC pull if during the prev launcher's run
            // it wasn't cleared for some reasons (crash?)
            await IPC.purge();

            // Create launcher folder if it doesn't exist
            if (!await fs.exists(launcherDir))
            {
                await fs.mkdir(launcherDir);

                // Needs for the ToS violation warning window
                await fs.write(path.join(launcherDir, '.first-run'), '');
            }
            
            // Create logs folder if it doesn't exist
            if (!await fs.exists(path.join(launcherDir, 'logs')))
                await fs.mkdir(path.join(launcherDir, 'logs'));

            // Applying default settings
            const defaultSetings = (await import('../defaultSettings')).default;

            defaultSetings.then(() => {
                this.updateDiscordRPC('in-launcher');

                this.progressBar = new ProgressBar(this);
                this.state = new State(this);
            });
        });
    }

    public showScreenshots(): Promise<boolean>
    {
        return new Promise(async (resolve) => {
            if (this.screenshotsWindow)
                resolve(false);
            
            else
            {
                this.screenshotsWindow = undefined;

                const window = await Windows.open('screenshots', {
                    title: 'Screenshots',
                    width: 900,
                    height: 600,
                    resizable: true,
                    // enableInspector: true,
                    exitProcessOnClose: true
                });

                if (window.status)
                {
                    this.screenshotsWindow = new Process(window.data!.pid);

                    this.screenshotsWindow.finish(async () => {
                        this.screenshotsWindow = undefined;

                        await Windows.current.show();
                        await Windows.current.center(1280, 700);
                    });

                    Windows.current.hide();
                }
                
                resolve(window.status);
            }
        })
    }

    public showSettings(): Promise<boolean>
    {
        return new Promise(async (resolve) => {
            if (this.settingsMenu)
                resolve(false);

            else
            {
                this.settingsMenu = undefined;

                const window = await Windows.open('settings', {
                    title: 'Settings',
                    width: 900,
                    height: 600,
                    // enableInspector: true,
                    exitProcessOnClose: false
                });

                if (window.status)
                {
                    this.settingsMenu = new Process(window.data!.pid);

                    this.settingsMenu.finish(async () => {
                        this.settingsMenu = undefined;

                        await Configs.load();

                        IPC.read().then((records) => {
                            records.forEach((record) => {
                                if (record.data.type !== undefined)
                                {
                                    if (record.data.type === 'log')
                                        Debug.merge(record.pop().data.records);

                                    else if (record.data.type === 'change-locale')
                                    {
                                        locale.set(record.pop().data.locale);

                                        Background.get().then((uri) => {
                                            if (uri)
                                                document.getElementsByClassName('background')[0]!.setAttribute('src', uri);
                                        });

                                        this.getSocial().then((uri) => document.getElementById('social-iframe')!.setAttribute('src', uri));
                                    }
                                }

                                else
                                {
                                    switch (record.data)
                                    {
                                        case 'update-state':
                                            this.state?.update();

                                            break;

                                        case 'clear-cache':
                                            Cache.clear(true);

                                            break;

                                        case 'check-files-integrity':
                                            this.state!.launchButton.style['display'] = 'none';
                                            this.state!.settingsButton.style['display'] = 'none';

                                            import('./launcher/states/CheckIntegrity').then((module) => {
                                                module.default(this).then(() => {
                                                    this.state!.update().then(() => {
                                                        this.state!.launchButton.style['display'] = 'block';
                                                        this.state!.settingsButton.style['display'] = 'block';
                                                    });
                                                });
                                            });

                                            break;
                                    }

                                    record.pop();
                                }
                            });
                        });

                        Windows.current.show();
                        Windows.current.center(1280, 700);
                    });

                    Windows.current.hide();
                }

                resolve(window.status);
            }
        });
    }

    /**
     * Get launcher social buttons uri
     */
    public getSocial(): Promise<string>
    {
        return new Promise(async (resolve) => {
            const uri = await Game.server == 'global' ?
                `https://${constants.placeholders.lowercase.first}.${constants.placeholders.lowercase.company}.com/launcher/${constants.api.launcher_id.global}/${Locales.fallback((await Locales.default()) ?? 'en-us')}?api_url=https%3A%2F%2Fapi-os-takumi.${constants.placeholders.lowercase.company}.com%2Fhk4e_global&key=${constants.api.key.global}&prev=false` :
                `https://ys.${constants.placeholders.lowercase.company}.com/launcher/${constants.api.launcher_id.cn}/?api_url=https%3A%2F%2Fapi-sdk.${constants.placeholders.lowercase.company}.com%2Fhk4e_cn&key=${constants.api.key.cn}&prev=false`;

            resolve(uri);
        });
    }

    /**
     * Update Discord RPC notification if it is enabled
     */
    public updateDiscordRPC(state: 'in-launcher' | 'in-game'): Promise<void>
    {
        return new Promise((resolve) => {
            Configs.get('discord').then((discord) => {
                // @ts-expect-error
                if (discord.enabled)
                {
                    if (this.rpc)
                        this.rpc.stop(true);
                    
                    this.rpc = new DiscordRPC({
                        id: '901534333360304168',

                        // @ts-expect-error
                        details: discord.states[state].details,

                        // @ts-expect-error
                        state: discord.states[state].state,
    
                        icon: {
                            // @ts-expect-error
                            large: discord.states[state].icon
                        },
    
                        time: {
                            // @ts-expect-error
                            start: state === 'in-launcher' ? 0 : (discord.timer ? Math.round(Date.now() / 1000) : 0)
                        }
                    });
    
                    resolve();
                }
            });
        });
    }

    /**
     * Check if the launcher is running under flatpak
     */
    public static isFlatpak(): Promise<boolean>
    {
        return new Promise((resolve) => {
            Neutralino.filesystem.getStats('/.flatpak-info')
                .then((stats) => resolve(stats.isFile))
                .catch(() => resolve(false));
        });
    }

    /**
     * Check if the launcher is running in steam os
     */
    public static isSteamOs(): Promise<boolean>
    {
        return new Promise((resolve) => {
            Neutralino.os.getEnv('SteamOS')
                .then((env) => resolve(env === '1'))
                .catch(() => resolve(false));
        });
    }
};
