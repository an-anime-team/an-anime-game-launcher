<script lang="ts">
    import { onMount } from 'svelte';

    import Gear from '/src/assets/images/gear.png';
    import GearActive from '/src/assets/images/gear-active.png';

    import Window from './ts/neutralino/Window';

    import Launcher from './ts/Launcher';
    import constants from './ts/Constants';
    import Game from './ts/Game';
    import Background from './ts/launcher/Background';

    // @ts-expect-error
    Neutralino.events.on('ready', () => {
        Window.open('splash', {
            title: 'Splash',
            width: 400,
            height: 500,
            borderless: true,
            exitProcessOnClose: false
        });
    });

    const launcher = new Launcher(onMount);

    // Do some stuff when all the content will be loaded
    onMount(() => {
        /**
         * Update launcher's title
         */
        Game.latest.then((game) => {
            Window.current.setTitle(`${constants.placeholders.uppercase.full} Linux Launcher - ${game.version}`);
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

    // Auto theme switcher
    // TODO: an option to disable it
    if (window.matchMedia('(prefers-color-scheme: dark)').matches)
        document.body.setAttribute('data-theme', 'dark');
</script>

<main>
    {#await Background.get() then uri}
        <img class="background" src="{uri}" alt="Missing Texture"> <!-- Alt: Random GMOD reference -->
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
    
    <button class="button" id="launch">Launch</button>
</main>
