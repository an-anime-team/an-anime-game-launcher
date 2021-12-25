import { createApp } from 'vue/dist/vue.esm-bundler';
import { createI18n } from 'vue-i18n';

import Window from '../ts/neutralino/Window';

import Locales from '../ts/core/Locales';
import DXVK from '../ts/core/DXVK';
import Configs from '../ts/Configs';

import Checkbox from '../components/Checkbox.vue';
import LanguageSelection from '../components/LanguageSelection.vue';
import DXVKS from '../components/DXVKs.vue';
import DXVKItem from '../components/DXVKItem.vue';

const app = createApp({
    data: () => ({
        title: 'about',

        // Languages selection
        languages: {
            'en-us': 'English (US)',
            'ru-ru': 'Русский'
        },

        dxvks: []
    }),

    provide()
    {
        return {
            languages: this.languages
        };
    },

    created()
    {
        DXVK.list().then((list) => this.dxvks = list);
    },

    components: {
        'l-checkbox': Checkbox,
        'l-language': LanguageSelection,
        'l-dxvks': DXVKS,
        'l-dxvk-item': DXVKItem
    },

    mounted: () => Window.current.show()
});

Locales.get().then(async (locales) => {
    const locale = await Configs.get('lang.launcher');

    app.use(createI18n({
        locale: locale as string,
        fallbackLocale: 'en-us',

        // @ts-expect-error
        messages: locales
    }));
    
    app.mount('#app');
});
