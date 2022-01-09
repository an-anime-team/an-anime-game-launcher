<script context="module" lang="ts">
    declare const Neutralino;
</script>

<script lang="ts">
    import { onMount } from 'svelte';
    import { _, locale, locales } from 'svelte-i18n';

    import constants from './ts/Constants';
    import Configs from './ts/Configs';
    import FPSUnlock from './ts/FPSUnlock';
    import Window from './ts/neutralino/Window';
    import Debug from './ts/core/Debug';
    import IPC from './ts/core/IPC';
    import Process from './ts/neutralino/Process';
    import Runners from './ts/core/Runners';

    import Button from './components/Button.svelte';
    import Checkbox from './components/Checkbox.svelte';
    import SelectionBox from './components/SelectionBox.svelte';
    import DropdownCheckboxes from './components/DropdownCheckboxes.svelte';
    import DiscordSettings from './components/DiscordSettings.svelte';
    import DXVKSelectionList from './components/DXVKSelectionList.svelte';
    import RunnerSelectionList from './components/RunnerSelectionList.svelte';
    import ShadersSelection from './components/ShadersSelection.svelte';

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
     * Delete launcher logs options
     */

    const purgeLauncherLogs = {
        '1d': 'settings.enhancements.items.purge_logs.launcher.items.1d',
        '3d': 'settings.enhancements.items.purge_logs.launcher.items.3d',
        '5d': 'settings.enhancements.items.purge_logs.launcher.items.5d',
        '7d': 'settings.enhancements.items.purge_logs.launcher.items.7d',
        '14d': 'settings.enhancements.items.purge_logs.launcher.items.14d',
        'never': 'settings.enhancements.items.purge_logs.launcher.items.never'
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
     * Some components stuff
     */
    let dxvkRecommendable = true,
        runnersRecommendable = true,
        fpsUnlockerAvailable = true,
        voiceUpdateRequired = false;

    let discordSettings: object = {}, discordSettingsUpdater = false;

    Configs.get('discord').then((settings) => discordSettings = settings as object);

    const handleDiscordRpcText = (field: 'in-game' | 'in-launcher', value: string) => {
        const lines = value.split(/\r\n|\r|\n/).filter((line) => line != '');

        discordSettings['states'][field]['details'] = lines[0];
        discordSettings['states'][field]['state'] = '';

        if (lines[1] !== undefined)
            discordSettings['states'][field]['state'] = lines[1];

        // This thing will update config file only after a second
        // so we'll not update it every time user prints some character
        // in textarea
        if (!discordSettingsUpdater)
        {
            discordSettingsUpdater = true;

            setTimeout(() => {
                discordSettingsUpdater = false;

                Configs.set('discord', discordSettings);
            }, 1000);
        }
    };

    const handleDiscordRpcIcon = (field: 'in-game' | 'in-launcher', icon: string) => {
        discordSettings['states'][field]['icon'] = icon;

        Configs.set('discord', discordSettings);
    };

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

    const switchTheme = (theme: string) => {
        if (theme === 'system')
            theme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';

        document.body.setAttribute('data-theme', theme as string);
    };

    // Auto theme switcher
    Configs.get('theme').then((theme) => switchTheme(theme as string));

    // Do some stuff when all the content will be loaded
    onMount(async () => {
        await Window.current.show();
        await Window.current.center(900, 600);

        // This thing will fix window resizing
        // in several cases (wayland + gnome + custom theme)
        const resizer = () => {
            if (window.innerWidth < 700)
                setTimeout(resizer, 10);

            else
            {
                Window.current.setSize({
                    width: 900 + (900 - window.innerWidth),
                    height: 600 + (600 - window.innerHeight),
                    resizable: false
                });
            }
        }

        setTimeout(resizer, 10);
    });

    Neutralino.events.on('windowClose', async () => {
        await IPC.write({
            type: 'log',
            records: Debug.getRecords()
        });

        if (voiceUpdateRequired)
            await IPC.write('voice-update-required');

        Neutralino.app.exit();
    });
</script>

{#if typeof $locale === 'string'}
    <main>
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
                    valueChanged={(value) => {
                        $locale = value;

                        IPC.write({
                            type: 'change-locale',
                            locale: value
                        });
                    }}
                />

                <DropdownCheckboxes
                    lang="settings.general.items.lang.voice.title"
                    tooltip="settings.general.items.lang.voice.tooltip"
                    prop="lang.voice"
                    selected={undefined}
                    items={voiceLocales}
                    selectionUpdated={() => voiceUpdateRequired = true}
                />

                <SelectionBox
                    lang="settings.general.items.theme.title"
                    prop="theme"
                    items={themes}
                    valueChanged={switchTheme}
                />

                <Checkbox
                    lang="settings.general.items.discord.title"
                    prop="discord.enabled"
                    valueChanged={(value) => discordSettings['enabled'] = value}
                />

                <DiscordSettings visible={discordSettings['enabled']} valueChanged={handleDiscordRpcText} iconChanged={handleDiscordRpcIcon} />

                <div style="margin-top: 24px">
                    <Button lang="settings.general.items.buttons.winetricks" click={async () => {
                        const runner = await Runners.current();

                        const runnersDir = await constants.paths.runnersDir;
                        
                        Process.run(`"${Process.addSlashes(await constants.paths.launcherDir)}/winetricks.sh"`, {
                            env: {
                                WINE: runner ? `${runnersDir}/${runner.name}/${runner.files.wine}` : 'wine',
                                WINESERVER: runner ? `${runnersDir}/${runner.name}/${runner.files.wineserver}` : 'wineserver',
                                WINEPREFIX: await constants.paths.prefix.current
                            }
                        });
                    }} />

                    <Button lang="settings.general.items.buttons.winecfg" click={async () => {
                        const runner = await Runners.current();

                        const runnerDir = runner ? `${await constants.paths.runnersDir}/${runner.name}` : '';
                        
                        Process.run(runner ? `"${Process.addSlashes(`${runnerDir}/${runner.files.wine}`)}" "${Process.addSlashes(`${runnerDir}/${runner.files.winecfg}`)}"` : 'winecfg', {
                            env: {
                                WINE: runner ? `${runnerDir}/${runner.files.wine}` : 'wine',
                                WINESERVER: runner ? `${runnerDir}/${runner.files.wineserver}` : 'wineserver',
                                WINEPREFIX: await constants.paths.prefix.current
                            }
                        });
                    }} />

                    <!-- svelte-ignore missing-declaration -->
                    <Button lang="settings.general.items.buttons.launcher" click={async () => {
                        Neutralino.os.execCommand(`xdg-open "${Process.addSlashes(await constants.paths.launcherDir)}"`, {
                            background: true
                        });
                    }} />

                    <!-- svelte-ignore missing-declaration -->
                    <Button lang="settings.general.items.buttons.game" click={async () => {
                        Neutralino.os.execCommand(`xdg-open "${Process.addSlashes(await constants.paths.gameDir)}"`, {
                            background: true
                        });
                    }} />
                </div>
            </div>

            <div class="settings-item" id="enhancements">
                <h1>{$_('settings.enhancements.title')}</h1>

                <SelectionBox
                    lang="settings.enhancements.items.hud.title"
                    prop="hud"
                    items={huds}
                />

                <Checkbox
                    lang="settings.enhancements.items.gamemode.title"
                    tooltip="settings.enhancements.items.gamemode.tooltip"
                    prop="gamemode"
                />

                <Checkbox
                    lang="settings.enhancements.items.fps_unlocker.title"
                    tooltip="settings.enhancements.items.fps_unlocker.tooltip"
                    prop="fps_unlocker"
                    disabled={!fpsUnlockerAvailable}
                    valueChanged={async (checked) => {
                        if (checked && !await FPSUnlock.installed())
                        {
                            fpsUnlockerAvailable = false;

                            FPSUnlock.install().then(() => fpsUnlockerAvailable = true);
                        }
                    }}
                />

                <Checkbox
                    lang="settings.enhancements.items.purge_logs.game.title"
                    tooltip="settings.enhancements.items.purge_logs.game.tooltip"
                    prop="purge_logs.game"
                />

                <SelectionBox
                    lang="settings.enhancements.items.purge_logs.launcher.title"
                    tooltip="settings.enhancements.items.purge_logs.launcher.tooltip"
                    prop="purge_logs.launcher"
                    items={purgeLauncherLogs}
                />
            </div>

            <div class="settings-item" id="runners">
                <h1>{$_('settings.runners.title')}</h1>

                <Checkbox
                    lang="settings.runners.items.recommended.title"
                    tooltip="settings.runners.items.recommended.tooltip"
                    valueChanged={(value) => runnersRecommendable = value}
                />

                <RunnerSelectionList recommendable={runnersRecommendable} />
            </div>

            <div class="settings-item" id="dxvks">
                <h1>{$_('settings.dxvks.title')}</h1>

                <Checkbox
                    lang="settings.dxvks.items.recommended.title"
                    tooltip="settings.dxvks.items.recommended.tooltip"
                    valueChanged={(value) => dxvkRecommendable = value}
                />

                <br><br>

                <DXVKSelectionList recommendable={dxvkRecommendable} />
            </div>

            <div class="settings-item" id="shaders">
                <h1>{$_('settings.shaders.title')}</h1>

                <ShadersSelection />
            </div>
        </div>
    </main>
{/if}
