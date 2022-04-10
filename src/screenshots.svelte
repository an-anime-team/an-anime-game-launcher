<script context="module" lang="ts">
    declare const Neutralino;
</script>

<script lang="ts">
    import { onMount } from 'svelte';
    import { _, locale } from 'svelte-i18n';

    import { Configs, Windows, path } from './empathize';

    import Gallery from 'svelte-gallery';
    import Image from './components/Image.svelte';
    import Button from './components/Button.svelte';

    import constants from './ts/Constants';

    function encodeBase64Bytes(bytes: Uint8Array): string {
        return btoa(
            bytes.reduce((acc, current) => acc + String.fromCharCode(current), "")
        );
    };

    //@ts-expect-error
    String.prototype.insertAt = function(index, str){
        return this.slice(0,index) + str + this.slice(index);
    };

    const images = new Array();

    // Svelte did a haha funny and didn't want to fucking bother working with the images array so use this as an jank alternative.
    let arrayempty = true;

    onMount(async () => {
        // This exists as you can't make an for loop asynchronus causing the window to load before every image is loaded into the HTML content.
        const fakewait = () => new Promise(resolve =>
            setTimeout(() => resolve(true), 500)
        );

        Neutralino.filesystem.readDirectory(await constants.paths.gameDir + '/ScreenShot').then(async imgs => {
            // Filter by FILES and sort by latest to oldest
            const sortedimgs = imgs.filter(img => img.type == "FILE")
                .sort(function(x, y){
                    let f1date = x.entry.replace('.png', '').insertAt(4, '-').insertAt(7, '-').substring(0, 10);
                    let f2date = y.entry.replace('.png', '').insertAt(4, '-').insertAt(7, '-').substring(0, 10);
                    const f1time = new Date(f1date).getTime();
                    const f2time = new Date(f2date).getTime();
                    return f2time - f1time;
                });
            
            if (sortedimgs.length > 0) {
                // Only let 4 items be in the array by modifying the length which is quicker than splicing.
                sortedimgs.length = 4;
                arrayempty = false;
            }

            sortedimgs.forEach(async img => {
                Neutralino.filesystem.readBinaryFile(await constants.paths.gameDir + '/ScreenShot/' + img.entry).then(async img => {
                    let uintdata = new Uint8Array(img);
                    
                    images.push({
                        src: 'data:image/png;base64,' + encodeBase64Bytes(uintdata),
                        orignialURI: await constants.paths.gameDir + '/ScreenShot/' + img.entry,
                        width: 480,
                        height: 270,
                        click: async (e) => {
                            Neutralino.os.execCommand(`xdg-open "${path.addSlashes(e.target.getAttribute('data-originalURI'))}"`, {
                                background: true
                            });
                        }
                    });
                });
            });

            await fakewait();

            await Windows.current.show();
            await Windows.current.center(900, 600);

            // Auto theme switcher
            Configs.get('theme').then((theme) => {
                if (theme === 'system')
                    theme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';

                document.body.setAttribute('data-theme', theme as string);
            });
        });
    });

    Neutralino.events.on('windowClose', async () => {
        Neutralino.app.exit();
    });
</script>

{#if typeof $locale === 'string'}
    <main>
        <h2 class="centtext">{$_('screenshots.heading')}</h2>

        <p class="centtext">{$_('screenshots.info.0')}</p>
        <p class="centtext">{$_('screenshots.info.1')}</p>

        {#if !arrayempty}
            <div class="galleryholder">
                <Gallery imageComponent={Image} gutter={6} {images} />
            </div>
        {:else}
            <p class="centtext">{$_('screenshots.noimages')}</p>
        {/if}

        <div class="buttons">
            <!-- svelte-ignore missing-declaration -->
            <Button lang="screenshots.button" click={async () => {
                Neutralino.os.execCommand(`xdg-open "${path.addSlashes(await constants.paths.gameDir + '/ScreenShot/')}"`, {
                    background: true
                });
            }} />
        </div>
    </main>
{/if}