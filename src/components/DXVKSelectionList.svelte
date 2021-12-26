<script lang="ts">
    import { _ } from 'svelte-i18n';

    import DXVK from '../ts/core/DXVK';

    import type {
        DXVK as TDXVK
    } from '../ts/types/DXVK';
    
    let dxvks: TDXVK[] = [];

    DXVK.list().then((list) => dxvks = list);

    import Delete from '../assets/images/delete.png';
    import Download from '../assets/images/download.png';
</script>

<h2>{ $_('settings.general.items.dxvks') }</h2>

<div class="list">
    {#each dxvks as dxvk}
        <div class="list-item">
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
