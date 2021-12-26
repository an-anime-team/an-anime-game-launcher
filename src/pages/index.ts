import Configs from '../ts/Configs';
import constants from '../ts/Constants';

import '../i18n';
import App from '../index.svelte';

declare const Neutralino;

Neutralino.init();

Neutralino.events.on('ready', async () => {
    Configs.defaults({
        lang: {
            launcher: 'en-us',
            voice: 'en-us'
        },

        // Path to wine prefix
        prefix: await constants.paths.prefix.default,

        // runner name to use, or null if runner is not specified
        runner: null,

        /**
         * HUD
         * 
         * null if don't use
         * otherwise should be "dxvk" or "mangohud"
         */
        hud: null,

        /**
         * vkBasalt preset to use
         * 
         * null if don't use
         * otherwise should be some folder name from the "shaders" folder
         */
        shaders: null
    });
});

const app = new App({
    target: document.getElementById('app')!
});

export default app;
