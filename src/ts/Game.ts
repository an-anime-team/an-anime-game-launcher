import type {
    ServerResponse,
    Data,
    Latest,
    Diff,
    PreDownloadGame,
    Game as GameData,
} from './types/GameData';

import type { Stream as DownloadingStream } from '@empathize/framework/dist/network/Downloader';

import { fetch, Domain, promisify, Downloader, Cache, Debug, Package, Configs } from '../empathize';
import { DebugThread } from '@empathize/framework/dist/meta/Debug';

import constants from './Constants';
import AbstractInstaller from './core/AbstractInstaller';
import { isDownloaded, resolveDownloadTarget } from './core/Download';

declare const Neutralino;

class Stream extends AbstractInstaller {
    public constructor(uri: string, predownloaded: boolean = false) {
        super(uri, constants.paths.gameDir, predownloaded);
    }
}

export default class Game {
    protected static _server: 'global' | 'cn' | null = null;

    public static get server(): Promise<'global' | 'cn'>
    {
        return new Promise(async (resolve) => {
            if (!this._server)
                this._server = (await Configs.get('server')) as 'global' | 'cn';

            resolve(this._server);
        });
    }

    /**
     * Get current installed game version
     * 
     * @returns null if the game version can't be parsed
     */
    public static get current(): Promise<string | null>
    {
        return new Promise(async (resolve) => {
            const globalGameManagersPath = `${await constants.paths.gameDataDir}/globalgamemanagers`;

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
    }

    /**
     * Get latest game data, including voice data and packages difference
     *
     * @returns JSON from API else throws Error if company's servers are unreachable or if they responded with an error
     */
    public static async getLatestData(): Promise<Data> {
        const cache = await Cache.get(`Game.getLatestData.ServerResponse.${await this.server}`);
        if (cache && !cache.expired) {
            return cache.value as Data;
        }
        const response = await fetch(constants.versionsUri(await this.server));
        if (response.ok) {
            const json: ServerResponse = JSON.parse(await response.body());
            if (json.message == 'OK') {
                Cache.set(`Game.getLatestData.ServerResponse.${await this.server}`, json.data, 6 * 3600);
                return json.data;
            } else {
                throw new Error(`${constants.placeholders.uppercase.company}'s versions server responds with an error: [${json.retcode}] ${json.message}`);
            }
        } else {
            throw new Error(`${constants.placeholders.uppercase.company}'s versions server is unreachable`);
        }
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
    public static getDiff(version: string): Promise<Diff | null>
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
    public static update(version: string | null = null): Promise<Stream | null>
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

                else (version === null ? this.latest : this.getDiff(version))
                    .then((data: Latest | Diff | null) => resolve(data === null ? null : new Stream(data.path)))
                    .catch((error) => reject(error));
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
    public static predownloadUpdate(version: string | null = null): Promise<DownloadingStream | null | unknown>
    {
        return new Promise((resolve, reject) => {
            const debugThread = new DebugThread('Game.predownloadUpdate', 'Predownloading game data...');

            this.getLatestData()
                .then(async (data) => {
                    if (data.pre_download_game)
                    {
                        const target = resolveDownloadTarget(data.pre_download_game, version);

                        debugThread.log(`Downloading update from the path: ${target.path}`);

                        const dir = await constants.paths.launcherDir;
                        const stream = await Downloader.download(target.path, `${dir}/game-predownloaded.zip`);

                        resolve(stream);
                    }

                    else resolve(null);
                })
                .catch((error) => reject(error));
        });
    }

    /**
     * Checks whether the update was downloaded or not
     */
    public static async isUpdatePredownloaded(pre_download_game?: PreDownloadGame | GameData): Promise<boolean>
    {
        const debugThread = new DebugThread('Game.isUpdatePredownloaded', 'Checking if the the pre-download package is downloaded');

        if (!pre_download_game)
        {
            const data = await this.getLatestData();

            pre_download_game = data.pre_download_game ?? data.game;
        }

        const version = await Game.current
        const target = resolveDownloadTarget(pre_download_game, version);

        debugThread.log(`Predownload target for version ${version}: ${JSON.stringify(target)}`);

        return await isDownloaded(target, `${await constants.paths.launcherDir}/game-predownloaded.zip`);
    }

    /**
     * Check if the telemetry servers are disabled
     * 
     * @returns throws Error object when iputils package (ping command) is not available
     */
    public static isTelemetryDisabled(): Promise<boolean>
    {
        const debugThread = new DebugThread('Game.isTelemetryDisabled', 'Checking if the telemetry servers are disabled');

        return new Promise(async (resolve, reject) => {
            // If ping command is not available - throw an error
            if (!await Package.exists('ping'))
            {
                debugThread.log('iputils package is not installed');

                reject(new Error('iputils package is not installed'));
            }

            // Otherwise - check if telemetry is disabled
            else
            {
                const pipeline = promisify({
                    callbacks: await constants.uri.telemetry[await this.server].map((domain) => {
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
            }
        });
    }
}

export { Stream };
