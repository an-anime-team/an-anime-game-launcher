<script lang="ts">
    import { _ } from 'svelte-i18n';

    import { Configs } from '../empathize';

    export let visible: boolean = false;

    export let valueChanged: (field: 'width' | 'height', value: string) => void = () => {};

    let virtual_desktop = {
        width: 0,
        height: 0
    };

    Configs.get('wine.virtual_desktop').then((desktop) => virtual_desktop = desktop as typeof virtual_desktop);

    const inputHandler = (event: KeyboardEvent, field: 'width' | 'height') => {
        const input = event.srcElement as HTMLInputElement;

        const content = input.value
                .replace(/\b0+/g, '')
                .replace(/[^0-9.]/g, '')
                .replace(/(\..*?)\..*/g, '$1')
                .replace(/([0-9]{0,6}(\.[0-9]{0,2})?).*/g, '$1');

        valueChanged(field, content);
    };
</script>

<div style="display: {visible ? 'block' : 'none'}">
    <h3>{$_('settings.enhancements.wine.items.winevd.settings.title')}</h3>
    
    <table class="table" style="margin: 16px 0">
        <tr>
            <td>
                <span>{$_('settings.enhancements.wine.items.winevd.settings.items.width')}</span>
            </td>

            <td>
                <input value={virtual_desktop.width} on:keyup={(e) => inputHandler(e, 'width')} />
            </td>
        </tr>
        <tr>
            <td>
                <span>{$_('settings.enhancements.wine.items.winevd.settings.items.height')}</span>
            </td>

            <td>
                <input value={virtual_desktop.height} on:keyup={(e) => inputHandler(e, 'height')} />
            </td>
        </tr>
    </table>
</div>

<style lang="sass">
    table.table
        td:last-child
            display: inline-flex
            align-items: center

            width: 100%
</style>