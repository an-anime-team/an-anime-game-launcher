<script context="module" lang="ts">
    declare const Neutralino;
</script>

<script lang="ts">
    import { onMount } from 'svelte';

    import { Configs, Windows } from './empathize';

    onMount(async () => {
        await Windows.current.show();
        await Windows.current.center(900, 600);
        
        // Auto theme switcher
        Configs.get('theme').then((theme) => {
            if (theme === 'system')
                theme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';

            document.body.setAttribute('data-theme', theme as string);
        });
    });
</script>
