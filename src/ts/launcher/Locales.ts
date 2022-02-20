import { get as svelteget } from 'svelte/store';
import { dictionary, locale } from 'svelte-i18n';

import YAML from 'yaml';

import { promisify, Configs } from '../../empathize';

import constants from '../Constants';

type AvailableLocales =
    // Supported by the game's API
    | 'en-us' | 'ru-ru' | 'es-es'
    | 'de-de' | 'fr-fr' | 'id-id'
    | 'zh-cn'

    // Unsupported by the game's API
    | 'it-it' | 'hu-hu' | 'uwu'
    | 'nb-no';

declare const Neutralino;

export default class Locales
{
    /**
     * List of locales supported by the game's API
     */
    public static readonly supportedLocales: AvailableLocales[] = [
        'en-us', 'ru-ru', 'es-es', 'de-de', 'fr-fr', 'id-id', 'zh-cn'
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
     * Get system-used ($LC_ALL / $LC_MESSAGES / $LANG) locale name, or {fallback}
     * if system-used locale is not supported
     * 
     * From "man locale.7":
     * 
     * 1. If there is a non-null environment variable LC_ALL, the value of LC_ALL is used.
     * 2. If an environment variable with the same name as one of the categories above exists and is non-null, its value is used for that category.
     * 3. If there is a non-null environment variable LANG, the value of LANG is used.
     */
    public static system(fallback: AvailableLocales = 'en-us'): Promise<AvailableLocales>
    {
        return new Promise(async (resolve) => {
            let locale = await Neutralino.os.getEnv('LC_ALL');

            // If $LC_ALL is empty - trying to get $LC_MESSAGES
            if (!locale)
                locale = await Neutralino.os.getEnv('LC_MESSAGES');

            // If $LC_MESSAGES is empty - getting $LANG
            // If it's empty - than {fallback} will be used by the this.fallback() method
            if (!locale)
                locale = await Neutralino.os.getEnv('LANG');

            // "en_US.UTF-8" -> "en-us"
            resolve(this.fallback(locale.substring(0, locale.indexOf('.')).toLowerCase().replace('_', '-') as AvailableLocales, fallback));
        });
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
            {
                message = message[path] ?? null;

                if (message === null)
                    break;
            }

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
     * Get translation from the currently selected locale
     */
    public static translate<T>(message: string): T
    {
        const currentDictionary = svelteget(dictionary) as object;

        let translation = currentDictionary[svelteget(locale) ?? 'en-us'] ?? currentDictionary['en-us'];

        for (const path of message.split('.'))
        {
            translation = translation[path] ?? null;

            if (translation === null)
                break;
        }

        return translation as T;
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
