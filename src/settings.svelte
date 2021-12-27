<script lang="ts">
    import { onMount } from 'svelte';
    import { _, locale, locales } from 'svelte-i18n';

    import Shaders from './ts/core/Shaders';

    import Checkbox from './components/Checkbox.svelte';
    import SelectionBox from './components/SelectionBox.svelte';
    import DXVKSelectionList from './components/DXVKSelectionList.svelte';
    import RunnerSelectionList from './components/RunnerSelectionList.svelte';
    import ShadersSelection from './components/ShadersSelection.svelte';

    import Window from './ts/neutralino/Window';

    // TODO: somehow simplify all this variables definitions

    /**
     * Launcher language
     */
    let launcherLocales = {};

    $locales.forEach((locale) => {
        launcherLocales[locale] = `settings.general.items.lang.launcher.items.${locale}`;
    });

    launcherLocales = launcherLocales;

    /**
     * Game voice packs languages
     */

    const voiceLocales = {
        'en-us': 'settings.general.items.lang.voice.items.en-us',
        'ja-jp': 'settings.general.items.lang.voice.items.ja-jp',
        'ko-kr': 'settings.general.items.lang.voice.items.ko-kr',
        'zn-cn': 'settings.general.items.lang.voice.items.zn-cn'
    };

    /**
     * Themes
     */

    const themes = {
        'system': 'settings.general.items.theme.items.system',
        'light': 'settings.general.items.theme.items.light',
        'dark': 'settings.general.items.theme.items.dark'
    };

    /**
     * HUD options
     */

    const huds = {
        'none': 'settings.enhancements.items.hud.items.none',
        'dxvk': 'settings.enhancements.items.hud.items.dxvk',
        'mangohud': 'settings.enhancements.items.hud.items.mangohud'
    };

    /**
     * Menu items
     */
    const menuItems = [
        'general',
        'enhancements',
        'runners',
        'dxvks',
        'shaders'
    ];

    /**
     * Menu items changing
     */
    let selectedItem: string = 'general';

    const changeItem = (event: MouseEvent) => {
        const item = event.target! as HTMLElement;
        const settings = document.getElementsByClassName('settings')[0]!;

        settings.scrollTop = document.getElementById(item.getAttribute('data-anchor') as string)!.offsetTop - 16;

        selectedItem = item.getAttribute('data-anchor')!;
    };

    const updateItems = () => {
        const settings = document.getElementsByClassName('settings')[0]! as HTMLElement;
        const settingsItems = <HTMLCollectionOf<HTMLElement>>settings.children;

        let visibleElement;

        for (let i = 0; i < settingsItems.length; ++i)
            if (settingsItems[i].offsetTop - settings.scrollTop < 180)
                visibleElement = settingsItems[i];

        selectedItem = visibleElement.getAttribute('id');
    };

    let dxvkRecommendable = true,
        runnersRecommendable = true;

    // Do some stuff when all the content will be loaded
    onMount(() => {
        Window.current.show();
    });
</script>

{#if typeof $locale === 'string'}
    <div class="menu">
        {#each menuItems as item}
            <div class="menu-item" on:click={changeItem} class:menu-item-active={selectedItem === item} data-anchor={item}>{ $_(`settings.${item}.title`) }</div>
        {/each}
    </div>

    <div class="settings" on:scroll={updateItems}>
        <div class="settings-item" id="general">
            <h1>{$_('settings.general.title')}</h1>

            <SelectionBox
                lang="settings.general.items.lang.launcher.title"
                prop="lang.launcher"
                items={launcherLocales}
                valueChanged={(value) => $locale = value}
            />

            <SelectionBox
                lang="settings.general.items.lang.voice.title"
                prop="lang.voice"
                items={voiceLocales}
            />

            <SelectionBox
                lang="settings.general.items.theme.title"
                prop="theme"
                items={themes}
            />

            <Checkbox lang="settings.general.items.discord" prop="discord.enabled" />
        </div>

        <div class="settings-item" id="enhancements">
            <h1>{$_('settings.enhancements.title')}</h1>

            <SelectionBox
                lang="settings.enhancements.items.hud.title"
                prop="hud"
                items={huds}
            />

            <Checkbox lang="settings.enhancements.items.gamemode" prop="gamemode" />
            <Checkbox lang="settings.enhancements.items.fps_unlocker" prop="fps_unlocker" />
            <Checkbox lang="settings.enhancements.items.purge_dxvk_logs" prop="purge_dxvk_logs" />
        </div>

        <div class="settings-item" id="runners">
            <h1>{$_('settings.runners.title')}</h1>

            <Checkbox lang="settings.runners.items.recommended" valueChanged={(value) => runnersRecommendable = value} />

            <RunnerSelectionList recommendable={runnersRecommendable} />
        </div>

        <div class="settings-item" id="dxvks">
            <h1>{$_('settings.dxvks.title')}</h1>

            <Checkbox lang="settings.runners.items.recommended" valueChanged={(value) => dxvkRecommendable = value} />

            <br><br>

            <DXVKSelectionList recommendable={dxvkRecommendable} />
        </div>

        <div class="settings-item" id="shaders">
            <h1>{$_('settings.shaders.title')}</h1>

            <ShadersSelection />
        </div>
    </div>
{/if}
