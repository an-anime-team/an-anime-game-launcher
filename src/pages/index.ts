import { createApp } from 'vue';

import Window from '../ts/neutralino/Window';

import Launcher from '../ts/Launcher';
import Configs from '../ts/Configs';
import constants from '../ts/Constants';
import promisify from '../ts/core/promisify';
import Process from '../ts/neutralino/Process';

promisify(async () => {
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

let app = createApp({
    data: () => ({
        uri: {
            social: '',
            background: ''
        }
    }),

    methods: {
        showAbout: () => Window.open('about')
    },

    mounted()
    {
        const launcher = new Launcher(this);

        new Promise(async (resolve) => {
            await launcher.updateSocial();
            await launcher.updateBackground();

            resolve(null);
        }).then(() => {
            Window.current.show();
        });
    }
});

app.mount('#app');
