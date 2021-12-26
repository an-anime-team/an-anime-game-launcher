<script lang="ts">
    import { onMount } from 'svelte';
    import { _, locale, locales } from 'svelte-i18n';

    import Checkbox from './components/Checkbox.svelte';

    import Window from './ts/neutralino/Window';

    // Do some stuff when all the content will be loaded
    onMount(() => {
        Window.current.show();
    });
</script>

{#if typeof $locale === 'string'}
    <div class="menu">
        <div class="menu-item menu-item-active" data-anchor="general">{$_('settings.general.title')}</div>
    </div>

    <div class="settings">
        <div class="settings-item" id="general">
            <h1>{$_('settings.general.title')}</h1>

            <Checkbox lang="settings.general.items.test1" prop="test.1" />
            <Checkbox lang="settings.general.items.test2" prop="test.2" />
            <Checkbox lang="settings.general.items.test3" prop="test.3" />

            <select bind:value={$locale}>
                {#each $locales as locale}
                    <option value={locale}>{locale}</option>
                {/each}
            </select>
        </div>
    </div>
{/if}
