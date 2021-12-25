<template>
    <h1>{{ property }}</h1>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    props: ['property'],

});
</script>
