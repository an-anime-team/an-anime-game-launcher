import type {
    ServerResponse,
    Data,
    Latest
} from './types/GameData';

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

    /**
     * Get latest game data, including voice data and packages difference
     * 
     * @returns Error object if company's servers are unreachable or they responded with an error
     */
    public static getLatestData(): Promise<Data>
    {
        return new Promise(async (resolve, reject) => {
            const response = await fetch(constants.versionsUri);

            if (response.ok)
            {
                const json: ServerResponse = await response.json();

                if (json.message == 'OK')
                    resolve(json.data);

                else reject(new Error(`${constants.placeholders.uppercase.company}'s versions server responds with an error: [${json.retcode}] ${json.message}`));
            }

            else reject(new Error(`${constants.placeholders.uppercase.company}'s versions server is unreachable`));
        });
    }

    /**
     * Get latest game version data
     * 
     * @returns Error object if company's servers are unreachable or they responded with an error
     */
    public static get latest(): Promise<Latest>
    {
        return new Promise(async (resolve, reject) => {
            this.getLatestData()
                .then((data) => resolve(data.game.latest))
                .catch((error) => reject(error));
        });
    }
}
