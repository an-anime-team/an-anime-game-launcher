<script context="module" lang="ts">
    declare const Neutralino;
</script>

<script lang="ts">
    import { onMount } from 'svelte';
    import { _, locale } from 'svelte-i18n';

    import Window from './ts/neutralino/Window';
    import Process from './ts/neutralino/Process';

    import Launcher from './ts/Launcher';
    import constants from './ts/Constants';
    import Game from './ts/Game';
    import Background from './ts/launcher/Background';
    import Archive from './ts/core/Archive';
    import Debug from './ts/core/Debug';
    import Downloader from './ts/core/Downloader';
    import IPC from './ts/core/IPC';
    import Configs from './ts/Configs';

    import Gear from './assets/images/gear.png';
    import GearActive from './assets/images/gear-active.png';
    import Download from './assets/images/cloud-download.png';

    constants.paths.launcherDir.then((dir) => {
        // Create launcher folder if it doesn't exist
        Neutralino.filesystem.getStats(dir)
            .catch(() => Neutralino.filesystem.createDirectory(dir));

        // Create logs folder if it doesn't exist
        Neutralino.filesystem.getStats(`${dir}/logs`)
            .catch(() => Neutralino.filesystem.createDirectory(`${dir}/logs`));
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

    Neutralino.events.on('windowClose', async () => {
        Downloader.closeStreams(true);
        Archive.closeStreams(true);

        const tempDir = await constants.paths.tempDir;
        const launcherDir = await constants.paths.launcherDir;

        // Remove IPC file
        await IPC.purge();

        // Turn off Discord RPC
        if (launcher.rpc)
            await launcher.rpc.stop(true);

        // Remove .tmp files from the temp folder
        await Neutralino.os.execCommand(`find "${Process.addSlashes(tempDir)}" -maxdepth 1 -type f -name "*.tmp" -delete`);

        // Remove old launcher's log files
        const purge_logs = await Configs.get('purge_logs.launcher') as string|null;

        if (purge_logs !== null && purge_logs[purge_logs.length - 1] == 'd')
            await Neutralino.os.execCommand(`find "${Process.addSlashes(launcherDir)}/logs" -maxdepth 1 -mtime ${purge_logs.substring(0, purge_logs.length - 1)} -delete`);

        // Save logs
        const log = Debug.get().join('\r\n');

        if (log != '')
            await Neutralino.filesystem.writeFile(`${launcherDir}/logs/${Debug.startedAt.getDate()}-${Debug.startedAt.getMonth() + 1}-${Debug.startedAt.getFullYear()}-${Debug.startedAt.getHours()}-${Debug.startedAt.getMinutes()}-${Debug.startedAt.getSeconds()}.log`, log);

        // And close the launcher when they was saved
        Neutralino.app.exit();
    });

    // Save logs
    let logSavingStarted = false;

    Debug.handler(() => {
        if (!logSavingStarted)
        {
            logSavingStarted = true;

            setTimeout(async () => {
                const log = `=== Log can be incomplete ===\r\n\r\n${Debug.get().join('\r\n')}`;

                if (log != '')
                    await Neutralino.filesystem.writeFile(`${await constants.paths.launcherDir}/logs/${Debug.startedAt.getDate()}-${Debug.startedAt.getMonth() + 1}-${Debug.startedAt.getFullYear()}-${Debug.startedAt.getHours()}-${Debug.startedAt.getMinutes()}-${Debug.startedAt.getSeconds()}.log`, log);

                logSavingStarted = false;
            }, 5000);
        }
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
        <img class="background" src={uri} alt="">
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
            <iframe id="social-iframe" src={uri} title="" scrolling="no" style="position: absolute; border: 0; top: 0; left: 0;" width="100%" height="100%"></iframe>
        {/await}
    </div>

    <div id="settings">
        <img src={Gear} class="unactive" alt="Settings">

        <img src={GearActive} class="active" alt="Settings">
    </div>
    
    <button class="button hint--left hint--small" aria-label="{typeof $locale === 'string' ? $_('launcher.states.ready.predownload') : ''}" id="predownload">
        <img src={Download} alt="Download" />
    </button>

    <button class="button hint--top hint--large" aria-label="" id="launch">Launch</button>
</main>
