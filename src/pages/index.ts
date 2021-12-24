import { createApp } from 'vue/dist/vue.esm-bundler';

import Window from '../ts/neutralino/Window';

import Launcher from '../ts/Launcher';
import Configs from '../ts/Configs';
import constants from '../ts/Constants';
import promisify from '../ts/core/promisify';
import Game from '../ts/Game';

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

    mounted()
    {
        const launcher = new Launcher(this);

        /**
         * Update launcher's title
         */
        Game.latest.then((game) => {
            Window.current.setTitle(`${constants.placeholders.uppercase.full} Linux Launcher - ${game.version}`);
        });

        /**
         * Add some events to some elements
         */
        const settingsButton = document.getElementById('settings');

        settingsButton!.onclick = () => launcher.showSettings();

        settingsButton!.onmouseenter = () => {
            settingsButton?.classList.add('hovered');
        };

        settingsButton!.onmouseleave = () => {
            settingsButton?.classList.remove('hovered');
        };

        /**
         * Do some launcher stuff
         */
        const pipeline = promisify({
            callbacks: [
                () => launcher.updateSocial(),
                () => launcher.updateBackground()
            ],
            callAtOnce: true
        });

        // Show window when all the stuff was completed
        pipeline.then(() => {
            Window.current.show();
        });
    }
});

app.mount('#app');
