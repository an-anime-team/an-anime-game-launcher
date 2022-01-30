<script lang="ts">
    import { _ } from 'svelte-i18n';

    import { Configs } from '../empathize';

    export let visible: boolean = false;

    export let valueChanged: (field: 'height' | 'width', value: string) => void = () => {};

    let winevdhandw = {
        height: '0',
        width: '0'
    }

    Configs.get('winevd.height').then(height => winevdhandw.height = height!.toString());
    Configs.get('winevd.width').then(width => winevdhandw.width = width!.toString());

    const textareaHandler = (event: KeyboardEvent, field: 'height' | 'width') => {
        const textArea = event.srcElement as HTMLTextAreaElement;
        const content = textArea.value
                .replace(/\b0+/g, '')
                .replace(/[^0-9.]/g, '')
                .replace(/(\..*?)\..*/g, '$1')
                .replace(/([0-9]{0,6}(\.[0-9]{0,2})?).*/g, '$1');

        valueChanged(field, content);
    };
</script>
<div style="display: {visible ? 'block' : 'none'}">
    <h3>{$_('settings.enhancements.items.winevd.settings.title')}</h3>
    
    <table class="table" style="margin-top: 16px">
        <tr>
            <td>
                <span>{$_('settings.enhancements.items.winevd.settings.items.height')}</span>
            </td>

            <td>
                <textarea rows="2" on:keyup={(e) => textareaHandler(e, 'height')}>{winevdhandw.height}</textarea>
            </td>
        </tr>
        <tr>
            <td>
                <span>{$_('settings.enhancements.items.winevd.settings.items.width')}</span>
            </td>

            <td>
                <textarea rows="2" on:keyup={(e) => textareaHandler(e, 'width')}>{winevdhandw.width}</textarea>
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