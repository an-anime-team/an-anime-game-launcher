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

declare const Neutralino;

class Stream extends AbstractInstaller
{
    public constructor(uri: string)
    {
        super(uri, constants.paths.gameDir);
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
            const persistentPath = `${await constants.paths.gameDataDir}/Persistent/ScriptVersion`;
            const globalGameManagersPath = `${await constants.paths.gameDataDir}/globalgamemanagers`;

            Neutralino.filesystem.readFile(persistentPath)
                .then((version) => resolve(version))
                .catch(() => {
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
                });
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
                const json: ServerResponse = JSON.parse(await response.body());

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
     * @returns null if the version can't be found
     * @returns Error if company's servers are unreachable or they responded with an error
     */
    public static update(version: string|null = null): Promise<Stream|null>
    {
        Debug.log(
            version !== null ?
            `Updating the game from the ${version} version` :
            'Installing the game'
        );

        return new Promise((resolve, reject) => {
            (version === null ? this.latest : this.getDiff(version))
                .then((data: Latest|Diff|null) => resolve(data === null ? null : new Stream(data.path)))
                .catch((error) => reject(error));
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
