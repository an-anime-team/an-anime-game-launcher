<script lang="ts">
    import { _ } from 'svelte-i18n';

    export let recommendable = true;

    import DXVK from '../ts/core/DXVK';

    import type {
        DXVK as TDXVK
    } from '../ts/types/DXVK';
    
    let dxvks: TDXVK[] = [], selectedVersion;

    DXVK.list().then((list) => dxvks = list);
    DXVK.current.then((current) => selectedVersion = current?.version);

    import Delete from '../assets/images/delete.png';
    import Download from '../assets/images/download.png';

    const dxvkInstalled = (dxvk: TDXVK): boolean => {
        const filtered = dxvks.filter((item) => item.version === dxvk.version);

        if (filtered.length === 1)
            return filtered[0].installed;

        else return false;
    };
</script>

<div class="list">
    {#each dxvks as dxvk}
        <div class="list-item" class:list-item-downloaded={dxvkInstalled(dxvk)} class:list-item-active={dxvk.version === selectedVersion} class:list-item-hidden={recommendable && !dxvk.recommended}>
            { dxvk.version }

            <div>
                <span></span>

                <!-- svelte-ignore a11y-missing-attribute -->
                <img class="item-delete" src={Delete}>

                <!-- svelte-ignore a11y-missing-attribute -->
                <img class="item-download" src={Download}>
            </div>
        </div>
    {/each}
</div>
