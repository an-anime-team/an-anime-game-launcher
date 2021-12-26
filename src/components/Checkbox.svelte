<script lang="ts">
    import { _ } from 'svelte-i18n';

    export let active: boolean = false;

    export let prop: string = '';
    export let lang: string = '';

    import Checkmark from '../assets/svgs/checkmark.svg';

    import Configs from '../ts/Configs';

    Configs.get(prop).then((value) => active = value as boolean);

    function updateCheckbox()
    {
        active = !active;

        Configs.set(prop, active);
    }
</script>

<div class="checkbox" class:checkbox-active={active}>
    { $_(lang) }

    <div class="checkbox-mark" on:click={updateCheckbox}>
        <!-- svelte-ignore a11y-missing-attribute -->
        <img src={Checkmark} />
    </div>
</div>
