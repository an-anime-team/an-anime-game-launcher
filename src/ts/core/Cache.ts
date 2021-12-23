import constants from '../Constants';

type Record = {
    expired: boolean;
    value: object|object[];
};

declare const Neutralino;

export default class Cache
{
    /**
     * Get cached value
     * 
     * @returns null if this value is not cached
     */
    public static get(name: string): Promise<Record|null>
    {
        return new Promise(async (resolve) => {
            Neutralino.filesystem.readFile(await constants.paths.cache)
                .then((cache) => {
                    cache = JSON.parse(cache);

                    if (cache[name] === undefined)
                        resolve(null);

                    else
                    {
                        resolve({
                            expired: cache[name].ttl !== null ? Date.now() > cache[name].ttl * 1000 : false,
                            value: JSON.parse(atob(cache[name].value))
                        });
                    }
                })
                .catch(() => resolve(null));
        });
    }

    /**
     * Cache value
     * 
     * @param name name of the value to cache
     * @param value value to cache
     * @param ttl number of seconds to cache
     * 
     * @returns promise that indicates when the value will be cached
     */
    public static set(name: string, value: object|object[], ttl: number|null = null): Promise<void>
    {
        return new Promise((resolve) => {
            constants.paths.cache.then((cacheFile) => {
                let cache = {};

                Neutralino.filesystem.readFile(cacheFile)
                    .then((cacheRaw) => cache = JSON.parse(cacheRaw))
                    .catch(() => {});

                cache[name] = {
                    ttl: ttl !== null ? Math.round(Date.now() / 1000) + ttl : null,
                    value: btoa(JSON.stringify(value))
                };

                Neutralino.filesystem.writeFile(cacheFile, JSON.stringify(cache))
                    .then(() => resolve());
            });
        });
    }
};

export type { Record };
