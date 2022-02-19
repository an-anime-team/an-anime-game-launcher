<script lang="ts">
    import { _ } from 'svelte-i18n';

    import type { Tooltip } from '../ts/types/Tooltip';

    import { Configs } from '../empathize';

    export let active: boolean = false;
    export let disabled: boolean = false;

    export let prop: string = '';
    export let lang: string = '';
    export let tooltip: Tooltip|undefined = undefined;

    export let valueChanged: (value: boolean) => void = () => {};

    import Checkmark from '../assets/svgs/checkmark.svg';

    Configs.get(prop).then((value) => active = value as boolean);

    async function updateCheckbox()
    {
        active = !active;

        if (prop)
            await Configs.set(prop, active);

        if (valueChanged)
            valueChanged(active);
    }
</script>

<div class="checkbox" class:checkbox-active={active} class:checkbox-disabled={disabled}>
    <div class="checkbox-mark" on:click={updateCheckbox}>
        <!-- svelte-ignore a11y-missing-attribute -->
        <img src={Checkmark} />
    </div>

    <span
        class="{tooltip === undefined ? '' : `hint--${tooltip.direction ?? 'bottom'} hint--${tooltip.size ?? 'medium'}`}"
        aria-label={tooltip ? $_(tooltip.lang) : null}
    >{$_(lang)}</span>
</div>

<style lang="sass">
    .checkbox-mark
        margin-left: 0 !important
        margin-right: 16px !important
</style>
