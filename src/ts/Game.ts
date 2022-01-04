import type {
    ServerResponse,
    Data,
    Latest,
    Diff
} from './types/GameData';

import constants from './Constants';
import fetch from './core/Fetch';
import AbstractInstaller from './core/AbstractInstaller';
import Domain from './core/Domain';
import promisify from './core/promisify';
import Debug, { DebugThread } from './core/Debug';
import Downloader, { Stream as DownloadingStream } from './core/Downloader';
import Cache from './core/Cache';

declare const Neutralino;

class Stream extends AbstractInstaller
{
    public constructor(uri: string, predownloaded: boolean = false)
    {
        super(uri, constants.paths.gameDir, predownloaded);
    }
}

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
            // const persistentPath = `${await constants.paths.gameDataDir}/Persistent/ScriptVersion`;
            const globalGameManagersPath = `${await constants.paths.gameDataDir}/globalgamemanagers`;

            /*Neutralino.filesystem.readFile(persistentPath)
                .then((version) => resolve(version))
                .catch(() => {*/
                    Neutralino.filesystem.readBinaryFile(globalGameManagersPath)
                        .then((config: ArrayBuffer) => {
                            const buffer = new TextDecoder('ascii').decode(new Uint8Array(config));
                            const version = /([1-9]+\.[0-9]+\.[0-9]+)_[\d]+_[\d]+/.exec(buffer);

                            Debug.log({
                                function: 'Game.current',
                                message: `Current game version: ${version !== null ? version[1] : '<unknown>'}`
                            });

                            resolve(version !== null ? version[1] : null);
                        })
                        .catch(() => resolve(null));
                // });
        });
    }

    /**
     * Get latest game data, including voice data and packages difference
     * 
     * @returns JSON from API else throws Error if company's servers are unreachable or if they responded with an error
     */
    public static getLatestData(): Promise<Data>
    {
        return new Promise(async (resolve, reject) => {
            const response = await fetch(constants.versionsUri);

            if (response.ok)
            {
                const cache = await Cache.get('Game.getLatestData.ServerResponse');

                if (cache && !cache.expired)
                    resolve(cache.value as Data);

                else
                {
                    const json: ServerResponse = JSON.parse(await response.body());

                    if (json.message == 'OK')
                    {
                        Cache.set('Game.getLatestData.ServerResponse', json.data, 24 * 3600);

                        resolve(json.data);
                    }

                    else reject(new Error(`${constants.placeholders.uppercase.company}'s versions server responds with an error: [${json.retcode}] ${json.message}`));
                }
            }

            else reject(new Error(`${constants.placeholders.uppercase.company}'s versions server is unreachable`));
        });
    }

    /**
     * Get latest game version data
     * 
     * @returns Latest version else throws Error if company's servers are unreachable or if they responded with an error
     */
    public static get latest(): Promise<Latest>
    {
        return new Promise((resolve, reject) => {
            this.getLatestData()
                .then((data) => resolve(data.game.latest))
                .catch((error) => reject(error));
        });
    }

    /**
     * Get some latest game versions list in descending order
     * e.g. ["2.3.0", "2.2.0", "2.1.0"]
     * 
     * @returns Version else throws Error if company's servers are unreachable or if they responded with an error
     */
    public static get versions(): Promise<string[]>
    {
        return new Promise((resolve, reject) => {
            this.getLatestData()
                .then((data) => {
                    let versions = [data.game.latest.version];

                    data.game.diffs.forEach((diff) => versions.push(diff.version));

                    resolve(versions);
                })
                .catch((error) => reject(error));
        });
    }

    /**
     * Get updated game data from the specified version to the latest
     * 
     * @returns null if the difference can't be calculated
     */
    public static getDiff(version: string): Promise<Diff|null>
    {
        return new Promise(async (resolve, reject) => {
            this.getLatestData()
                .then((data) => {
                    for (const diff of data.game.diffs)
                        if (diff.version == version)
                        {
                            resolve(diff);

                            return;
                        }

                    resolve(null);
                })
                .catch((error) => reject(error));
        });
    }

    /**
     * Get the game installation stream
     * 
     * @param version current game version to download difference from
     * 
     * @returns null if the version can't be found
     * @returns Error if company's servers are unreachable or they responded with an error
     */
    public static update(version: string|null = null): Promise<Stream|null>
    {
        Debug.log({
            function: 'Game.update',
            message: version !== null ?
                `Updating the game from the ${version} version` :
                'Installing the game'
        });

        return new Promise((resolve, reject) => {
            this.isUpdatePredownloaded().then(async (predownloaded) => {
                if (predownloaded)
                {
                    Debug.log({
                        function: 'Game.update',
                        message: 'Update is already pre-downloaded. Unpacking started'
                    });

                    resolve(new Stream(`${await constants.paths.launcherDir}/game-predownloaded.zip`, true));
                }

                else
                {
                    (version === null ? this.latest : this.getDiff(version))
                        .then((data: Latest|Diff|null) => resolve(data === null ? null : new Stream(data.path)))
                        .catch((error) => reject(error));
                }
            });
        });
    }

    /**
     * Pre-download the game update
     * 
     * @param version current game version to download difference from
     * 
     * @returns null if the game pre-downloading is not available. Otherwise - downloading stream
     * @returns Error if company's servers are unreachable or they responded with an error
     */
    public static predownloadUpdate(version: string|null = null): Promise<DownloadingStream|null>
    {
        const debugThread = new DebugThread('Game.predownloadUpdate', 'Predownloading game data...')

        return new Promise((resolve) => {
            this.getLatestData()
                .then((data) => {
                    if (data.pre_download_game)
                    {
                        let path = data.pre_download_game.latest.path;

                        if (version !== null)
                            for (const diff of data.pre_download_game.diffs)
                                if (diff.version == version)
                                {
                                    path = diff.path;

                                    break;
                                }

                        debugThread.log(`Downloading update from the path: ${path}`);

                        constants.paths.launcherDir.then((dir) => {
                            Downloader.download(path, `${dir}/game-predownloaded.zip`)
                                .then((stream) => resolve(stream));
                        });
                    }

                    else resolve(null);
                })
                .catch((error) => resolve(error));
        });
    }

    /**
     * Checks whether the update was downloaded or not
     */
    public static isUpdatePredownloaded(): Promise<boolean>
    {
        return new Promise(async (resolve) => {
            Neutralino.filesystem.getStats(`${await constants.paths.launcherDir}/game-predownloaded.zip`)
                .then(() => resolve(true))
                .catch(() => resolve(false));
        });
    }

    /**
     * Check if the telemetry servers are disabled
     */
    public static isTelemetryDisabled(): Promise<boolean>
    {
        const debugThread = new DebugThread('Game.isTelemetryDisabled', 'Checking if the telemetry servers are disabled');

        return new Promise(async (resolve) => {
            const pipeline = promisify({
                callbacks: await constants.uri.telemetry.map((domain) => {
                    return new Promise((resolve) => {
                        Domain.getInfo(domain).then((info) => resolve(info.available));
                    });
                }),
                callAtOnce: true,
                interval: 500
            });

            pipeline.then((result) => {
                let disabled = false;

                Object.values(result).forEach((value) => disabled ||= value as boolean);

                debugThread.log(`Telemetry is ${disabled ? 'not ' : ''}disabled`);

                resolve(disabled === false);
            });
        });
    }
}

export { Stream };
