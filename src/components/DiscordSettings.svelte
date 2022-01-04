<script lang="ts">
    import { _ } from 'svelte-i18n';

    export let visible: boolean = false;
    export let valuesChanged: (values: { game: string, launcher: string }) => void = () => {};

    import Checkbox from './Checkbox.svelte';
</script>

<div style="display: {visible ? 'block' : 'none'}">
    <h3>{$_('settings.general.items.discord.settings.title')}</h3>

    <Checkbox lang="settings.general.items.discord.settings.items.timer" prop="discord.fields.timer" />

    <table class="table" style="margin-top: 16px">
        <tr>
            <td>
                <span>{$_('settings.general.items.discord.settings.items.in-launcher')}</span>
            </td>

            <td>
                <input value="Preparing to launch" on:input={(value) => console.log(value)}>
            </td>
        </tr>
        <tr>
            <td>
                <span>{$_('settings.general.items.discord.settings.items.in-game')}</span>
            </td>

            <td>
                <input value="Playing the game">
            </td>
        </tr>
    </table>
</div>
