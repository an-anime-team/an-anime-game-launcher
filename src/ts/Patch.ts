import type { PatchInfo } from './types/Patch';

import md5 from 'js-md5';

import constants from './Constants';
import Game from './Game';
import fetch from './core/Fetch';
import AbstractInstaller from './core/AbstractInstaller';
import promisify from './core/promisify';
import Process from './neutralino/Process';
import Debug, { DebugThread } from './core/Debug';

declare const Neutralino;

class Stream extends AbstractInstaller
{
    protected userUnpackFinishCallback?: () => void;
    protected onPatchFinish?: () => void;

    protected patchFinished: boolean = false;

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

            /**
             * Patch out the testing phase content from the shell files
             * if active and make sure the shell files are executable
             */
            const pipeline = promisify({
                callbacks: [
                    /**
                     * Remove test version restrictions from the main patch
                     */
                    () => Neutralino.os.execCommand(`cd '${patchDir}' && sed -i '/^echo "If you would like to test this patch, modify this script and remove the line below this one."/,+5d' patch.sh`),

                    /**
                     * Remove /etc/hosts editing due to sudo permissions
                     */
                    () => Neutralino.os.execCommand(`cd '${patchDir}' && sed -i '/^# ===========================================================/,+68d' patch.sh`),
                    
                    /**
                     * Remove test version restrictions from the anti-login crash patch
                     */
                    () => Neutralino.os.execCommand(`cd '${patchDir}' && sed -i '/^echo "       necessary afterwards (Friday?). If that's the case, comment the line below."/,+2d' patch_anti_logincrash.sh`),

                    /**
                     * Make the main patch executable
                     */
                    () => Neutralino.os.execCommand(`chmod +x '${patchDir}/patch.sh'`),

                    /**
                     * Make the anti-login crash patch executable
                     */
                    () => Neutralino.os.execCommand(`chmod +x '${patchDir}/patch_anti_logincrash.sh'`),

                    /**
                     * Execute the main patch installation script
                     */
                    (): Promise<void> => {
                        return new Promise(async (resolve) => {
                            Process.run(`yes yes | bash '${patchDir}/patch.sh'`, {
                                cwd: await constants.paths.gameDir
                            }).then((process) => {
                                process.finish(() => resolve());
                            });
                        });
                    },

                    /**
                     * Execute the anti-login crash patch installation script
                     */
                    (): Promise<void> => {
                        return new Promise(async (resolve) => {
                            Process.run(`yes | bash '${patchDir}/patch_anti_logincrash.sh'`, {
                                cwd: await constants.paths.gameDir
                            }).then((process) => {
                                process.finish(() => resolve());
                            });
                        });
                    }
                ]
            });

            // When all the things above are done
            pipeline.then(() => {
                this.patchFinished = true;

                if (this.onPatchFinish)
                    this.onPatchFinish();
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
    public patchFinish(callback: () => void)
    {
        this.onPatchFinish = callback;

        if (this.patchFinished)
            callback();
    }
}

export default class Patch
{
    public static fetchTimeout: number|null = 3000;

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
            const patchUri = constants.uri.patch[source];

            fetch(`${patchUri}/raw/master/${version.replaceAll('.', '')}/patch.sh`, this.fetchTimeout)
                .then((patcherResponse) => {
                    // Return an error if patch's server is unavailable
                    if (patcherResponse.status === null)
                        reject(new Error(`${source} patch repository is unreachable`));

                    // If [version]/patch.sh file doesn't exist - it means
                    // that patch repo has no [version]
                    else if (patcherResponse.status === 404)
                        resolve(null);

                    // Otherwise it should be [preparation], [testing] or [stable]
                    else
                    {
                        
                        fetch(`${patchUri}/raw/master/${version.replaceAll('.', '')}/patch_files/unityplayer_patch.vcdiff`, this.fetchTimeout)
                            .then((response) => {
                                // Return an error if patch's server is unavailable
                                if (response.status === null)
                                    reject(new Error(`${source} patch repository is unreachable`));

                                // If [version]/patch_files/unityplayer_patch.vcdiff file doesn't exist
                                // then it's [preparation] state and Krock just moved patch.sh file to the [version] folder
                                else if (response.status === 404)
                                {
                                    resolve({
                                        version: version,
                                        state: 'preparation',
                                        applied: false,
                                        source: source
                                    });
                                }

                                // Otherwise it's [testing] or [stable]
                                else
                                {
                                    patcherResponse.body(this.fetchTimeout)
                                        .then((response) => {
                                            // Return an error if patch's server is unavailable
                                            if (response === '')
                                                reject(new Error(`${source} patch repository is unreachable`));

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

                                                const originalPlayer = /if \[ "\${sum}" != "([a-z0-9]{32})" \]; then/mg.exec(response);

                                                // If we could get original UnityPlayer.dll hash - then we can
                                                // compare it with actual UnityPlayer.dll hash and say whether the patch
                                                // was applied or not
                                                if (originalPlayer !== null)
                                                {
                                                    constants.paths.gameDir.then((gameDir) => {
                                                        Neutralino.filesystem.readBinaryFile(`${gameDir}/UnityPlayer.dll`)
                                                            .then((currPlayer: ArrayBuffer) => {
                                                                patchInfo.applied = md5(currPlayer) != originalPlayer[1];

                                                                resolve(patchInfo);
                                                            })
                                                            .catch(() => resolve(patchInfo));
                                                    });
                                                }

                                                else resolve(patchInfo);
                                            }
                                        });
                                }
                            });
                    }
                });
        });
    }

    /**
     * Get patch installation stream
     * 
     * @returns null if the latest available patch in preparation state
     * @returns rejects Error object if the patch's repositories are unreachable or they responded with an error
     */
    public static install(): Promise<Stream|null>
    {
        Debug.log({
            function: 'Patch.install',
            message: 'Installing the patch...'
        });

        return new Promise((resolve, reject) => {
            this.latest
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
