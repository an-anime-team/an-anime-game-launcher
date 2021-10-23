const path = require("path");
const fs = require('fs');
let loadedLanguage: any;

function i18n(): any {
     if(fs.existsSync(path.join(path.dirname(__dirname), 'locales', navigator.language.toLowerCase() + '.json'))) {
          loadedLanguage = JSON.parse(fs.readFileSync(path.join(path.dirname(__dirname), 'locales', navigator.language.toLowerCase() + '.json'), 'utf8'));
     }
     else {
          loadedLanguage = JSON.parse(fs.readFileSync(path.join(path.dirname(__dirname), 'locales', 'en.json'), 'utf8'));
     }
};

i18n.prototype.translate = function(phrase: any) {
    let translation = loadedLanguage[phrase];

    if(translation === undefined) {
         translation = phrase;
    }

    return translation
}

i18n.prototype.updatelang = function(newlang: string) {
     // Test if the locale is the same string so if it's de-de or id-id remove -de or -id like navigator.language does.
     let samecode = new RegExp(`(${newlang.toLowerCase().replace(/-.*$/, '')}.*){2}`, 'g');
     samecode.test(newlang.toLowerCase()) ? newlang = newlang.toLowerCase().replace(/-.*$/, '') : newlang = newlang.toLowerCase();
     if (newlang == 'ja-jp') newlang = 'ja';
     if (newlang == 'vi-vn') newlang = 'vi';

     if(fs.existsSync(path.join(path.dirname(__dirname), 'locales', newlang + '.json'))) {
          loadedLanguage = JSON.parse(fs.readFileSync(path.join(path.dirname(__dirname), 'locales', newlang + '.json'), 'utf8'));
     }
     else {
          loadedLanguage = JSON.parse(fs.readFileSync(path.join(path.dirname(__dirname), 'locales', 'en.json'), 'utf8'));
     }
}

export default new (i18n as any);