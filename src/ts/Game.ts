import constants from './Constants';

declare const Neutralino;

export default class Game
{
    /**
     * Get current installed game version
     * 
     * @returns null if the game version can't be parsed
     */
    public static get current(): Promise<string|null>
    {
        return new Promise(async (resolve) => {
            const persistentPath = `${await constants.paths.gameDataDir}/Persistent/ScriptVersion`;
            const globalGameManagersPath = `${await constants.paths.gameDataDir}/globalgamemanagers`;

            Neutralino.filesystem.readFile(persistentPath)
                .then((version) => resolve(version))
                .catch(() => {
                    Neutralino.filesystem.readFile(globalGameManagersPath)
                        .then((config) => {
                            const version = /([1-9]+\.[0-9]+\.[0-9]+)_[\d]+_[\d]+/.exec(config);

                            resolve(version !== null ? version[1] : null);
                        })
                        .catch(() => resolve(null));
                });
        });
    }
}
