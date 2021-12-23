import { createApp } from 'vue/dist/vue.esm-bundler';

import Window from '../ts/neutralino/Window';

import Launcher from '../ts/Launcher';

let app = createApp({
    data: () => ({
        socialUri: '',
        backgroundUri: ''
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
