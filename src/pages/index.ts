import * as Vue from 'vue/dist/vue.esm-bundler';

import Window from '../ts/neutralino/Window';

import Downloader from '../ts/Downloader';

Downloader.download('https://vitejs.dev/', 'test.html').then((stream) => {
    stream.progress((current: number, total: number) => {
        console.log(`${Math.round(current / total * 100)}%`);
    });

    stream.finish(() => {
        console.log('finished');
    });
});

Vue.createApp({
    data: () => {
        return {
            title: 'index'
        };
    },

    methods: {
        showAbout: () => Window.open('about')
    },

    mounted: () => Window.current.show()
}).mount('#app');
