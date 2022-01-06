declare const Neutralino;

type Item = {
    /**
     * Item text
     */
    text: string;

    /**
     * Item id
     */
    id?: string;

    /**
     * Whether the item disabled or not
     * 
     * If yes, then it will be a string
     */
    disabled?: boolean;

    /**
     * Is this item a checkbox or not
     */
    checked?: boolean;

    /**
     * Event on click
     * 
     * If specified, then will generate random
     * item id if it is not specified
     */
    click?: (item: Item) => void;
};

Neutralino.events.on('trayMenuItemClicked', (item) => {
    for (const tray of Tray.trays)
        for (const trayItem of tray.items)
            if (trayItem.id === item.detail.id)
            {
                if (trayItem.click)
                {
                    trayItem.click({
                        id: item.detail.id,
                        text: item.detail.text,
                        disabled: item.detail['isDisabled'],
                        checked: item.detail['isChecked'],
                        click: trayItem.click
                    });
                }

                return;
            }
});

export default class Tray
{
    public static trays: Tray[] = [];

    public icon: string;

    protected _items: Item[] = [];

    public get items(): Item[]
    {
        return this._items.map((item) => {
            return {
                id: item.id,
                text: item.text,
                disabled: item['isDisabled'],
                checked: item['isChecked'],
                click: item.click
            };
        });
    }

    public set items(items: Item[])
    {
        this._items = items.map((item) => {
            if (item.id === undefined && item.click !== undefined)
                item.id = 'click:' + Math.random().toString().substring(2);
            
            return {
                id: item.id,
                text: item.text,
                isDisabled: item.disabled,
                isChecked: item.checked,
                click: item.click
            };
        });
    }

    public constructor(icon: string, items: Item[] = [])
    {
        this.icon = icon;
        this.items = items;

        Tray.trays.push(this);
    }

    public update(items: Item[]|null = null): Promise<void>
    {
        if (items !== null)
            this.items = items;
        
        return Neutralino.os.setTray({
            icon: this.icon,
            menuItems: this._items
        });
    }

    public hide(): Promise<void>
    {
        return Neutralino.os.setTray({
            icon: this.icon,
            menuItems: []
        });
    }
};

export type { Item };
