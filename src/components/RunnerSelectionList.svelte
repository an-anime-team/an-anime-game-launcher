<script lang="ts">
    import { _ } from 'svelte-i18n';

    import Runners from '../ts/core/Runners';

    import type { Runner, RunnerFamily } from '../ts/types/Runners';
    
    let runners: RunnerFamily[] = [], selectedVersion;

    Runners.list().then((list) => runners = list);
    Runners.current.then((current) => selectedVersion = current?.name);

    import Delete from '../assets/images/delete.png';
    import Download from '../assets/images/download.png';

    const runnerInstalled = (runner: Runner): boolean => {
        for (const family of runners)
            for (const wine of family.runners)
                if (wine.name === runner.name)
                    return wine.installed;

        return false;
    };
</script>

<div class="list">
    {#each runners as family}
        <h2>{ family.title }</h2>

        {#each family.runners as runner}
            <div class="list-item" class:list-item-downloaded={runnerInstalled(runner)} class:list-item-active={runner.name === selectedVersion}>
                { runner.title }

                <div>
                    <span></span>

                    <!-- svelte-ignore a11y-missing-attribute -->
                    <img class="item-delete" src={Delete}>

                    <!-- svelte-ignore a11y-missing-attribute -->
                    <img class="item-download" src={Download}>
                </div>
            </div>
        {/each}
    {/each}
</div>
