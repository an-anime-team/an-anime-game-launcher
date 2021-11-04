import $ from 'cash-dom';

$(() => {
    $('.checkbox-mark').on('click', (e) => {
        let item = $(e.target);

        while (!item.hasClass('checkbox'))
            item = item.parent();

        if (!item.hasClass('selectable-checkbox'))
            item.toggleClass('checkbox-active').trigger('classChange');
    });

    $('.selectable-checkbox').on('click', (e) => {
        let item = $(e.target);

        while (!item.hasClass('checkbox'))
            item = item.parent();

        item.toggleClass('checkbox-active').trigger('classChange');
    });

    $('.selected-item').on('click', (e) => {
        let item = $(e.target);

        while (!item.hasClass('select'))
            item = item.parent();

        item.toggleClass('select-active').trigger('classChange');
    });

    $('.select-options li').on('click', (e) => {
        let item = $(e.target), li = $(e.target);

        if (!item.hasClass('selected'))
        {
            while (!item.hasClass('select'))
                item = item.parent();

            item.find('.select-options li').removeClass('selected');
            li.addClass('selected');

            item.removeClass('select-active');

            item.find('.selected-item span').text(li.text());

            item.trigger('selectionChanged', {
                caption: li.text(),
                value: li.attr('value')
            });
        }
    });

    /**
     * properties-list
     */

    let propsRowSelect = (e: any) => {
        let item = $(e.target);

        while (!item.is('tr'))
            item = item.parent();

        if ($(item.children()[0]).is('td'))
        {
            $('.properties-list tr').removeClass('selected');
            item.addClass('selected');
        }
    };

    $('.properties-list tr').on('click', propsRowSelect);

    let propsInputsOnChange = (e: any) => {
        let item = $(e.target),
            td = item.parent(),
            td2 = td.siblings().first();

        // property name
        if (td.index() == 0)
        {
            $('.properties-list').trigger('propertyNameChanged', {
                name: {
                    before: td.find('span').text(),
                    after: item.val()
                },
                value: td2.find('input').val()
            });
        }

        // property value
        else
        {
            $('.properties-list').trigger('propertyValueChanged', {
                name: td2.find('input').val(),
                value: {
                    before: td.find('span').text(),
                    after: item.val()
                }
            });
        }

        td.find('span').text(item.val() as string);
    };

    $('.properties-list input').on('change', propsInputsOnChange);

    $('.properties-list .button#add').on('click', (e) => {
        let newRow = $(`<tr>
            <td>
                <span></span>
                <input>
            </td>
            <td>
                <span></span>
                <input>
            </td>
        </tr>`);

        newRow.on('click', propsRowSelect);
        newRow.find('input').on('change', propsInputsOnChange);
        
        newRow.appendTo($(e.target).parent().parent().find('table'));
    });

    $('.properties-list .button#delete').on('click', (e) => {
        $('.properties-list tr.selected').each((i, el) => {
            $('.properties-list').trigger('propertyDeleted', {
                name: $(el).find('input').first().val()
            });

            $(el).remove();
        });
    });
});
