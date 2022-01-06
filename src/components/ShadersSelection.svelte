<script lang="ts">
    import { _ } from 'svelte-i18n';

    import type { Shader } from '../ts/types/Shaders';
    import Shaders from '../ts/core/Shaders';
    
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

<SelectionBox
    lang="settings.shaders.items.shaders.title"
    tooltip="settings.shaders.items.shaders.tooltip"
    prop="shaders"
    items={shadersOptions}
/>
