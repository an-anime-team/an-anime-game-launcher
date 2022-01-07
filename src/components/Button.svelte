<script lang="ts">
    import { _ } from 'svelte-i18n';

    export let lang: string = '';

    export let click: () => void = () => {};
</script>

<button class="button" on:click={click}>{$_(lang)}</button>
