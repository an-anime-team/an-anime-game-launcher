import { Configs, promisify } from './empathize';

import constants from './ts/Constants';
import Locales from './ts/launcher/Locales';

export default new Promise<void>(async (resolve) => {
    await Configs.defaults({
        lang: {
            launcher: await Locales.system(),
            voice: [
                'en-us'
            ]
        },

        folders: {
            /**
             * Path to wine prefix
             * 
             * @default "~/.local/share/anime-game-launcher/game"
             */
            prefix: await constants.paths.prefix.default,

            /**
             * Path to game installation folder
             * 
             * @default "~/.local/share/anime-game-launcher/game/drive_c/Program Files/[An Anime Game]"
             */
            game: `${await constants.paths.prefix.default}/drive_c/Program Files/${constants.placeholders.uppercase.first} ${constants.placeholders.uppercase.second}`,
            
            /**
             * Path to some temp folder
             * 
             * @default "~/.local/share/anime-game-launcher"
             */
            temp: await constants.paths.launcherDir
        },
    
        /**
         * Runner name to use, or null if runner is not specified
         * 
         * @default null
         */
        runner: null,
    
        /**
         * DXVK name to use, or null if DXVK is not specified
         * 
         * @default null
         */
        dxvk: null,

        /**
         * Environment variables
         * 
         * null to disable any
         * 
         * @default null
         */
        env: null,
    
        /**
         * Launcher theme
         * 
         * Can be "system", "light" and "dark"
         * 
         * @default "system"
         */
        theme: 'system',
    
        /**
         * HUD
         * 
         * "none" if not in use. Otherwise it's "dxvk" or "mangohud"
         * 
         * @default "none"
         */
        hud: 'none',

        /**
         * Wine-related configs
         */
        wine: {
            /**
             * Wine synchronization
             * 
             * Available options: none, esync, fsync
             * 
             * @default "esync"
             */
            sync: 'esync',

            /**
             * If the launcher should enable AMD FSR
             * 
             * @default true
             */
            fsr: true,

            /**
             * Wine Virtual Desktop
             */
            virtual_desktop: {
                /**
                 * If it is enabled
                 * 
                 * @default false
                 */
                enabled: false,

                /**
                 * Virtual Desktop Width
                 * 
                 * @default 720
                 */
                width: 720,

                /**
                 * Virtual Desktop Height
                 * 
                 * @default 1280
                 */
                height: 1280
            }
        },
    
        /**
         * vkBasalt preset to use
         * 
         * "none" if not in use. Otherwise it should be a folder name from the "shaders" folder
         * 
         * @default "none"
         */
        shaders: 'none',

        /**
         * Discord RPC integration
         */
        discord: {
            /**
             * If it is enabled
             * 
             * @default false
             */
            enabled: false,

            /**
             * Should it display amount of spent time or not
             * 
             * @default true
             */
            timer: true,

            /**
             * Discord RPC states
             */
            states: {
                'in-launcher': {
                    details: 'Preparing to launch',
                    state: '',
                    icon: 'launcher'
                },

                'in-game': {
                    details: 'Exploring the landscape',
                    state: 'of Teyvat',
                    icon: 'game'
                }
            }
        },

        /**
         * If the launcher should use GameMode
         * 
         * @default false
         */
        gamemode: false,

        /**
         * Allow Borderless Window
         * 
         * Whether the game launches with the parameters "-screen-fullscreen 0 -popupwindow"
         * This allows the game to be played in a borderless window by selecting fullscreen and pressing alt+enter
         * 
         * @default false
         */
        borderless_window: false,

        /**
         * If the launcher should unlock FPS
         * 
         * @default false
         */
        fps_unlocker: false,

        /**
         * Use separate terminal window to run the wine command
         * 
         * It'll try to use some of the supported default terminal applications. Otherwise the option will not have an effect
         * 
         * More details in the `Launch.ts` script
         * 
         * @default false
         */
        use_terminal: false,

        /**
         * If the launcher should automatically delete log files
         */
        purge_logs: {
            /**
             * Should launcher delete some game logs (DXVK .log and .dmp files)
             * 
             * @default true
             */
            game: true,

            /**
             * Period of time launcher should delete its logs
             * 
             * Can be in "*d" format, where * is amount of days (e.g. 5d = 5 days), or "never" to never delete logs
             * 
             * @default "5d"
             */
            launcher: '5d'
        }
    });

    /**
     * 2.1.0-beta1 -> 2.1.1
     */
    const winesync = await Configs.get('winesync');
    const fsr = await Configs.get('fsr');
    const winevd = await Configs.get('winevd');

    if (winesync !== undefined)
    {
        await Configs.set('wine.sync', winesync);
        await Configs.remove('winesync');
    }

    if (fsr !== undefined)
    {
        await Configs.set('wine.fsr', fsr);
        await Configs.remove('fsr');
    }

    if (winevd !== undefined)
    {
        await Configs.set('wine.virtual_desktop', winevd);
        await Configs.remove('winevd');
    }

    resolve();
});
