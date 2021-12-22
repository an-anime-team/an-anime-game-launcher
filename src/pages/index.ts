import * as Vue from 'vue/dist/vue.esm-bundler';

import Window from '../ts/neutralino/Window';
import Patch from '../ts/Patch';
import Voice from '../ts/Voice';
import fetch from '../ts/core/Fetch';
import Downloader from '../ts/core/Downloader';

const app = Vue.createApp({
    data: () => ({
        progress: '0%'
    }),

    methods: {
        showAbout: () => Window.open('about')
    },

    mounted: () => {
        Window.current.show();

        /*fetch('https://sdk-os-static.mihoyo.com/hk4e_global/mdk/launcher/api/resource?key=gcStgarh&launcher_id=10', 1000)
            .then((response) => {
                console.log(response);

                response.body().then((body) => console.log(body.length));
            });*/

        /*Downloader.download('https://github.com/GloriousEggroll/wine-ge-custom/releases/download/6.20-GE-1/wine-lutris-ge-6.20-1-x86_64.tar.xz', '123.tar.xz').then((stream) => {
            stream.start(() => console.log('Downloading started'));
            stream.finish(() => console.log('Downloading finished'));

            stream.progress((current, total) => console.log(`${Math.round(current / total * 100)}%`));
        });*/

        /*Runners.download('wine-lutris-ge-6.20-1-x86_64').then((stream) => {
            stream.downloadStart(() => console.log('Downloading started'));
            stream.downloadFinish(() => console.log('Downloading finished'));

            stream.unpackStart(() => console.log('Unpacking started'));
            stream.unpackFinish(() => console.log('Unpacking finished'));
        });*/

        /*DXVK.download('1.8').then((stream) => {
            stream.downloadStart(() => console.log('Downloading started'));
            stream.downloadFinish(() => console.log('Downloading finished'));

            stream.unpackStart(() => console.log('Unpacking started'));
            stream.unpackFinish(() => console.log('Unpacking finished'));
        });*/

        /*Archive.unpack('Audio_English(US)_2.3.0.zip', 'tmp').then((stream) => {
            stream.progress((current, total) => {
                console.log(`${Math.round(current / total * 100)}%`);
            });

            stream.finish(() => console.log('finished'));
        });*/
    }
});

app.mount('#app');
