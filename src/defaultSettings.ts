import Configs from './ts/Configs';
import constants from './ts/Constants';
import promisify from './ts/core/promisify';

promisify(async () => {
    Configs.defaults({
        lang: {
            launcher: 'en-us',
            voice: 'en-us'
        },
    
        /**
         * Path to wine prefix
         * 
         * @default constants.paths.prefix.default
         */
        prefix: await constants.paths.prefix.default,
    
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
         * "none" if don't use. Otherwise should be "dxvk" or "mangohud"
         * 
         * @default "none"
         */
        hud: 'none',
    
        /**
         * vkBasalt preset to use
         * 
         * null if don't use. Otherwise should be some folder name from the "shaders" folder
         * 
         * @default null
         */
        shaders: null,

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

            // todo
            texts: {
                idk: true
            }
        },

        /**
         * Do the launcher should use GameMode
         * 
         * @default false
         */
        gamemode: false,

        /**
         * Do the launcher should use FPS unlocker
         * 
         * @default false
         */
        fps_unlocker: false,

        /**
         * Do the launcher should automatically delete DXVK log files
         * 
         * @default true
         */
        purge_dxvk_logs: true
    });
});
