<script context="module" lang="ts">
    declare const Neutralino;
</script>

<script lang="ts">
    import { onMount } from 'svelte';
    import { _, locale } from 'svelte-i18n';

    import Window from './ts/neutralino/Window';

    import Launcher from './ts/Launcher';
    import constants from './ts/Constants';
    import Game from './ts/Game';
    import Background from './ts/launcher/Background';
    import Archive from './ts/core/Archive';
    import Debug from './ts/core/Debug';
    import Downloader from './ts/core/Downloader';
    import IPC from './ts/core/IPC';

    import Gear from './assets/images/gear.png';
    import GearActive from './assets/images/gear-active.png';
    import Download from './assets/images/cloud-download.png';

    constants.paths.launcherDir.then((dir) => {
        Neutralino.filesystem.getStats(dir)
            .catch(() => Neutralino.filesystem.createDirectory(dir));
    });

    const launcher = new Launcher(onMount);

    Neutralino.events.on('ready', () => {
        Window.open('splash', {
            title: 'Splash',
            width: 300,
            height: 400,
            borderless: true,
            exitProcessOnClose: false
        });
    });

    Neutralino.events.on('windowClose', () => {
        Downloader.closeStreams(true);
        Archive.closeStreams(true);

        constants.paths.launcherDir.then(async (path) => {
            const time = new Date;

            await IPC.purge();

            if (launcher.rpc)
                await launcher.rpc.stop(true);

            Neutralino.filesystem.getStats(`${path}/logs`)
                .then(() => saveLog())
                .catch(async () => {
                    await Neutralino.filesystem.createDirectory(`${path}/logs`);

                    saveLog();
                });

            const saveLog = async () => {
                const log = Debug.get().join("\r\n");

                if (log != '')
                    await Neutralino.filesystem.writeFile(`${path}/logs/${time.getDate()}-${time.getMonth() + 1}-${time.getFullYear()}-${time.getHours()}-${time.getMinutes()}-${time.getSeconds()}.log`, log);

                Neutralino.app.exit();
            };
        });
    });

    // Do some stuff when all the content will be loaded
    onMount(() => {
        /**
         * Update launcher's title
         */
        Game.latest.then((game) => {
            Window.current.setTitle(`${constants.placeholders.uppercase.full} Linux Launcher - ${game.version} (beta revision)`);
        });

        /**
         * Add some events to some elements
         */
        const settingsButton = document.getElementById('settings');

        settingsButton!.onclick = () => launcher.showSettings();

        settingsButton!.onmouseenter = () => {
            settingsButton?.classList.add('hovered');
        };

        settingsButton!.onmouseleave = () => {
            settingsButton?.classList.remove('hovered');
        };
    });
</script>

<main>
    {#await Background.get() then uri}
        <img class="background" src="{uri}" alt="">
    {/await}

    <div class="downloader-panel" data-theme="light">
        <div class="downloader-label">
            <span id="downloaded"></span>
            <span id="speed"></span>
            <span id="eta"></span>
        </div>

        <div class="progress-bar">
            <div class="progress"></div>
        </div>
    </div>

    <div id="launcher-content">
        {#await launcher.getSocial() then uri}
            <iframe title="Launcher-iframe" src="{uri}" scrolling="no" style="position: absolute; border: 0; top: 0; left: 0;" width="100%" height="100%"></iframe>
        {/await}
    </div>

    <div id="settings">
        <img src={Gear} class="unactive" alt="Settings">

        <img src={GearActive} class="active" alt="Settings">
    </div>
    
    <button class="button hint--left hint--small" aria-label="{typeof $locale === 'string' ? $_('launcher.predownload') : ''}" id="predownload">
        <img src={Download} alt="Download" />
    </button>

    <button class="button hint--top hint--large" aria-label="" id="launch">Launch</button>
</main>
