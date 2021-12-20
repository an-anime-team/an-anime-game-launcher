import * as Vue from 'vue/dist/vue.esm-bundler';

import Window from '../ts/neutralino/Window';

Vue.createApp({
    data: () => {
        return {
            title: 'about'
        };
    },

    mounted: () => Window.current.show()
}).mount('#app');
