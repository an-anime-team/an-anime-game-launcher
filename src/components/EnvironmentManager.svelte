<script lang="ts">
    import { _ } from 'svelte-i18n';

    import Configs from '../ts/Configs';

    import Button from './Button.svelte';

    let last_id = 0, variables = {}, selected;

    Configs.get('env').then((env) => {
        for (const key of Object.keys(env as object))
        {
            variables[last_id++] = {
                key: key,
                value: env![key]
            };
        }
    });

    const updateEnv = () => {
        let env = {};

        for (const key of Object.keys(variables))
            if (variables[key].key && variables[key].value)
                env[variables[key].key] = variables[key].value;

        Configs.set('env', env);
    };
</script>

<div>
    <table class="table" style="margin-top: 16px">
        <tr>
            <th>{$_('settings.environment.items.table.name')}</th>
            <th>{$_('settings.environment.items.table.value')}</th>
        </tr>

        {#each Object.keys(variables) as key}
            <tr on:click={() => selected = key} class:selected={selected === key}>
                <td>
                    <span>{variables[key].key}</span>
                    <input bind:value={variables[key].key} on:change={updateEnv} />
                </td>
        
                <td>
                    <span>{variables[key].value}</span>
                    <input bind:value={variables[key].value} on:change={updateEnv} />
                </td>
            </tr>
        {/each}
    </table>

    <div style="margin-top: 16px">
        <Button lang="settings.environment.items.buttons.add" click={() => variables[last_id++] = { key: '', value: '' }} />
        
        <Button lang="settings.environment.items.buttons.delete" click={() => {
            delete variables[selected];

            selected = undefined;

            updateEnv();
        }} />
    </div>
</div>
