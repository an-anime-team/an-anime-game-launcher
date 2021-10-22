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

module.exports = i18n;