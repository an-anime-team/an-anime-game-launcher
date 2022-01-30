import { register, init } from 'svelte-i18n';

import Locales from './ts/launcher/Locales';

// Supported by the game's API
register('en-us', () => Locales.get('en-us'));
register('ru-ru', () => Locales.get('ru-ru'));
register('es-es', () => Locales.get('es-es'));
register('de-de', () => Locales.get('de-de'));
register('fr-fr', () => Locales.get('fr-fr'));
register('id-id', () => Locales.get('id-id'));

// Unsupported by the game's API
register('it-it', () => Locales.get('it-it'));
register('hu-hu', () => Locales.get('hu-hu'));
register('uwu', () => Locales.get('uwu'));

Locales.default().then((locale) => {
    init({
        fallbackLocale: 'en-us',
        initialLocale: locale,
    });
});
