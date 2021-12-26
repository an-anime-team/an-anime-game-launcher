<script lang="ts">
    import { _ } from 'svelte-i18n';

    import Configs from '../ts/Configs';

    export let prop: string = '';
    export let lang: string = '';
    export let items: ArrayLike<string> = [];
    export let valueChanged: (value: string) => void = () => {};

    import Arrow from '../assets/svgs/arrow.svg';

    let selectionOpen = false;
    let selectedValue;

    const selectionChanged = (e: MouseEvent) => {
        const li = e.target as HTMLElement;

        if (!li.classList.contains('selected'))
        {
            const lis = li.parentElement!.children!;

            for (let i = 0; i < lis.length; i++)
                lis[i].classList.remove('selected');

            li.classList.add('selected');

            Configs.set(prop, li.getAttribute('data-value'));

            selectionOpen = false;
        }
    };

    Configs.get(prop).then((value) => selectedValue = value);
</script>

<div class="select" class:select-active={selectionOpen}>
    <span>{ $_(lang) }</span>

    <div class="select-options">
        <ul>
            {#each Object.keys(items) as itemLang}
                <li
                    data-value={itemLang}

                    on:click={selectionChanged}
                    
                    on:click={() => {
                        selectedValue = itemLang;

                        valueChanged(selectedValue);
                    }}

                    class:selected={itemLang === selectedValue}
                >{ $_(items[itemLang]) }</li>
            {/each}
        </ul>
    </div>

    <div class="selected-item" on:click={() => selectionOpen = !selectionOpen}>
        <span>{ $_(items[selectedValue]) }</span>

        <!-- svelte-ignore a11y-missing-attribute -->
        <img src={Arrow} />
    </div>
</div>
