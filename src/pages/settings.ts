import { createApp } from 'vue/dist/vue.esm-bundler';
import { createI18n } from 'vue-i18n';

import Window from '../ts/neutralino/Window';

import Checkbox from '../components/Checkbox.vue';
import Locales from '../ts/core/Locales';

const app = createApp({
    data: () => ({
        title: 'about'
    }),

    components: {
        'l-checkbox': Checkbox
    },

    mounted: () => Window.current.show()
});

Locales.get().then((locales) => {
    app.use(createI18n({
        locale: 'en',
        fallbackLocale: 'en',

        // @ts-expect-error
        messages: locales
    }));
    
    app.mount('#app');
});
