<script context="module" lang="ts">
    declare const Neutralino;
</script>

<script lang="ts">
    import { onMount } from 'svelte';

    import { Configs, Windows, path } from './empathize';

    import Gallery from 'svelte-gallery';
    import Image from './components/Image.svelte';

    import constants from './ts/Constants';

    function encodeBase64Bytes(bytes: Uint8Array): string {
        return btoa(
            bytes.reduce((acc, current) => acc + String.fromCharCode(current), "")
        );
    }

    const images = [
        { src: 'https://source.unsplash.com/random', width: 600, height: 400 }
    ];

    onMount(async () => {
        await Windows.current.show();
        await Windows.current.center(900, 600);
        
        // Auto theme switcher
        Configs.get('theme').then((theme) => {
            if (theme === 'system')
                theme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';

            document.body.setAttribute('data-theme', theme as string);
        });

        Neutralino.filesystem.readBinaryFile(await constants.paths.gameDir + '/ScreenShot/20220317130153.png').then(async img => {
            let uintdata = new Uint8Array(img);
            images.push({
                src: 'data:image/png;base64,' + encodeBase64Bytes(uintdata),
                // @ts-expect-error
                orginialURI: await constants.paths.gameDir + '/ScreenShot/20220317130153.png',
                width: 600,
                height: 400,
                click: async (e) => {
                    Neutralino.os.execCommand(`xdg-open "${path.addSlashes(e.target.getAttribute('data-originalURI'))}"`, {
                        background: true
                    });
                }
            });
        });
    });
</script>

<main>
    <Gallery imageComponent={Image} {images} />
</main>