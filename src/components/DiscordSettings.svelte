<script lang="ts">
    import { _ } from 'svelte-i18n';

    import Configs from '../ts/Configs';

    export let visible: boolean = false;
    export let valueChanged: (field: 'in-game' | 'in-launcher', value: string) => void = () => {};

    import Checkbox from './Checkbox.svelte';

    let states = {
        'in-game': '',
        'in-launcher': ''
    };

    Configs.get('discord.states').then((settings) => {
        states = {
            'in-game': settings!['in-game']['details'],
            'in-launcher': settings!['in-launcher']['details']
        };

        if (settings!['in-game']['state'] != '')
            states['in-game'] += `\n${settings!['in-game']['state']}`;

        if (settings!['in-launcher']['state'] != '')
            states['in-launcher'] += `\n${settings!['in-launcher']['state']}`;
    });

    const textareaHandler = (event: KeyboardEvent, field: 'in-game' | 'in-launcher') => {
        const textArea = event.srcElement as HTMLTextAreaElement;
        const content = textArea.value;

        if (event.key === 'Enter')
        {
            if (content.split('\n').length > 2)
                textArea.value = content.substring(0, content.length - 1);
        }

        else valueChanged(field, content.trim());
    };
</script>

<div style="display: {visible ? 'block' : 'none'}">
    <h3>{$_('settings.general.items.discord.settings.title')}</h3>

    <Checkbox lang="settings.general.items.discord.settings.items.timer" prop="discord.timer" />

    <table class="table" style="margin-top: 16px">
        <tr>
            <td>
                <span>{$_('settings.general.items.discord.settings.items.in-launcher')}</span>
            </td>

            <td>
                <textarea rows="2" on:keyup={(e) => textareaHandler(e, 'in-launcher')}>{states['in-launcher']}</textarea>
            </td>
        </tr>
        <tr>
            <td>
                <span>{$_('settings.general.items.discord.settings.items.in-game')}</span>
            </td>

            <td>
                <textarea rows="2" on:keyup={(e) => textareaHandler(e, 'in-game')}>{states['in-game']}</textarea>
            </td>
        </tr>
    </table>
</div>
