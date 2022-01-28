<script lang="ts">
    import { _ } from 'svelte-i18n';

    import { Configs } from '../empathize';

    export let visible: boolean = false;

    export let valueChanged: (field: 'in-game' | 'in-launcher', value: string) => void = () => {};
    export let iconChanged: (field: 'in-game' | 'in-launcher', icon: string) => void = () => {};

    import Checkbox from './Checkbox.svelte';

    // Discord RPC icons imports
    // We must import them manually because otherwise
    // neutralino won't be able to load them because of its restrictions
    const icons = {
        'bGF1bmNoZXI=': import('../../public/icons/256x256.png'),
        'Z2ktaWNvbg==': import('../assets/images/discord/anime-icon.jpg'),
        'Z2FtZQ==': import('../assets/images/discord/game.jpg'),

        'YXJ0Z2FtZQ==': import('../assets/images/discord/chr-ms-ab-1.jpg'),
        'YXJ0Z2FtZTM=': import('../assets/images/discord/chr-ms-ab-2.jpg'),

        'YmVpZG91Z2FtZQ==': import('../assets/images/discord/chr-ly-bd-1.jpg'),

        'a2xlZWdhbWU=': import('../assets/images/discord/chr-ms-kl-1.jpg'),
        'a2xlZWdhbWUy': import('../assets/images/discord/chr-ms-kl-2.jpg'),
        'YXJ0Z2FtZTI=': import('../assets/images/discord/chr-ms-kl-3.jpg'),

        'YmFhbDE=': import('../assets/images/discord/chr-in-rs-1.webp'),

        'eWFlbWlrbzE=': import('../assets/images/discord/chr-in-ym-1.webp'),
        'eWFlbWlrbzI=': import('../assets/images/discord/chr-in-ym-2.jpg'),

        'bGl5dWVnYW1l': import('../assets/images/discord/loc-ly-1.jpg'),

        'aW5henVtYTE=': import('../assets/images/discord/loc-in-1.jpg'),
        'aW5henVtYTI=': import('../assets/images/discord/loc-in-2.jpg'),
        'aW5henVtYTM=': import('../assets/images/discord/loc-in-3.jpg'),
        'aW5henVtYTQ=': import('../assets/images/discord/loc-in-4.jpg'),
        'aW5henVtYTU=': import('../assets/images/discord/loc-in-5.jpg')
    };

    let iconSelector: 'in-game' | 'in-launcher' | null = null;

    let states = {
        'in-game': {
            text: '',
            icon: 'Z2FtZQ=='
        },
        'in-launcher': {
            text: '',
            icon: 'bGF1bmNoZXI='
        }
    };

    Configs.get('discord.states').then((settings) => {
        states = {
            'in-game': {
                text: settings!['in-game']['details'],
                icon: btoa(settings!['in-game']['icon'])
            },
            'in-launcher': {
                text: settings!['in-launcher']['details'],
                icon: btoa(settings!['in-launcher']['icon'])
            }
        };

        if (settings!['in-game']['state'] != '')
            states['in-game']['text'] += `\n${settings!['in-game']['state']}`;

        if (settings!['in-launcher']['state'] != '')
            states['in-launcher']['text'] += `\n${settings!['in-launcher']['state']}`;
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

    const selectIcon = (icon: string) => {
        states[iconSelector!]['icon'] = icon;

        iconChanged(iconSelector!, atob(icon));

        iconSelector = null;
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
                <textarea rows="2" on:keyup={(e) => textareaHandler(e, 'in-launcher')}>{states['in-launcher']['text']}</textarea>

                {#await icons[states['in-launcher']['icon']] then iconUri}
                    <img src={iconUri.default} alt="" on:click={() => iconSelector = iconSelector ? null : 'in-launcher'} />
                {/await}
            </td>
        </tr>
        <tr>
            <td>
                <span>{$_('settings.general.items.discord.settings.items.in-game')}</span>
            </td>

            <td>
                <textarea rows="2" on:keyup={(e) => textareaHandler(e, 'in-game')}>{states['in-game']['text']}</textarea>

                {#await icons[states['in-game']['icon']] then iconUri}
                    <img src={iconUri.default} alt="" on:click={() => iconSelector = iconSelector ? null : 'in-game'} />
                {/await}
            </td>
        </tr>
    </table>

    <div class="icon-selection" style="display: {iconSelector ? 'block' : 'none'}">
        <h4>{$_('settings.general.items.discord.settings.items.selectIcon')}</h4>

        <div>
            {#each Object.keys(icons) as icon}
                {#await icons[icon] then iconUri}
                    <img src={iconUri.default} alt="" on:click={() => selectIcon(icon)} />
                {/await}
            {/each}
        </div>
    </div>
</div>

<style lang="sass">
    .icon-selection
        div
            display: flex
            flex-wrap: wrap

            width: 100%

            img
                width: 48px
                height: 48px

                border-radius: 24px

                margin: 4px

                cursor: pointer

                &:hover
                    box-shadow: 0px 0px 12px 0px rgba(0, 0, 0, 0.45)

    table.table
        td:last-child
            display: inline-flex
            align-items: center

            width: 100%

            img
                width: 40px
                height: 40px

                border-radius: 20px
</style>
