<script lang="ts">
    import { _ } from 'svelte-i18n';

    import { Configs } from '../empathize';

    export let prop: string = '';
    export let lang: string = '';
    export let tooltip: string = '';
    export let selected: string|undefined;
    export let items = {};

    export let selectionUpdated: (property: string, value: boolean, list: object) => void = () => {};

    import Arrow from '../assets/svgs/arrow.svg';
    import Checkmark from '../assets/svgs/checkmark.svg';

    let selectionOpen = false;
    let selectedValue = selected;
    let selectedValues = {};

    Object.keys(items).forEach((key) => selectedValues[key] = false);
    Configs.get(prop).then((values) => {
        (values as string[]).forEach((key) => selectedValues[key] = true);
    });

    const updateCheckbox = (value: string) => {
        selectedValues[value] = !selectedValues[value];

        let activeVoices: string[] = [];

        Object.keys(selectedValues).forEach((key) => {
            if (selectedValues[key])
                activeVoices.push(key);
        });

        Configs.set(prop, activeVoices);

        selectionUpdated(value, selectedValues[value], selectedValues);
    };
</script>

<div class="select dropdown-checkboxes" class:select-active={selectionOpen}>
    <span>{ $_(lang) }</span>

    <div class="select-options">
        <ul>
            {#each Object.keys(items) as value}
                <li>
                    <div class="checkbox" class:checkbox-active={selectedValues[value]}>
                        <span>{ $_(items[value]) }</span>
                    
                        <div class="checkbox-mark" on:click={() => updateCheckbox(value)}>
                            <!-- svelte-ignore a11y-missing-attribute -->
                            <img src={Checkmark} />
                        </div>
                    </div>
                </li>
            {/each}
        </ul>
    </div>

    <div
        class="selected-item"
        class:hint--left={tooltip !== ''}
        class:hint--medium={tooltip !== ''}
        aria-label={$_(tooltip)}
        on:click={() => selectionOpen = !selectionOpen}
    >
        <span>{ selectedValue ? $_(items[selectedValue]) : '' }</span>

        <!-- svelte-ignore a11y-missing-attribute -->
        <img src={Arrow} class:selection-empty={selectedValue === undefined} />
    </div>
</div>

<style>
    .dropdown-checkboxes .selected-item img.selection-empty
    {
        margin-left: 0;
    }
</style>
