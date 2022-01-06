import YAML from 'yaml';

import constants from '../Constants';
import promisify from './promisify';
import Configs from '../Configs';

type AvailableLocales =
    | 'en-us'
    | 'ru-ru'
    | 'de-de'
    | 'fr-fr'
    | 'es-es'
    | 'vi-vn';

declare const Neutralino;

export default class Locales
{
    /**
     * Get or update the default locale
     */
    public static default(lang: AvailableLocales|null = null): Promise<string>
    {
        if (lang !== null)
            Configs.set('lang.launcher', lang);

        return Configs.get('lang.launcher') as Promise<string>;
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
};
