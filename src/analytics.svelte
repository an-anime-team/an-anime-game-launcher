<script context="module" lang="ts">
    declare const Neutralino;
</script>

<script lang="ts">
    import { onMount } from 'svelte';
    import { _, locale } from 'svelte-i18n';

    import { Configs, Windows } from './empathize';

    onMount(() => {
        Windows.current.show();
        Windows.current.center();
    });

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
        
    </main>
{/if}
