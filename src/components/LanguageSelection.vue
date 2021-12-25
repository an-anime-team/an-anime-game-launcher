<script lang="ts">
import { defineComponent } from 'vue';
import Configs from '../ts/Configs';

import Selectbox from './bases/Selectbox.vue';

export default defineComponent({
    mixins: [Selectbox],
    inject: ['languages'],

    data()
    {
        return {
            trueTitle: '',
            selectionOpen: false,
            options: {
                selected: 'en-us',
                available: this.languages
            }
        };
    },

    created()
    {
        const group = this.locale.split('.')[0];
        const locale = this.locale.substring(this.locale.indexOf('.') + 1);

        this.trueTitle = `settings.${group}.items.${locale}`;

        Configs.get('lang.launcher').then((lang) => {
            this.options.selected = (lang as string|null) ?? 'en-us';

            const children = document.getElementById(this.prop)!.children;

            /*for (let i = 0; i < children.length; ++i)
                if (children[i].getAttribute('lang') === )*/
        });
    },

    methods: {
        selectionChanged(value: string)
        {
            this.$i18n.locale = value;
        },

        defaultOption(): Promise<string>
        {
            return Configs.get('lang.launcher') as Promise<string>;
        }
    }
});
</script>
