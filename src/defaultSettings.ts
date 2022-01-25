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
         * Wine synchronization
         * 
         * Available options: none, esync, fsync
         * 
         * @defaul "esync"
         */
        winesync: 'esync',
    
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
         * If the launcher should enable AMD FSR
         * 
         * @default true
         */
        fsr: true,

        /**
         * If the launcher should unlock FPS
         * 
         * @default false
         */
        fps_unlocker: false,

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
});
