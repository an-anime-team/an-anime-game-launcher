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

        list.forEach((shader) => {
            shadersOptions[shader.folder] = shader.name;
        });
    });
</script>

<SelectionBox
    lang="settings.shaders.items.shaders.title"
    prop="shaders"
    items={shadersOptions}
/>
