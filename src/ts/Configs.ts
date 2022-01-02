import YAML from 'yaml';

import constants from './Constants';

declare const Neutralino;

// Ok yea, null, object and boolean aren't scalars
// but I don't care
type scalar = null | string | number | boolean | object;

export default class Configs
{
    /**
     * Get config value
     * 
     * @param name config name, e.g. "lang.launcher"
     * 
     * @returns undefined if config doesn't exist. Otherwise - config value
     */
    public static get(name: string = ''): Promise<undefined|scalar|scalar[]>
    {
        return new Promise(async (resolve) => {
            Neutralino.filesystem.readFile(await constants.paths.config).then((config) => {
                config = YAML.parse(config);

                if (name !== '')
                {
                    name.split('.').forEach((value) => {
                        config = config[value];
                    });
                }

                resolve(config);
            }).catch(() => {
                setTimeout(() => resolve(this.get(name)), 100);
            });
        });
    }

    /**
     * Set config value
     * 
     * @param name config name, e.g. "lang.launcher"
     * @param value config value, e.g. "en-us"
     * 
     * @returns Promise<void> indicates if the settings were updated
     */
    public static set(name: string, value: scalar|scalar[]|Promise<scalar|scalar[]>): Promise<void>
    {
        const getUpdatedArray = (path: string[], array: scalar|scalar[], value: scalar|scalar[]): scalar|scalar[] => {
            array![path[0]] = path.length > 1 ?
                getUpdatedArray(path.slice(1), array![path[0]] ?? {}, value) : value;

            return array;
        };

        return new Promise(async (resolve) => {
            value = await Promise.resolve(value);

            Neutralino.filesystem.readFile(await constants.paths.config).then(async (config) => {
                config = YAML.stringify(getUpdatedArray(name.split('.'), YAML.parse(config), value));

                Neutralino.filesystem.writeFile(await constants.paths.config, config)
                    .then(() => resolve());
            }).catch(async () => {
                let config = YAML.stringify(getUpdatedArray(name.split('.'), {}, value));

                Neutralino.filesystem.writeFile(await constants.paths.config, config)
                    .then(() => resolve());
            });
        });
    }

    /**
     * Set default values
     * 
     * @param configs object of default values
     * 
     * @returns Promise<void> indicates if the default settings were applied
     */
    public static defaults(configs: object): Promise<void>
    {
        return new Promise(async (resolve) => {
            const setDefaults = async (current: object) => {
                const updateDefaults = (current: object, defaults: object) => {
                    Object.keys(defaults).forEach((key) => {
                        // If the field exists in defaults and doesn't exist in current
                        if (current[key] === undefined)
                            current[key] = defaults[key];

                        // If both default and current are objects
                        // and we also should check if they're not nulls
                        // because JS thinks that [typeof null === 'object']
                        else if (typeof current[key] == 'object' && typeof defaults[key] == 'object' && current[key] !== null && defaults[key] !== null)
                            current[key] = updateDefaults(current[key], defaults![key]);
                    });

                    return current;
                };

                Neutralino.filesystem.writeFile(await constants.paths.config, YAML.stringify(updateDefaults(current, configs)))
                    .then(() => resolve());
            };

            Neutralino.filesystem.readFile(await constants.paths.config)
                .then((config) => setDefaults(YAML.parse(config)))
                .catch(() => setDefaults({}));
        });
    }
}
