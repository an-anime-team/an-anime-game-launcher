<script lang="ts">
    import { _, locale } from 'svelte-i18n';

    import type { Shader } from '../ts/types/Shaders';
    import Shaders from '../ts/launcher/Shaders';
    
    import SelectionBox from './SelectionBox.svelte';

    let shaders: Shader[] = [],
        shadersOptions = {
            'none': 'settings.shaders.items.shaders.items.none'
        };

    Shaders.list().then((list) => {
        shaders = list;

        for (const shader of list)
            shadersOptions[shader.folder] = shader.name;

        shadersOptions['custom'] = 'settings.shaders.items.shaders.items.custom';
    });
</script>

<div>
    <SelectionBox
        lang="settings.shaders.items.shaders.title"
        tooltip="settings.shaders.items.shaders.tooltip"
        prop="shaders"
        items={shadersOptions}
    />

    {#each shaders as shader}
        <h3>{shader.name}</h3>

        <p>{$_('settings.shaders.items.author', { values: { author: shader.author } })}</p>

        {#if !shader.images}
            <p>{$_('settings.shaders.items.no_images')}</p>
        {:else}
            {#each shader.images as image}
                {#await Shaders.getPicture(shader.folder, image.file) then picture}
                    <img class="shader-image" src={picture} alt="" />

                    <p class="shader-image-title">{$locale && image.caption[$locale] ? image.caption[$locale] : image.caption['en-us']}</p>
                {/await}
            {/each}
        {/if}
    {/each}
</div>

<style lang="sass">
    .shader-image
        width: 100%

    .shader-image-title
        text-align: center

        margin-top: 8px
</style>
