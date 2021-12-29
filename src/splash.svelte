<script lang="ts">
    import { onMount } from 'svelte';
    import { _, locale } from 'svelte-i18n';

    import Window from './ts/neutralino/Window';

    import Splash from './assets/webms/loading.webm';

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
    // TODO: an option to disable it
    if (window.matchMedia('(prefers-color-scheme: dark)').matches)
        document.body.setAttribute('data-theme', 'dark');
</script>

{#if typeof $locale === 'string'}
    <main>
        <video src={Splash} loop muted autoplay></video>

        <h2>{$_('splash.title')}</h2>
        <p>{$_(`splash.phrases.${phrase}`)}</p>
    </main>
{/if}
