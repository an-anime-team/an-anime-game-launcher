<template>
    <div class="select" :class="{'select-active': selectionOpen}">
        <span>{{ $t(this.trueTitle) }}</span>

        <div class="select-options">
            <ul @click="selectItem" :id="prop">
                <slot></slot>
            </ul>
        </div>

        <div class="selected-item" @click="this.selectionOpen = !this.selectionOpen">
            <span>{{ options.available[options.selected] }}</span>

            <img src="../../assets/svgs/arrow.svg" />
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Configs from '../../ts/Configs';

export default defineComponent({
    props: ['locale', 'prop'],

    data()
    {
        return {
            trueTitle: '',
            selectionOpen: false,
            options: {
                selected: '',
                available: []
            }
        };
    },

    created()
    {
        const group = this.locale.split('.')[0];
        const locale = this.locale.substring(this.locale.indexOf('.') + 1);

        this.trueTitle = `settings.${group}.items.${locale}`;

        this.defaultOption().then((option) => {
            this.options.selected = option;

            const children = document.getElementById(this.prop)!.children;

            for (let i = 0; i < children.length; ++i)
                if (children[i].getAttribute('option') === option)
                    children[i].classList.add('selected');
        });
    },
    
    methods: {
        selectItem(event: MouseEvent)
        {
            const li = event.target as HTMLElement;

            if (!li.classList.contains('selected'))
            {
                const children = li.parentElement!.children;

                for (let i = 0; i < children.length; ++i)
                    children[i].classList.remove('selected');

                li.classList.add('selected');

                this.options.selected = li.getAttribute('option')!;
                this.selectionOpen = false;

                this.selectionChanged(this.options.selected);

                Configs.set(this.prop, this.options.selected);
            }
        },

        selectionChanged(value: string) {},
        defaultOption(): Promise<string> { return new Promise((resolve) => resolve('')); }
    }
});
</script>

<style src="../../sass/components/selectbox.sass" lang="sass"></style>
