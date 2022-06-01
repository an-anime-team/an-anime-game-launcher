<script context="module" lang="ts">
    declare const Neutralino;
    declare const NL_ARGS: string[];
</script>

<script lang="ts">
    import { onMount } from 'svelte';
    import { _, locale } from 'svelte-i18n';

    import { Windows, path, Archive, Debug, Downloader, IPC, Configs, promisify } from './empathize';

    import { version } from '../package.json';

    import Launcher from './ts/Launcher';
    import constants from './ts/Constants';
    import Game from './ts/Game';

    import GearIcon from './assets/images/gear.png';
    import GearActiveIcon from './assets/images/gear-active.png';
    import ScreenshotsIcon from './assets/images/camera.png';
    import ScreenshotsActiveIcon from './assets/images/camera-active.png';
    import DownloadIcon from './assets/images/cloud-download.png';

    promisify(async () => {
        Debug.log([
            'An Anime Game Launcher',
            `Version: ${version}`,
            `Sandboxed (flatpak): ${await Launcher.isFlatpak() ? 'yes' : 'no'}`
        ].join('\r\n - '));
    });

    // Steam Deck users asked me to add something like that
    if (NL_ARGS.includes('--run-game'))
    {
        import('./ts/launcher/states/Launch').then((module) => {
            module.default(null).then(() => {
                Neutralino.app.exit();
            });
        });
    }

    // Otherwise just open the launcher as always
    else
    {
        const launcher = new Launcher(onMount);

        const getLogFilename = (date: Date = Debug.startedAt) => {
            const prefixZero = (num: number) => num < 10 ? `0${num}` : num;

            return `${date.getFullYear()}-${prefixZero(date.getMonth() + 1)}-${prefixZero(date.getDate())}-${prefixZero(date.getHours())}-${prefixZero(date.getMinutes())}-${prefixZero(date.getSeconds())}.log`;
        };

        constants.paths.launcherDir.then((launcherDir) => {
            Neutralino.filesystem.getStats(`${launcherDir}/logs/latest.log`)
                .then(async () => {
                    let created_at = (await Neutralino.os.execCommand(`stat -c '%W' "${path.addSlashes(`${launcherDir}/logs/latest.log`)}"`)).stdOut;

                    if (!created_at)
                        created_at = Date.now() / 1000;

                    Neutralino.filesystem.moveFile(`${launcherDir}/logs/latest.log`, `${launcherDir}/logs/${getLogFilename(new Date(created_at * 1000))}`);
                })
                .catch(() => {});
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
            await Neutralino.os.execCommand(`find "${path.addSlashes(tempDir)}" -maxdepth 1 -type f -name "*.tmp" -delete`);

            // Remove old launcher's log files
            const purge_logs = await Configs.get('purge_logs.launcher') as string|null;

            if (purge_logs !== null && purge_logs[purge_logs.length - 1] == 'd')
                await Neutralino.os.execCommand(`find "${path.addSlashes(launcherDir)}/logs" -maxdepth 1 -mtime +${purge_logs.substring(0, purge_logs.length - 1)} -delete`);

            // Save logs
            const log = Debug.get().join('\r\n');

            if (log != '')
                await Neutralino.filesystem.writeFile(`${launcherDir}/logs/latest.log`, log);

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

                    if (log.length > 35)
                        await Neutralino.filesystem.writeFile(`${await constants.paths.launcherDir}/logs/latest.log`, log);

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
                Windows.current.setTitle(`An Anime Game Launcher - ${game.version}`);
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

            const screenshotsButton = document.getElementById('screenshots');

            screenshotsButton!.onclick = () => launcher.showScreenshots();

            screenshotsButton!.onmouseenter = () => {
                screenshotsButton?.classList.add('hovered');
            };

            screenshotsButton!.onmouseleave = () => {
                screenshotsButton?.classList.remove('hovered');
            };
        });
    }
</script>

<main>
    <img id="background" src="" alt="">

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
        <iframe id="social-iframe" src="" title="" scrolling="no" style="position: absolute; border: 0; top: 0; left: 0;" width="100%" height="100%"></iframe>
    </div>

    <div id="screenshots">
        <img src={ScreenshotsIcon} class="unactive" alt="Screenshots">

        <img src={ScreenshotsActiveIcon} class="active" alt="Screenshots">
    </div>

    <div id="settings">
        <img src={GearIcon} class="unactive" alt="Settings">

        <img src={GearActiveIcon} class="active" alt="Settings">
    </div>

    <button class="button hint--left hint--small" aria-label="{typeof $locale === 'string' ? $_('launcher.states.ready.predownload') : ''}" id="predownload">
        <img src={DownloadIcon} alt="Download" />
    </button>

    <button class="button hint--top hint--large" aria-label="" id="launch">Launch</button>
    <button class="button" id="pause">Pause</button>
</main>
