<script lang="ts">
    import { onMount } from 'svelte';
    import { _, locale, locales } from 'svelte-i18n';

    import Checkbox from './components/Checkbox.svelte';
    import SelectionBox from './components/SelectionBox.svelte';
    import DXVKSelectionList from './components/DXVKSelectionList.svelte';

    let availableLocales: ArrayLike<string> = [];

    $locales.forEach((locale) => {
        availableLocales[locale] = `settings.general.items.lang.launcher.items.${locale}`;
    });

    availableLocales = availableLocales;

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

            <SelectionBox
                lang="settings.general.items.lang.launcher.title"
                prop="lang.launcher"
                items={availableLocales}
                valueChanged={(value) => $locale = value}
            />

            <DXVKSelectionList />
        </div>
    </div>
{/if}
