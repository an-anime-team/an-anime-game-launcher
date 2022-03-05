<script context="module" lang="ts">
    declare const Neutralino;
</script>

<script lang="ts">
    import { onMount } from 'svelte';
    import { _, locale, locales } from 'svelte-i18n';

    import { Windows, Configs, Debug, IPC, Process, path, Package } from './empathize';

    import constants from './ts/Constants';
    import Launcher from './ts/Launcher';
    import FPSUnlock from './ts/FPSUnlock';
    import Runners from './ts/core/Runners';
    import Patch from './ts/Patch';

    import type { PatchInfo } from './ts/types/Patch';

    import Button from './components/Button.svelte';
    import Checkbox from './components/Checkbox.svelte';
    import SelectionBox from './components/SelectionBox.svelte';
    import DropdownCheckboxes from './components/DropdownCheckboxes.svelte';
    import DiscordSettings from './components/DiscordSettings.svelte';
    import WineVDSettings from './components/WineVDSettings.svelte';
    import DXVKSelectionList from './components/DXVKSelectionList.svelte';
    import RunnerSelectionList from './components/RunnerSelectionList.svelte';
    import ShadersSelection from './components/ShadersSelection.svelte';
    import EnvironmentManager from './components/EnvironmentManager.svelte';

    /**
     * Launcher language
     */
    let launcherLocales = {};

    $locales.forEach((locale) => {
        launcherLocales[locale] = `settings.general.items.lang.launcher.items.${locale}`;
    });

    launcherLocales = launcherLocales;

    /**
     * Some components stuff
     */
    let dxvkRecommendable = true,
        runnersRecommendable = true,
        fpsUnlockerAvailable = true,
        voiceUpdateRequired = false;
    
    let winevdSettings: object = {},
        winevdSettingsUpdater = false;

    Configs.get('wine.virtual_desktop').then((settings) => winevdSettings = settings as object);

    const handleWineVD = (field: 'width' | 'height', value: string) => {
        winevdSettings[field] = parseInt(value);

        // This thing will update config file only after a second
        // so we'll not update it every time user prints some character
        // in textarea
        if (!winevdSettingsUpdater)
        {
            winevdSettingsUpdater = true;

            setTimeout(() => {
                winevdSettingsUpdater = false;

                Configs.set('wine.virtual_desktop', winevdSettings);
            }, 1000);
        }
    };

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
     * GameMode option
     */
    let gamemode = {
        disabled: false,
        tooltip: 'settings.enhancements.game.items.gamemode.tooltip.enabled'
    };

    Package.exists('gamemoderun').then((available) => {
        gamemode.disabled = !available;

        if (gamemode.disabled)
            gamemode.tooltip = 'settings.enhancements.game.items.gamemode.tooltip.disabled';
    });

    let borderless_active = Configs.get('borderless_window').then((value) => borderless_active = value);

    /**
     * Patch info
     */
    let patchInfo: PatchInfo|null = null;

    Patch.latest.then((value) => patchInfo = value);

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
        await Windows.current.show();
        await Windows.current.center(900, 600);

        // This thing will fix window resizing
        // in several cases (wayland + gnome + custom theme)
        const resizer = () => {
            if (window.innerWidth < 700)
                setTimeout(resizer, 10);

            else
            {
                Windows.current.setSize({
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
            await IPC.write('update-state');

        Neutralino.app.exit();
    });
</script>

{#if typeof $locale === 'string'}
    <main>
        <div class="menu">
            {#each ['general', 'enhancements', 'runners', 'dxvks', 'shaders', 'environment', 'patch'] as item}
                <div
                    class="menu-item"
                    class:menu-item-active={selectedItem === item}
                    data-anchor={item}
                    on:click={changeItem}
                >{$_(`settings.${item}.title`)}</div>
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
                    items={{
                        'en-us': 'settings.general.items.lang.voice.items.en-us',
                        'ja-jp': 'settings.general.items.lang.voice.items.ja-jp',
                        'ko-kr': 'settings.general.items.lang.voice.items.ko-kr',
                        'zh-cn': 'settings.general.items.lang.voice.items.zh-cn'
                    }}
                    selectionUpdated={() => voiceUpdateRequired = true}
                />

                <SelectionBox
                    lang="settings.general.items.theme.title"
                    prop="theme"
                    items={{
                        'system': 'settings.general.items.theme.items.system',
                        'light': 'settings.general.items.theme.items.light',
                        'dark': 'settings.general.items.theme.items.dark'
                    }}
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
                        
                        Process.run(`"${path.addSlashes(await constants.paths.launcherDir)}/winetricks.sh"`, {
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
                        
                        Process.run(runner ? `"${path.addSlashes(`${runnerDir}/${runner.files.wine}`)}" "${path.addSlashes(`${runnerDir}/${runner.files.winecfg}`)}"` : 'winecfg', {
                            env: {
                                WINE: runner ? `${runnerDir}/${runner.files.wine}` : 'wine',
                                WINESERVER: runner ? `${runnerDir}/${runner.files.wineserver}` : 'wineserver',
                                WINEPREFIX: await constants.paths.prefix.current
                            }
                        });
                    }} />

                    <!-- svelte-ignore missing-declaration -->
                    <Button lang="settings.general.items.buttons.launcher" click={async () => {
                        Neutralino.os.execCommand(`xdg-open "${path.addSlashes(await constants.paths.launcherDir)}"`, {
                            background: true
                        });
                    }} />

                    <!-- svelte-ignore missing-declaration -->
                    <Button lang="settings.general.items.buttons.game" click={async () => {
                        Neutralino.os.execCommand(`xdg-open "${path.addSlashes(await constants.paths.gameDir)}"`, {
                            background: true
                        });
                    }} />

                    <!-- svelte-ignore missing-declaration -->
                    <Button
                        lang="settings.general.items.buttons.repair_game"
                        click={async () => {
                            await IPC.write('check-files-integrity');

                            Neutralino.app.exit();
                        }}
                    />
                </div>
            </div>

            <div class="settings-item" id="enhancements">
                <h1>{$_('settings.enhancements.title')}</h1>

                <h3>{$_('settings.enhancements.wine.title')}</h3>

                <SelectionBox
                    lang="settings.enhancements.wine.items.hud.title"
                    prop="hud"
                    items={{
                        'none': 'settings.enhancements.wine.items.hud.items.none',
                        'dxvk': 'settings.enhancements.wine.items.hud.items.dxvk',
                        'mangohud': 'settings.enhancements.wine.items.hud.items.mangohud'
                    }}
                />

                <SelectionBox
                    lang="settings.enhancements.wine.items.winesync.title"
                    prop="wine.sync"
                    tooltip="settings.enhancements.wine.items.winesync.tooltip"
                    tooltip_size="large"
                    items={{
                        'none': 'settings.enhancements.wine.items.winesync.items.none',
                        'esync': 'settings.enhancements.wine.items.winesync.items.esync',
                        'fsync': 'settings.enhancements.wine.items.winesync.items.fsync',
                        'futex2': 'settings.enhancements.wine.items.winesync.items.futex2'
                    }}
                />

                <Checkbox
                    lang="settings.enhancements.wine.items.fsr.title"
                    tooltip={{
                        lang: 'settings.enhancements.wine.items.fsr.tooltip'
                    }}
                    prop="wine.fsr"
                    disabled={winevdSettings['enabled'] || borderless_active}
                />

                <Checkbox
                    lang="settings.enhancements.wine.items.winevd.title"
                    prop="wine.virtual_desktop.enabled"
                    valueChanged={(value) => winevdSettings['enabled'] = value}
                />

                <WineVDSettings visible={winevdSettings['enabled']} valueChanged={handleWineVD} />

                <h3>{$_('settings.enhancements.game.title')}</h3>

                <Checkbox
                    lang="settings.enhancements.game.items.gamemode.title"
                    prop="gamemode"
                    tooltip={{
                        lang: gamemode.tooltip
                    }}
                    disabled={gamemode.disabled}
                />

                <Checkbox
                    lang="settings.enhancements.game.items.borderless_window.title"
                    tooltip={{
                        lang: 'settings.enhancements.game.items.borderless_window.tooltip'
                    }}
                    prop="borderless_window"
                    valueChanged={(value) => borderless_active = value}
                />

                <Checkbox
                    lang="settings.enhancements.game.items.fps_unlocker.title"
                    tooltip={{
                        lang: 'settings.enhancements.game.items.fps_unlocker.tooltip'
                    }}
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
                    lang="settings.enhancements.game.items.use_terminal.title"
                    tooltip={{
                        lang: 'settings.enhancements.game.items.use_terminal.tooltip'
                    }}
                    prop="use_terminal"
                />

                <h3>{$_('settings.enhancements.launcher.title')}</h3>

                <Checkbox
                    lang="settings.enhancements.launcher.items.purge_logs.game.title"
                    tooltip={{
                        lang: 'settings.enhancements.launcher.items.purge_logs.game.tooltip'
                    }}
                    prop="purge_logs.game"
                />

                <SelectionBox
                    lang="settings.enhancements.launcher.items.purge_logs.launcher.title"
                    tooltip="settings.enhancements.launcher.items.purge_logs.launcher.tooltip"
                    prop="purge_logs.launcher"
                    items={{
                        '1d': 'settings.enhancements.launcher.items.purge_logs.launcher.items.1d',
                        '3d': 'settings.enhancements.launcher.items.purge_logs.launcher.items.3d',
                        '5d': 'settings.enhancements.launcher.items.purge_logs.launcher.items.5d',
                        '7d': 'settings.enhancements.launcher.items.purge_logs.launcher.items.7d',
                        '14d': 'settings.enhancements.launcher.items.purge_logs.launcher.items.14d',
                        'never': 'settings.enhancements.launcher.items.purge_logs.launcher.items.never'
                    }}
                />
            </div>

            <div class="settings-item" id="runners">
                <h1>{$_('settings.runners.title')}</h1>

                <Checkbox
                    lang="settings.runners.items.recommended.title"
                    tooltip={{
                        lang: 'settings.runners.items.recommended.tooltip'
                    }}
                    valueChanged={(value) => runnersRecommendable = value}
                />

                <RunnerSelectionList recommendable={runnersRecommendable} />
            </div>

            <div class="settings-item" id="dxvks">
                <h1>{$_('settings.dxvks.title')}</h1>

                <Checkbox
                    lang="settings.dxvks.items.recommended.title"
                    tooltip={{
                        lang: 'settings.dxvks.items.recommended.tooltip'
                    }}
                    valueChanged={(value) => dxvkRecommendable = value}
                />

                <br><br>

                <DXVKSelectionList recommendable={dxvkRecommendable} />
            </div>

            <div class="settings-item" id="shaders">
                <h1>{$_('settings.shaders.title')}</h1>

                <ShadersSelection />
            </div>

            <div class="settings-item" id="environment">
                <h1>{$_('settings.environment.title')}</h1>

                <EnvironmentManager />
            </div>

            <div class="settings-item" id="patch">
                <h1>{$_('settings.patch.title')}</h1>

                {#if patchInfo !== null}
                    <div class="patch-version">
                        {$_('settings.patch.items.patch_version')}
                        
                        <span class:warning={!patchInfo.applied}>
                            { `${patchInfo.version} ${patchInfo.state}` }
                        </span>
                    </div>

                    <div style="margin-top: 24px">
                        {#if patchInfo.applied}
                            <!-- svelte-ignore missing-declaration -->
                            <Button
                                lang="settings.patch.items.buttons.revert_patch"
                                click={async () => {
                                    if (patchInfo)
                                    {
                                        const prevPatchInfo = patchInfo;

                                        patchInfo = null;

                                        patchInfo = await Patch.revert(prevPatchInfo) ?
                                            await Patch.latest : prevPatchInfo;
                                    }
                                }}
                            />

                            <!-- svelte-ignore missing-declaration -->
                            <Button
                                lang="settings.patch.items.buttons.reapply_patch"
                                click={async () => {
                                    if (patchInfo)
                                    {
                                        const prevPatchInfo = patchInfo;

                                        patchInfo = null;

                                        if (await Patch.revert(prevPatchInfo))
                                        {
                                            await IPC.write('update-state');

                                            Neutralino.app.exit();
                                        }

                                        else patchInfo = prevPatchInfo;
                                    }
                                }}
                            />
                        {:else}
                            <!-- svelte-ignore missing-declaration -->
                            <Button
                                lang="settings.patch.items.buttons.apply_patch"
                                click={async () => {
                                    await IPC.write('update-state');

                                    Neutralino.app.exit();
                                }}
                            />
                        {/if}
                    </div>
                {:else}
                    <p>Updating patch info...</p>
                {/if}
            </div>

            <div class="settings-footer">
                <span>An Anime Game Launcher {Launcher.version}</span>

                <!-- svelte-ignore missing-declaration -->
                <span>Licensed under <u on:click={() => Neutralino.os.open('https://www.gnu.org/licenses/gpl-3.0.en.html')}>GNU GPL-3.0</u></span>

                <br>

                <!-- svelte-ignore missing-declaration -->
                <span><u on:click={() => Neutralino.os.open(constants.uri.launcher)}>GitLab</u></span>

                <!-- svelte-ignore missing-declaration -->
                <span><u on:click={() => Neutralino.os.open(constants.uri.discord)}>Discord</u></span>

                <!-- svelte-ignore missing-declaration -->
                <span><u on:click={() => Neutralino.os.open(constants.uri.patch.origin)}>Patch repository</u></span>
            </div>
        </div>
    </main>
{/if}
