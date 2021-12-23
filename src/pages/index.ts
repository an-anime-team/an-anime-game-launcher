import { createApp } from 'vue/dist/vue.esm-bundler';

import Window from '../ts/neutralino/Window';

import Launcher from '../ts/Launcher';
import Configs from '../ts/Configs';
import constants from '../ts/Constants';

(async () => {
    Configs.defaults({
        lang: {
            launcher: 'en-us',
            voice: 'en-us'
        },
        prefix: await constants.paths.prefix.default
    });
})();

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
