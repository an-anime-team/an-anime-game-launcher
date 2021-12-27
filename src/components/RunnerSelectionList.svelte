<script lang="ts">
    import { _ } from 'svelte-i18n';

    export let recommendable = true;

    import Runners from '../ts/core/Runners';

    import type { Runner, RunnerFamily } from '../ts/types/Runners';

    import Delete from '../assets/images/delete.png';
    import Download from '../assets/images/download.png';
    
    let runners: RunnerFamily[] = [],
        installedRunners = {},
        disabledRunners = {},
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

    Runners.current().then((current) => selectedVersion = current?.name);

    let progress = {};

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
            });
        });
    };

    const deleteRunner = (runner: Runner) => {
        disabledRunners[runner.name] = true;

        Runners.delete(runner).then(() => {
            installedRunners[runner.name] = false;
            disabledRunners[runner.name] = false;
        });
    };
</script>

<div class="list">
    {#each runners as family}
        <h2>{ family.title }</h2>

        {#each family.runners as runner}
            <div
                class="list-item"
                class:list-item-downloaded={installedRunners[runner.name]}
                class:list-item-active={runner.name === selectedVersion}
                class:list-item-hidden={recommendable && !runner.recommended}
                class:list-item-downloading={progress[runner.name] !== undefined}
                class:list-item-disabled={disabledRunners[runner.name]}
                on:click={() => {
                    if (installedRunners[runner.name])
                    {
                        selectedVersion = runner.name;

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
    {/each}
</div>
