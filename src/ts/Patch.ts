import type { PatchInfo } from './types/Patch';

import md5 from 'js-md5';

import { fetch, promisify, Debug, Cache, path } from '../empathize';
import { DebugThread } from '@empathize/framework/dist/meta/Debug';

import constants from './Constants';
import Game from './Game';
import AbstractInstaller from './core/AbstractInstaller';
import Launcher from './Launcher';

declare const Neutralino;

class Stream extends AbstractInstaller
{
    protected userUnpackFinishCallback?: () => void;
    protected onPatchFinish?: (result: boolean) => void;

    protected patchFinished: boolean = false;
    protected patchResult: boolean = false;

    public constructor(uri: string, version: string|null = null)
    {
        super(uri, constants.paths.launcherDir);

        /**
         * We'll make our own AbstractInstaller unpacking finish event
         * and provide some hack to call another user-provided function
         * if he wants to make something after patch's archive unpacking
         */
        this.onUnpackFinish = async () => {
            if (this.userUnpackFinishCallback)
                this.userUnpackFinishCallback();

            // Find patch version if it wasn't provided
            if (version === null)
                version = (await Patch.latest).version;

            const patchDir = `${await constants.paths.launcherDir}/dawn/${version.replaceAll('.', '')}`;
            const gameDir = await constants.paths.gameDir;
            const isFlatpak = await Launcher.isFlatpak();

            /**
             * Patch out the testing phase content from the shell files
             * if active and make sure the shell files are executable
             */
            const pipeline = promisify({
                callbacks: [
                    /**
                     * Remove test version restrictions from the main patch
                     */
                    () => Neutralino.os.execCommand(`cd "${path.addSlashes(patchDir)}" && sed -i '/^echo "If you would like to test this patch, modify this script and remove the line below this one."/,+5d' patch.sh`),

                    /**
                     * Remove /etc/hosts editing if running in Flatpak
                     */
                    () => isFlatpak ? Neutralino.os.execCommand(`cd "${path.addSlashes(patchDir)}" && sed -i '/^# ===========================================================/,+79d' patch.sh`) : null,
                    
                    /**
                     * Remove test version restrictions from the anti-login crash patch
                     */
                    () => Neutralino.os.execCommand(`cd "${path.addSlashes(patchDir)}" && sed -i '/^echo "       necessary afterwards (Friday?). If that's the case, comment the line below."/,+2d' patch_anti_logincrash.sh`),

                    /**
                     * Make the main patch executable
                     */
                    () => Neutralino.os.execCommand(`chmod +x "${path.addSlashes(patchDir)}/patch.sh"`),

                    /**
                     * Make the anti-login crash patch executable
                     */
                    () => Neutralino.os.execCommand(`chmod +x "${path.addSlashes(patchDir)}/patch_anti_logincrash.sh"`),

                    /**
                     * Execute the main patch installation script
                     * Use pkexec if not running in Flatpak
                     */
                     () => Neutralino.os.execCommand(`yes yes | ${isFlatpak ? '' : 'pkexec'} bash -c 'cd "${path.addSlashes(gameDir)}" ; "${path.addSlashes(patchDir)}/patch.sh"'`),

                    /**
                     * Execute the anti-login crash patch installation script
                     */
                    () => Neutralino.os.execCommand(`cd "${path.addSlashes(gameDir)}" && yes | bash "${path.addSlashes(patchDir)}/patch_anti_logincrash.sh"`)
                ]
            });

            // When all the things above are done
            pipeline.then((outputs) => {
                this.patchFinished = true;

                Debug.log({
                    function: 'Patch/Stream',
                    message: [
                        'Patch script output:',
                        ...outputs[5].stdOut.split(/\r\n|\r|\n/)
                    ]
                });

                this.patchResult = outputs[5].stdOut.includes('==> Patch applied! Enjoy the game');

                if (this.onPatchFinish)
                    this.onPatchFinish(this.patchResult);
            });
        };
    }

    public unpackFinish(callback: () => void)
    {
        this.userUnpackFinishCallback = callback;

        if (this.unpackFinished)
            callback();
    }

    /**
     * Specify event that will be called when the patch will be applied
     */
    public patchFinish(callback: (result: boolean) => void)
    {
        this.onPatchFinish = callback;

        if (this.patchFinished)
            callback(this.patchResult);
    }
}

export default class Patch
{
    public static fetchTimeout: number|null = 5000;

    /**
     * Get information about latest available patch
     * 
     * @returns Patch information otherwise throws Error object if the patch's repositories are unreachable or they responded with an error
     */
    public static get latest(): Promise<PatchInfo>
    {
        const debugThread = new DebugThread('Patch.latest', 'Getting the latest patch information');

        return new Promise(async (resolve, reject) => {
            const getLatestPatchInfo = (versions: string[], source: 'origin' | 'additional'): Promise<PatchInfo> => {
                return new Promise(async (resolve) => {
                    const version = versions[0];

                    this.getPatchInfo(version, source)
                        .then(async (patchInfo) => {
                            // Patch with version e.g. [2.4.0] doesn't exist
                            // so we're looking for [2.3.0] instead
                            if (patchInfo === null)
                                resolve(await getLatestPatchInfo(versions.slice(1), 'origin'));

                            // Otherwise - return found info
                            else
                            {
                                debugThread.log({ message: patchInfo });

                                resolve(patchInfo);
                            }
                        })
                        .catch(async (error) => {
                            // If we couldn't connect to the origin repo
                            // then we can try to connect to the additional one
                            if (source === 'origin')
                                resolve(await getLatestPatchInfo(versions, 'additional'));

                            // Otherwise both of origin and additional repos
                            // are unreachable and we should notice about that
                            else reject(error);
                        });
                });
            };

            resolve(await getLatestPatchInfo(await Game.versions, 'origin'));
        });
    }

    /**
     * Get information about the patch with specified version
     * 
     * Be aware that `applied = true` field may mean that the game version
     * is more recent than the one that this patch was made for because
     * this field actually compares some files hashes
     * 
     * @returns null if patch with given version doesn't exist in given source
     * @returns rejects Error object if the source is unreachable or it responded with an error
     */
    public static getPatchInfo(version: string, source: 'origin' | 'additional' = 'origin'): Promise<PatchInfo|null>
    {
        return new Promise(async (resolve, reject) => {
            const resolveOutput = (output: PatchInfo|null, unityPlayerHash: string|null = null) => {
                Cache.set(`Patch.getPatchInfo.${version}.${source}`, {
                    available: true,
                    output: output,
                    playerHash: unityPlayerHash
                }, 6 * 3600);

                resolve(output);
            };

            const rejectOutput = (error: Error) => {
                // Cache this error only on an hour
                // because then the server can become alive
                Cache.set(`Patch.getPatchInfo.${version}.${source}`, {
                    available: false,
                    error: error
                }, 3600);

                reject(error);
            };

            const cache = await Cache.get(`Patch.getPatchInfo.${version}.${source}`);

            // If we have result cached
            if (cache && !cache.expired)
            {
                if (cache.value['available'])
                {
                    // Verify UnityPlayer.dll file hash
                    // before responding whether the patch applied or not
                    if (cache.value['playerHash'] !== null)
                    {
                        constants.paths.gameDir.then((gameDir) => {
                            Neutralino.filesystem.readBinaryFile(`${gameDir}/UnityPlayer.dll`)
                                .then((currPlayer: ArrayBuffer) => {
                                    cache.value['output']['applied'] = md5(currPlayer) != cache.value['playerHash'];

                                    resolve(cache.value['output']);
                                })
                                .catch(() => resolve(cache.value['output']));
                        });
                    }

                    else resolve(cache.value['output']);
                }

                else reject(cache.value['error']);
            }

            // And otherwise
            else
            {
                const patchUri = constants.uri.patch[source];

                fetch(`${patchUri}/raw/master/${version.replaceAll('.', '')}/README.txt`, this.fetchTimeout)
                    .then((readmeResponse) => {
                        // Return an error if patch's server is unavailable
                        if (readmeResponse.status === null)
                            rejectOutput(new Error(`${source} patch repository is unreachable`));

                        // If [version]/README.txt file doesn't exist - it means
                        // that patch repo has no [version]
                        else if (readmeResponse.status === 404)
                            resolveOutput(null);

                        // Otherwise it should be [preparation], [testing] or [stable]
                        else
                        {
                            fetch(`${patchUri}/raw/master/${version.replaceAll('.', '')}/patch_files/unityplayer_patch_os.vcdiff`, this.fetchTimeout)
                                .then((response) => {
                                    // Return an error if patch's server is unavailable
                                    if (response.status === null)
                                        rejectOutput(new Error(`${source} patch repository is unreachable`));

                                    // If [version]/patch_files/unityplayer_patch.vcdiff file doesn't exist
                                    // then it's [preparation] state and Krock just moved patch.sh file to the [version] folder
                                    else if (response.status === 404)
                                    {
                                        resolveOutput({
                                            version: version,
                                            state: 'preparation',
                                            applied: false,
                                            source: source
                                        });
                                    }

                                    // Otherwise it's [testing] or [stable]
                                    else
                                    {
                                        fetch(`${patchUri}/raw/master/${version.replaceAll('.', '')}/patch.sh`, this.fetchTimeout)
                                            .then((patcherResponse) => {
                                                // Return an error if patch's server is unavailable
                                                if (patcherResponse.status === null)
                                                    rejectOutput(new Error(`${source} patch repository is unreachable`));
                                                
                                                else patcherResponse.body(this.fetchTimeout)
                                                    .then((response) => {
                                                        // Return an error if patch's server is unavailable
                                                        if (response === '')
                                                            rejectOutput(new Error(`${source} patch repository is unreachable`));

                                                        // Otherwise - let's prepare [testing] or [stable] output
                                                        else
                                                        {
                                                            // If this line is commented - then it's [stable] version
                                                            // Otherwise it's [testing]
                                                            const stableMark = '#echo "If you would like to test this patch, modify this script and remove the line below this one."';

                                                            let patchInfo: PatchInfo = {
                                                                version: version,
                                                                state: response.includes(stableMark) ? 'stable' : 'testing',
                                                                applied: false,
                                                                source: source
                                                            };

                                                            const originalPlayer = /if \[ "\${sum}" == "([a-z0-9]{32})" \]; then/mg.exec(response);

                                                            // If we could get original UnityPlayer.dll hash - then we can
                                                            // compare it with actual UnityPlayer.dll hash and say whether the patch
                                                            // was applied or not
                                                            if (originalPlayer !== null)
                                                            {
                                                                constants.paths.gameDir.then((gameDir) => {
                                                                    Neutralino.filesystem.readBinaryFile(`${gameDir}/UnityPlayer.dll`)
                                                                        .then((currPlayer: ArrayBuffer) => {
                                                                            patchInfo.applied = md5(currPlayer) != originalPlayer[1];

                                                                            resolveOutput(patchInfo, originalPlayer[1]);
                                                                        })
                                                                        .catch(() => resolveOutput(patchInfo));
                                                                });
                                                            }

                                                            else resolveOutput(patchInfo);
                                                        }
                                                    });
                                            });
                                    }
                                });
                        }
                    });
            }
        });
    }

    /**
     * Get patch installation stream
     * 
     * @returns null if the latest available patch in preparation state
     * @returns rejects Error object if the patch's repositories are unreachable or they responded with an error
     */
    public static install(patch: PatchInfo|null = null): Promise<Stream|null>
    {
        Debug.log({
            function: 'Patch.install',
            message: 'Installing the patch...'
        });

        return new Promise((resolve, reject) => {
            Promise.resolve((patch ?? this.latest))
                .then((patch) => {
                    if (patch.state === 'preparation')
                        resolve(null);
                    
                    else resolve(new Stream(constants.getPatchUri(patch.source ?? 'origin'), patch.version));
                })
                .catch((err) => reject(err));
        });
    }
}

export { Stream };
