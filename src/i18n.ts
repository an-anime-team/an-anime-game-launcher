import { register, init } from 'svelte-i18n';

import Locales from './ts/core/Locales';

register('en-us', () => Locales.get('en-us'));
register('ru-ru', () => Locales.get('ru-ru'));
register('de-de', () => Locales.get('de-de'));

Locales.default().then((locale) => {
    init({
        fallbackLocale: 'en-us',
        initialLocale: locale,
    });
});
