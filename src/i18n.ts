import { register, init } from 'svelte-i18n';

import Locales from './ts/launcher/Locales';

register('en-us', () => Locales.get('en-us'));
register('ru-ru', () => Locales.get('ru-ru'));
register('de-de', () => Locales.get('de-de'));
register('fr-fr', () => Locales.get('fr-fr'));
register('it-it', () => Locales.get('it-it'));
register('uwu', () => Locales.get('uwu'));

Locales.default().then((locale) => {
    init({
        fallbackLocale: 'en-us',
        initialLocale: locale,
    });
});
