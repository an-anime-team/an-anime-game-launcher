<script lang="ts">
    import { _ } from 'svelte-i18n';

    export let recommendable = true;

    import DXVK from '../ts/core/DXVK';
    import constants from '../ts/Constants';

    import type {
        DXVK as TDXVK
    } from '../ts/types/DXVK';

    import Delete from '../assets/images/delete.png';
    import Download from '../assets/images/download.png';
    import Loading from '../assets/gifs/loading.gif';
    
    let dxvks: TDXVK[] = [],
        installedDxvks = {},
        disabledDxvks = {},
        selectedVersion;

    DXVK.list().then((list) => {
        dxvks = list;

        list.forEach((dxvk) => {
            installedDxvks[dxvk.version] = dxvk.installed;
            disabledDxvks[dxvk.version] = false;
        });
    });

    DXVK.current().then((current) => selectedVersion = current?.version);

    let progress = {}, applying = {};

    const downloadDxvk = (dxvk: TDXVK) => {
        DXVK.download(dxvk).then((stream) => {
            stream?.downloadStart(() => disabledDxvks[dxvk.version] = true);

            stream?.downloadProgress((current, total) => {
                progress[dxvk.version] = `${Math.round(current / total * 100)}%`;
            });

            stream?.unpackProgress((current, total) => {
                progress[dxvk.version] = `${Math.round(current / total * 100)}%`;
            });

            stream?.unpackFinish(async () => {
                installedDxvks[dxvk.version] = true;
                progress[dxvk.version] = undefined;

                applying[dxvk.version] = true;

                DXVK.current(dxvk);

                DXVK.apply(await constants.paths.prefix.current, dxvk).then(() => {
                    applying[dxvk.version] = false;
                    disabledDxvks[dxvk.version] = false;

                    selectedVersion = dxvk.version;
                });
            });
        });
    };

    const deleteDxvk = (dxvk: TDXVK) => {
        disabledDxvks[dxvk.version] = true;
        applying[dxvk.version] = true;

        DXVK.delete(dxvk).then(() => {
            installedDxvks[dxvk.version] = false;
            disabledDxvks[dxvk.version] = false;
            applying[dxvk.version] = false;
        });
    };

    const selectDxvk = async (dxvk: TDXVK) => {
        disabledDxvks[dxvk.version] = true;
        applying[dxvk.version] = true;

        DXVK.current(dxvk);

        DXVK.apply(await constants.paths.prefix.current, dxvk).then(() => {
            disabledDxvks[dxvk.version] = false;
            applying[dxvk.version] = false;

            selectedVersion = dxvk.version;
        });
    };
</script>

<div class="list">
    {#each dxvks as dxvk}
        <div
            class="list-item"
            class:list-item-downloaded={installedDxvks[dxvk.version]}
            class:list-item-active={dxvk.version === selectedVersion}
            class:list-item-hidden={recommendable && !dxvk.recommended}
            class:list-item-downloading={progress[dxvk.version]}
            class:list-item-applying={applying[dxvk.version]}
            class:list-item-disabled={disabledDxvks[dxvk.version]}

            on:click|self={() => {
                if (installedDxvks[dxvk.version] && selectedVersion !== dxvk.version)
                    selectDxvk(dxvk);
            }}
        >
            { dxvk.version }

            <div>
                <span>{progress[dxvk.version] ?? ''}</span>

                <!-- svelte-ignore a11y-missing-attribute -->
                <img class="item-loading" src={Loading}>

                <!-- svelte-ignore a11y-missing-attribute -->
                <img class="item-delete" src={Delete} on:click={() => deleteDxvk(dxvk)}>

                <!-- svelte-ignore a11y-missing-attribute -->
                <img class="item-download" src={Download} on:click={() => downloadDxvk(dxvk)}>
            </div>
        </div>
    {/each}
</div>
