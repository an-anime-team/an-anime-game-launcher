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
});
