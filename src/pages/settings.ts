import { createApp } from 'vue/dist/vue.esm-bundler';
import { createI18n } from 'vue-i18n';

import Window from '../ts/neutralino/Window';

import Locales from '../ts/core/Locales';

import Checkbox from '../components/Checkbox.vue';
import Selectbox from '../components/Selectbox.vue';

const app = createApp({
    data: () => ({
        title: 'about',

        // Languages selection
        languages: {
            'en-us': 'English (US)',
            'ru-ru': 'Russian'
        }
    }),

    provide()
    {
        return {
            languages: this.languages
        };
    },

    components: {
        'l-checkbox': Checkbox,
        'l-selectbox': Selectbox
    },

    mounted: () => Window.current.show()
});

Locales.get().then((locales) => {
    app.use(createI18n({
        locale: 'en-us',
        fallbackLocale: 'en-us',

        // @ts-expect-error
        messages: locales
    }));
    
    app.mount('#app');
});
