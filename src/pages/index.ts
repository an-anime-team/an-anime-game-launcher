import * as Vue from 'vue/dist/vue.esm-bundler';

import Window from '../ts/neutralino/Window';

import Downloader from '../ts/Downloader';
import Archive from '../ts/Archive';

const app = Vue.createApp({
    data: () => ({
        progress: '0%'
    }),

    methods: {
        showAbout: () => Window.open('about')
    },

    mounted: () => {
        Window.current.show();

        /*Archive.unpack('Audio_English(US)_2.3.0.zip', 'tmp').then((stream) => {
            stream.progress((current, total) => {
                console.log(`${Math.round(current / total * 100)}%`);
            });

            stream.finish(() => console.log('finished'));
        });*/

        /*Downloader.download('https://autopatchhk.yuanshen.com/client_app/download/pc_zip/20211117173404_G0gLRnxvOd4PvSu9/Audio_English(US)_2.3.0.zip').then((stream) => {
            stream.progress((current: number, total: number) => {
                document.getElementById('progress').innerHTML = `${Math.round(current / total * 100)}%`;
            });

            stream.finish(() => {
                console.log('finished');
            });
        });*/
    }
});

app.mount('#app');
