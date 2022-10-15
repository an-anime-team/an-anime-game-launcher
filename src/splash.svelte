<script context="module" lang="ts">
    declare const Neutralino;
</script>

<script lang="ts">
    import { onMount } from 'svelte';
    import { _, locale } from 'svelte-i18n';

    import { Configs, IPC, Windows } from './empathize';

    import constants from './ts/Constants';

    import Splash from './assets/gifs/running-qiqi.gif';
    import SplashSecret from './assets/gifs/loading-marie-please.gif';

    const splash = Math.round(Math.random() * 100) < 100 ? Splash : SplashSecret;

    let phrase = Math.round(Math.random() * 8);

    onMount(() => {
        Windows.current.show();
        Windows.current.center(300, 400);
    });

    const isLauncherLoaded = () => {
        IPC.read().then(async (records) => {
            const launcherLoaded = records.filter((record) => record.data === 'launcher-loaded');

            if (launcherLoaded.length > 0)
            {
                for (const record of launcherLoaded)
                    await record.pop();

                Windows.current.hide();
                Neutralino.app.exit();
            }

            else setTimeout(isLauncherLoaded, 500);
        });
    };

    Neutralino.events.on('ready', () => isLauncherLoaded());

    Neutralino.events.on('windowClose', () => {
        Neutralino.app.exit();
    });

    // Auto theme switcher
    Configs.get('theme').then((theme) => {
        if (theme === 'system')
            theme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';

        document.body.setAttribute('data-theme', theme as string);
    });
</script>

{#if typeof $locale === 'string'}
    <main>
        <img src={splash} alt="" />

        <h2>{$_('splash.title')}</h2>
        <p>{$_(`splash.phrases.${phrase}`, {
            values: {
                // Required by de-de locale
                game: constants.placeholders.uppercase.full.global
            }
        })}</p>
    </main>
{/if}
