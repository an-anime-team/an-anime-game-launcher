import * as Vue from 'vue/dist/vue.esm-bundler';

import Window from '../ts/neutralino/Window';
import Runners from '../ts/Runners';

Runners.get().then(console.log);

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
