import { locale } from 'svelte-i18n';

import {
    Windows, Process, Tray,
    Configs, Debug, IPC
} from '../empathize';

import constants from './Constants';
import DiscordRPC from './core/DiscordRPC';
import Locales from './launcher/Locales';

import ProgressBar from './launcher/ProgressBar';
import State from './launcher/State';
import Background from './launcher/Background';

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

    public constructor(onMount)
    {
        this.tray = new Tray(`/public/icons/256x256.png`);
        this.tray.update();

        this.updateDiscordRPC('in-launcher');

        onMount(() => {
            this.progressBar = new ProgressBar(this);
            this.state = new State(this);
        });
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

                    this.settingsMenu.finish(() => {
                        this.settingsMenu = undefined;

                        IPC.read().then((records) => {
                            records.forEach((record) => {
                                if (record.data.type !== undefined)
                                {
                                    if (record.data.type === 'log')
                                        Debug.merge(record.pop().data.records);

                                    else if (record.data.type === 'change-locale')
                                    {
                                        locale.set(record.pop().data.locale);

                                        Background.get().then((uri) => document.getElementsByClassName('background')[0]!.setAttribute('src', uri));
                                        this.getSocial().then((uri) => document.getElementById('social-iframe')!.setAttribute('src', uri));
                                    }
                                }

                                else if (record.data === 'voice-update-required')
                                {
                                    this.state!.set('game-voice-update-required');

                                    record.pop();
                                }
                            });
                        });

                        Windows.current.show();
                        Windows.current.center(1280, 700);
                    })

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
            resolve(`https://${constants.placeholders.lowercase.first}.${constants.placeholders.lowercase.company}.com/launcher/10/${Locales.fallback((await Locales.default()) ?? 'en-us')}?api_url=https%3A%2F%2Fapi-os-takumi.${constants.placeholders.lowercase.company}.com%2Fhk4e_global&key=gcStgarh&prev=false`);
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
};
