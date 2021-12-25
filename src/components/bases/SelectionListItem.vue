<template>
    <div class="list-item">
        {{ title }}

        <div>
            <span></span>
            <img class="item-delete" src="../../assets/images/delete.png">
            <img class="item-download" src="../../assets/images/download.png">
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    props: ['title']
});
</script>
