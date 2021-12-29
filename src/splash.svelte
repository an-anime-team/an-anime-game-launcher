<script lang="ts">
    import { onMount } from 'svelte';
    import { _, locale } from 'svelte-i18n';

    import Configs from './ts/Configs';

    import Window from './ts/neutralino/Window';

    import Splash from './assets/gifs/running-qiqi.gif';
    import SplashSecret from './assets/gifs/loading-marie-please.gif';

    const splash = Math.round(Math.random() * 100) < 100 ? Splash : SplashSecret;

    let phrase = Math.round(Math.random() * 2);

    onMount(() => {
        Window.current.show();
    });

    const isLauncherLoaded = () => {
        // @ts-expect-error
        Neutralino.storage.getData('launcherLoaded')
            .then(() => {
                // @ts-expect-error
                Neutralino.storage.setData('launcherLoaded', undefined);

                Window.current.hide();

                // @ts-expect-error
                Neutralino.app.exit();
            })
            .catch(() => setTimeout(isLauncherLoaded, 1000));
    };

    // @ts-expect-error
    Neutralino.events.on('ready', () => setTimeout(isLauncherLoaded, 1000));

    // @ts-expect-error
    Neutralino.events.on('windowClose', () => {
        // @ts-expect-error
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
        <p>{$_(`splash.phrases.${phrase}`)}</p>
    </main>
{/if}
