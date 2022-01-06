<script lang="ts">
    import { _ } from 'svelte-i18n';

    import Configs from '../ts/Configs';

    export let visible: boolean = false;

    export let valueChanged: (field: 'in-game' | 'in-launcher', value: string) => void = () => {};
    export let iconChanged: (field: 'in-game' | 'in-launcher', icon: string) => void = () => {};

    import Checkbox from './Checkbox.svelte';

    // Discord RPC icons imports
    // It's better to do it manually so Vite
    // will be able to pack them automatically
    import LauncherIcon from '../../public/icons/256x256.png';
    import GameOriginalIcon from '../assets/images/discord/gi-icon.jpg';
    import GameIcon from '../assets/images/discord/game.jpg';
    import ArtGameIcon from '../assets/images/discord/artgame.jpg';
    import ArtGame2Icon from '../assets/images/discord/artgame2.jpg';
    import ArtGame3Icon from '../assets/images/discord/artgame3.jpg';
    import BeidouGameIcon from '../assets/images/discord/beidougame.jpg';
    import KleeGameIcon from '../assets/images/discord/kleegame.jpg';
    import KleeGame2Icon from '../assets/images/discord/kleegame2.jpg';
    import LiyueGameIcon from '../assets/images/discord/liyuegame.jpg';

    const icons = {
        'launcher': LauncherIcon,
        'gi-icon': GameOriginalIcon,
        'game': GameIcon,
        'artgame': ArtGameIcon,
        'artgame2': ArtGame2Icon,
        'artgame3': ArtGame3Icon,
        'beidougame': BeidouGameIcon,
        'kleegame': KleeGameIcon,
        'kleegame2': KleeGame2Icon,
        'liyuegame': LiyueGameIcon
    };

    let iconSelector: 'in-game'|'in-launcher'|null = null;

    let states = {
        'in-game': {
            text: '',
            icon: 'game'
        },
        'in-launcher': {
            text: '',
            icon: 'launcher'
        }
    };

    Configs.get('discord.states').then((settings) => {
        states = {
            'in-game': {
                text: settings!['in-game']['details'],
                icon: settings!['in-game']['icon']
            },
            'in-launcher': {
                text: settings!['in-launcher']['details'],
                icon: settings!['in-launcher']['icon']
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

        iconChanged(iconSelector!, icon);

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

                <img src={icons[states['in-launcher']['icon']]} alt="" on:click={() => iconSelector = iconSelector ? null : 'in-launcher'} />
            </td>
        </tr>
        <tr>
            <td>
                <span>{$_('settings.general.items.discord.settings.items.in-game')}</span>
            </td>

            <td>
                <textarea rows="2" on:keyup={(e) => textareaHandler(e, 'in-game')}>{states['in-game']['text']}</textarea>

                <img src={icons[states['in-game']['icon']]} alt="" on:click={() => iconSelector = iconSelector ? null : 'in-game'} />
            </td>
        </tr>
    </table>

    <div class="icon-selection" style="display: {iconSelector ? 'block' : 'none'}">
        <h4>{$_('settings.general.items.discord.settings.items.selectIcon')}</h4>

        <div>
            {#each Object.keys(icons) as icon}
                <img src={icons[icon]} alt="" on:click={() => selectIcon(icon)} />
            {/each}
        </div>
    </div>
</div>

<style lang="sass">
    .icon-selection
        div
            display: inline-flex

            width: 100%

            img
                width: 48px
                height: 48px

                border-radius: 24px

                margin: 0 auto

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
