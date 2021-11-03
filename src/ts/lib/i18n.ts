const path = require('path');
const fs = require('fs');

import { constants } from './constants';

export class i18n
{
    public static readonly localesDir = path.join(path.dirname(__dirname), '..', 'locales');

    public static loadedLanguage: any;

    public static translate (phrase: string): string
    {
        if (i18n.loadedLanguage === undefined)
            this.setLang(navigator.language);

        let translation = i18n.loadedLanguage[phrase] ?? phrase;
        let item;

        while ((item = /\{([a-zA-Z\.]+)\}/g.exec(translation)) !== null)
        {
            let value = constants;

            // @ts-expect-error
            item[1].split('.').forEach(ref => value = value[ref]);

            translation = translation.replace(item[0], value);
        }

        return translation;
    }

    public static setLang (lang: string)
    {
        lang = lang.toLowerCase();

        // Test if the locale is the same string so if it's de-de or id-id remove -de or -id like navigator.language does.
        let samecode = new RegExp(`(${lang.replace(/-.*$/, '')}.*){2}`, 'g');

        if (samecode.test(lang))
            lang = lang.replace(/-.*$/, '');

        switch (lang)
        {
            case 'ja-jp':
                lang = 'ja';

            break;

            case 'vi-vn':
                lang = 'vi';

            break;
        }

        i18n.loadedLanguage = JSON.parse(fs.readFileSync(path.join(this.localesDir, 
            fs.existsSync(path.join(this.localesDir, lang + '.json')) ?
                lang + '.json' : 'en.json'
        )));
    }
}
