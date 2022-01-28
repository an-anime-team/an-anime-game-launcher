import { dictionary, locale } from 'svelte-i18n';

import YAML from 'yaml';

import { promisify, Configs } from '../../empathize';

import constants from '../Constants';

type AvailableLocales =
    | 'en-us'
    | 'ru-ru'
    | 'es-es'
    | 'de-de'
    | 'fr-fr'
    | 'it-it'
    | 'id-id'
    | 'uwu';

declare const Neutralino;

export default class Locales
{
    /**
     * List of locales supported by the game's API
     */
    public static readonly supportedLocales: AvailableLocales[] = [
        'en-us', 'ru-ru', 'es-es', 'de-de', 'fr-fr', 'id-id'
    ];

    /**
     * Get or update the default locale
     */
    public static default(lang: AvailableLocales|null = null): Promise<AvailableLocales>
    {
        if (lang !== null)
        {
            Configs.set('lang.launcher', lang);

            return Promise.resolve(lang);
        }
        
        else return Configs.get('lang.launcher') as Promise<AvailableLocales>;
    }

    /**
     * Get locales
     * 
     * @param locale - locale name to get. If null - then will be returned array of all available locales 
     */
    public static get(locale: AvailableLocales|null = null): Promise<object>
    {
        return new Promise((resolve) => {
            if (locale === null)
            {
                Neutralino.filesystem.readDirectory(constants.paths.localesDir)
                    .then(async (folders: { entry: string, type: string }[]) => {
                        folders = folders.filter((folder) => folder.type === 'FILE');

                        const pipeline = promisify({
                            callbacks: folders.map((folder) => {
                                return new Promise((resolve) => {
                                    Neutralino.filesystem.readFile(`${constants.paths.localesDir}/${folder.entry}`)
                                        .then((locale) => resolve(YAML.parse(locale)));
                                });
                            }),
                            callAtOnce: true
                        });

                        pipeline.then((locales) => {
                            let result = {};

                            for (let i = 0; i < folders.length; i++)
                            {
                                const lang = folders[i].entry.substring(0, folders[i].entry.length - 5);

                                result[lang] = locales[i];
                            }

                            resolve(result);
                        });
                    });
            }

            else Neutralino.filesystem.readFile(`${constants.paths.localesDir}/${locale}.yaml`)
                .then((locale) => resolve(YAML.parse(locale)));
        });
    }

    /**
     * Bind some callback to be called every time
     * the locale will be changed
     */
    public static bind(localizer: (message: string|object) => void, localeName: string)
    {
        let currentLocale, currentDictionary;

        const updateLocalizer = () => {
            let message = currentDictionary[currentLocale] ?? currentDictionary['en-us'];

            for (const path of localeName.split('.'))
                message = message[path];

            localizer(message);
        };

        locale.subscribe((locale) => {
            currentLocale = locale;

            if (currentDictionary)
                updateLocalizer();
        });

        dictionary.subscribe((dictionary) => {
            currentDictionary = dictionary;

            if (currentLocale)
                updateLocalizer();
        });
    }

    /**
     * Checks if the specified language supported
     * by the game's API
     */
    public static supported(lang: AvailableLocales): boolean
    {
        return this.supportedLocales.includes(lang);
    }

    /**
     * Returns provided language if it is supported
     * by the game's API. Otherwise returns fallback language (en-us by default)
     */
    public static fallback(lang: AvailableLocales, fallback: AvailableLocales = 'en-us'): AvailableLocales
    {
        return this.supported(lang) ? lang : fallback;
    }
};

export type { AvailableLocales };
