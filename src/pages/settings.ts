import { createApp } from 'vue/dist/vue.esm-bundler';

import Window from '../ts/neutralino/Window';

createApp({
    data: () => ({
        title: 'about'
    }),

    mounted: () => Window.current.show()
}).mount('#app');
