import Configs from './ts/Configs';
import constants from './ts/Constants';
import promisify from './ts/core/promisify';

promisify(async () => {
    Configs.defaults({
        lang: {
            launcher: 'en-us',
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
            game: `${await constants.paths.prefix.default}/game/drive_c/Program Files/${constants.placeholders.uppercase.first} ${constants.placeholders.uppercase.second}`,
            
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
         * @default null
         */
        env: null,
    
        /**
         * Launcher theme
         * 
         * Can be "system", "light" and "dark"
         * 
         * @defaul "system"
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
         * If the launcher should unlock FPS
         * 
         * @default false
         */
        fps_unlocker: false,

        /**
         * If the launcher should automatically delete DXVK log files
         * 
         * @default true
         */
        purge_dxvk_logs: true
    });
});
