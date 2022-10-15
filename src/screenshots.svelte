<script lang="ts">
    import { onMount } from 'svelte';
    import { _, locale } from 'svelte-i18n';

    import { Configs, Windows, path, fs } from './empathize';
    import type { Entry } from '@empathize/framework/dist/fs/fs';

    import constants from './ts/Constants';

    import Button from './components/Button.svelte';

    import LoaderImage from './assets/gifs/loading-marie-please.gif';

    type Image = {
        src: string,
        path: string
    };

    type RequestedImages = {
        images: Promise<Image[]>,
        hasMore: boolean
    };

    let totalImages: Entry[] = [];

    let images: Image[] = [];
    let columns: Image[][] = [[], [], []];

    let lastLoaded = 0;
    let loading = true;

    let can_load_more = true;

    const encodeBase64Bytes = (bytes: Uint8Array) => btoa(bytes.reduce((acc, current) => acc + String.fromCharCode(current), ''));

    const requestImages = (amount: number = 9): RequestedImages|null => {
        if (images.length == totalImages.length)
            return null;

        else return {
            images: new Promise(async (resolve) => {
                let newImages: Image[] = [];

                for (let i = 0; lastLoaded < totalImages.length && i < amount; ++lastLoaded, ++i)
                {
                    const imagePath = `${await constants.paths.gameDir}/ScreenShot/${totalImages[lastLoaded].name}`;
                    const binaryData = await Neutralino.filesystem.readBinaryFile(imagePath);

                    // TODO: optimize images resolution
                    newImages.push({
                        src: 'data:image/png;base64,' + encodeBase64Bytes(new Uint8Array(binaryData)),
                        path: imagePath
                    });
                }

                resolve(newImages);
            }),
            hasMore: (lastLoaded + amount + 1) < totalImages.length
        };
    };

    const updateColumns = (): Promise<void> => {
        return new Promise(async (resolve) => {
            const requestedImages = requestImages();

            if (requestedImages !== null)
            {
                loading = true;
                can_load_more = requestedImages.hasMore;

                const columnsElements = document.getElementsByClassName('column');

                for (const newImage of await requestedImages.images)
                {
                    columns[images.length % 3].push(newImage);

                    const newImg = document.createElement('img');
                    newImg.src = newImage.src;

                    newImg.onclick = () => {
                        Neutralino.os.execCommand(`xdg-open "${path.addSlashes(newImage.path)}"`, {
                            background: true
                        });
                    };

                    columnsElements[images.length % 3].appendChild(newImg);
                    
                    images.push(newImage);
                }

                // TODO: scroll to the (last - 2) element
                (columnsElements[0].lastChild as HTMLElement).onload = (e) => (e.target as HTMLElement).scrollIntoView();

                loading = false;
            }

            resolve();
        });
    };

    onMount(async () => {
        fs.files(`${await constants.paths.gameDir}/ScreenShot`).then(async (files) => {
            totalImages = files
                .filter((file) => file.type == 'file')
                .sort((a, b) => a.name < b.name ? 1 : (a.name > b.name ? -1 : 0));

            updateColumns();
        }).catch(() => loading = false);

        // Auto theme switcher
        Configs.get('theme').then((theme) => {
            if (theme === 'system')
                theme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';

            document.body.setAttribute('data-theme', theme as string);
        });

        await Windows.current.show();
        await Windows.current.center(900, 600);
    });
</script>

{#if typeof $locale === 'string'}
    <main>
        <h2>{$_('screenshots.heading')}</h2>

        <p>{$_('screenshots.info')}</p>

        <div class="images" style="display: {loading || images.length == 0 ? 'none' : 'flex'}">
            <div class="column"></div>
            <div class="column"></div>
            <div class="column"></div>
        </div>

        {#if loading}
            <img class="loader" src={LoaderImage} alt="">
        {:else if images.length == 0}
            <p>{$_('screenshots.no_images')}</p>
        {/if}

        <div class="buttons">
            {#if !loading && can_load_more}
                <Button lang="screenshots.buttons.more" click={() => {
                    updateColumns();
                }} />
            {/if}
            
            <!-- svelte-ignore missing-declaration -->
            <Button lang="screenshots.buttons.folder" click={async () => {
                Neutralino.os.execCommand(`xdg-open "${path.addSlashes(await constants.paths.gameDir + '/ScreenShot/')}"`, {
                    background: true
                });
            }} />
        </div>
    </main>
{/if}
