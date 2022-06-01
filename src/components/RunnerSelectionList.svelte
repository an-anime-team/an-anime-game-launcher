<script lang="ts">
    import { _ } from 'svelte-i18n';

    export let recommendable = true;

    import type { Runner, RunnerFamily } from '../ts/types/Runners';

    import Runners from '../ts/core/Runners';

    import Delete from '../assets/images/delete.png';
    import Download from '../assets/images/download.png';
    import Arrow from '../assets/svgs/arrow.svg';
    
    let runners: RunnerFamily[] = [],
        installedRunners = {},
        disabledRunners = {},
        openedFamily,
        selectedVersion;

    Runners.list().then((list) => {
        runners = list;

        for (const family of runners)
            for (const runner of family.runners)
            {
                installedRunners[runner.name] = runner.installed;
                disabledRunners[runner.name] = false;
            }
    });

    Runners.current().then((current) => {
        selectedVersion = current;

        if (current)
            openedFamily = current.family;
    });

    let progress = {}, applying = {};

    const downloadRunner = (runner: Runner) => {
        Runners.download(runner).then((stream) => {
            stream?.downloadStart(() => disabledRunners[runner.name] = true);

            stream?.downloadProgress((current, total) => {
                progress[runner.name] = `${Math.round(current / total * 100)}%`;
            });

            stream?.unpackProgress((current, total) => {
                progress[runner.name] = `${Math.round(current / total * 100)}%`;
            });

            stream?.unpackFinish(() => {
                installedRunners[runner.name] = true;
                disabledRunners[runner.name] = false;

                progress[runner.name] = undefined;

                selectedVersion = runner;

                Runners.current(runner);
            });
        });
    };

    const deleteRunner = (runner: Runner) => {
        disabledRunners[runner.name] = true;
        applying[runner.name] = true;

        Runners.delete(runner).then(() => {
            installedRunners[runner.name] = false;
            disabledRunners[runner.name] = false;
            applying[runner.name] = false;
        });
    };
</script>

<div class="list">
    {#each runners as family}
        <div
            class="list-title"
            class:list-title-open={openedFamily === family.title}
            on:click={() => openedFamily = openedFamily != family.title ? family.title : null}
        >
            <span>{family.title}</span>

            <img src={Arrow} alt="" />
        </div>

        {#if openedFamily === family.title}
            {#each family.runners as runner}
                <div
                    class="list-item"
                    class:list-item-downloaded={installedRunners[runner.name]}
                    class:list-item-active={runner.name === selectedVersion.name}
                    class:list-item-hidden={recommendable && !runner.recommended && runner.name !== selectedVersion.name}
                    class:list-item-downloading={progress[runner.name]}
                    class:list-item-applying={applying[runner.name]}
                    class:list-item-disabled={disabledRunners[runner.name]}

                    on:click|self={() => {
                        if (installedRunners[runner.name] && selectedVersion.name !== runner.name)
                        {
                            selectedVersion = runner;

                            Runners.current(runner);
                        }
                    }}
                >
                    { runner.title }

                    <div>
                        <span>{progress[runner.name] ?? ''}</span>

                        <!-- svelte-ignore a11y-missing-attribute -->
                        <img class="item-delete" src={Delete} on:click={() => deleteRunner(runner)}>

                        <!-- svelte-ignore a11y-missing-attribute -->
                        <img class="item-download" src={Download} on:click={() => downloadRunner(runner)}>
                    </div>
                </div>
            {/each}
        {/if}
    {/each}
</div>
