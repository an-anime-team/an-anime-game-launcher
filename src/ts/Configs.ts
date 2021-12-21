import constants from './Constants';

declare const Neutralino;
declare const NL_CWD;

// Ok yea, null and object aren't scalars
// but I don't care
type scalar = null | string | number | object;

export default class Configs
{
    /**
     * Get config value
     * 
     * @param name config name, e.g. "lang.launcher"
     * @returns undefined if config doesn't exist. Otherwise - config value
     */
    public static get(name: string = ''): Promise<undefined|scalar|scalar[]>
    {
        return new Promise(async (resolve) => {
            Neutralino.filesystem.readFile(await constants.paths.config).then((config) => {
                config = JSON.parse(config);

                if (name !== '')
                {
                    name.split('.').forEach((value) => {
                        config = config[value];
                    });
                }

                resolve(config);
            }).catch(() => resolve(undefined));
        });
    }

    /**
     * Set config value
     * 
     * @param name config name, e.g. "lang.launcher"
     * @param value config value, e.g. "en-us"
     * @returns Promise<void> indicates if the settings were updated
     */
    public static set(name: string, value: scalar|scalar[]): Promise<void>
    {
        const getUpdatedArray = (path: string[], array: scalar|scalar[], value: scalar|scalar[]): scalar|scalar[] => {
            array[path[0]] = path.length > 1 ?
                getUpdatedArray(path.slice(1), array[path[0]] ?? {}, value) : value;

            return array;
        };

        return new Promise(async (resolve) => {
            Neutralino.filesystem.readFile(await constants.paths.config).then(async (config) => {
                config = JSON.stringify(getUpdatedArray(name.split('.'), JSON.parse(config), value), null, 4);

                Neutralino.filesystem.writeFile(await constants.paths.config, config)
                    .then(() => resolve());
            }).catch(async () => {
                let config = JSON.stringify(getUpdatedArray(name.split('.'), {}, value), null, 4);

                Neutralino.filesystem.writeFile(await constants.paths.config, config)
                    .then(() => resolve());
            });
        });
    }

    /**
     * Set default values
     * 
     * @param configs object of default values
     * @returns Promise<void> indicates if the default settings were applied
     */
    public static defaults(configs: scalar): Promise<void>
    {
        return new Promise(async (resolve) => {
            const setDefaults = async (current: scalar) => {
                const updateDefaults = (current: scalar, defaults: scalar) => {
                    Object.keys(defaults).forEach((key) => {
                        // If the field exists in defaults and doesn't exist in current
                        if (current[key] === undefined)
                            current[key] = defaults[key];

                        // If default is scalar and current object
                        else if (typeof current[key] == 'object' && typeof defaults[key] != 'object')
                            current[key] = defaults[key];

                        // If default is object and current scalar
                        else if (typeof current[key] != 'object' && typeof defaults[key] == 'object')
                            current[key] = defaults[key];

                        // If both of default and current are objects
                        else if (typeof current[key] == 'object' && typeof defaults[key] == 'object')
                            current[key] = updateDefaults(current[key], defaults[key]);
                    });

                    return current;
                };

                current = JSON.stringify(updateDefaults(current, configs), null, 4);

                Neutralino.filesystem.writeFile(await constants.paths.config, current)
                    .then(() => resolve());
            };

            Neutralino.filesystem.readFile(await constants.paths.config)
                .then((config) => setDefaults(JSON.parse(config)))
                .catch(() => setDefaults({}));
        });
    }
}
