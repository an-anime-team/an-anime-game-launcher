const path = require('path');
const fs = require('fs');

export class i18n
{
     public static readonly localesDir = path.join(path.dirname(__dirname), '..', 'locales');

     public static loadedLanguage: any;

     public static translate (phrase: string)
     {
          if (i18n.loadedLanguage === undefined)
               this.setLang(navigator.language);

          return i18n.loadedLanguage[phrase] ?? phrase;
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
