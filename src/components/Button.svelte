<script lang="ts">
    import { _ } from 'svelte-i18n';

    import type { Tooltip } from '../ts/types/Tooltip';

    export let lang: string = '';
    export let primary: boolean = false;
    export let disabled: boolean = false;
    export let tooltip: Tooltip|undefined = undefined;

    export let click: () => void = () => {};
</script>

<button
    class="button {tooltip === undefined ? '' : `hint--${tooltip.direction ?? 'bottom'} hint--${tooltip.size ?? 'medium'}`}"
    class:button-primary={primary}
    aria-label={tooltip ? $_(tooltip.lang) : null}
    disabled={disabled}

    on:click={click}
>{$_(lang)}</button>
