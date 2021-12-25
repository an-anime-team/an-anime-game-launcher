<template>
    <div class="checkbox" :class="{'checkbox-active': active}">
        {{ $t(`settings.${locale.split('.')[0]}.items.${locale.substring(locale.indexOf('.') + 1)}`) }}

        <div class="checkbox-mark" @click="updateSetting">
            <img src="../assets/svgs/checkmark.svg" />
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Configs from '../ts/Configs';

export default defineComponent({
    props: ['locale', 'prop'],

    data: () => ({
        active: false
    }),

    async created()
    {
        this.active = await Configs.get(this.prop) as boolean;
    },

    methods: {
        updateSetting()
        {
            this.active = !this.active;

            Configs.set(this.prop, this.active);
        }
    }
});
</script>

<style src="../sass/components/checkbox.sass" lang="sass"></style>
