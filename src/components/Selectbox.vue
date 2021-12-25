<template>
    <div class="select" :class="{'select-active': selectionOpen}">
        <span>{{ $t(this.trueLocale) }}</span>

        <div class="select-options">
            <ul @click="selectLang">
                <slot></slot>
            </ul>
        </div>

        <div class="selected-item" @click="this.selectionOpen = !this.selectionOpen">
            <span>{{ lang.available[lang.selected] }}</span>

            <img src="../assets/svgs/arrow.svg" />
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Configs from '../ts/Configs';

export default defineComponent({
    props: ['locale', 'prop'],
    inject: ['languages'],

    data()
    {
        return {
            trueLocale: '',
            selectionOpen: false,
            lang: {
                selected: 'en-us',
                available: this.languages
            }
        };
    },

    created()
    {
        const group = this.locale.split('.')[0];
        const locale = this.locale.substring(this.locale.indexOf('.') + 1);

        this.trueLocale = `settings.${group}.items.${locale}`;

        Configs.get('lang.launcher').then((lang) => {
            this.lang.selected = (lang as string|null) ?? 'en-us';
        });
    },

    methods: {
        selectLang(event: MouseEvent)
        {
            const li = event.target as HTMLElement;

            if (!li.classList.contains('selected'))
            {
                const children = li.parentElement!.children;

                for (let i = 0; i < children.length; ++i)
                    children[i].classList.remove('selected');

                li.classList.add('selected');

                this.lang.selected = li.getAttribute('lang')!;
                this.selectionOpen = false;

                this.$i18n.locale = this.lang.selected;

                Configs.set(this.prop, this.lang.selected);
            }
        }
    }
});
</script>

<style src="../sass/components/selectbox.sass" lang="sass"></style>
