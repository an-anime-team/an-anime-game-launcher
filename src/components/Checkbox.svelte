<script lang="ts">
    import { _ } from 'svelte-i18n';

    export let active: boolean = false;
    export let disabled: boolean = false;

    export let prop: string = '';
    export let lang: string = '';
    export let tooltip: string = '';

    export let valueChanged: (value: boolean) => void = () => {};

    import Checkmark from '../assets/svgs/checkmark.svg';

    import Configs from '../ts/Configs';

    Configs.get(prop).then((value) => active = value as boolean);

    function updateCheckbox()
    {
        active = !active;

        if (prop)
            Configs.set(prop, active);

        if (valueChanged)
            valueChanged(active);
    }
</script>

<div class="checkbox" class:checkbox-active={active} class:checkbox-disabled={disabled}>
    <span
        class:hint--bottom={tooltip !== ''}
        class:hint--medium={tooltip !== ''}
        aria-label={$_(tooltip)}
    >{ $_(lang) }</span>

    <div class="checkbox-mark" on:click={updateCheckbox}>
        <!-- svelte-ignore a11y-missing-attribute -->
        <img src={Checkmark} />
    </div>
</div>
