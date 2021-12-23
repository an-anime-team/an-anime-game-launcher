import type {
    PatchState,
    PatchInfo
} from './types/Patch';

import md5 from 'js-md5';

import constants from './Constants';
import Game from './Game';
import fetch from './core/Fetch';

export default class Patch
{
    public static fetchTimeout: number = 3000;

    /**
     * Get information about latest available patch
     * 
     * @returns rejects Error object if the patch's repositories are unreachable or they responded with an error
     */
    public static get latest(): Promise<PatchInfo>
    {
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
                            else resolve(patchInfo);
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
     * is more modern that the one that this patch was made for because
     * this field actually compares some files hashes
     * 
     * @returns null if patch with given version doesn't exist in given source
     * @returns rejects Error object if the source is unreachable or it responded with an error
     */
    public static getPatchInfo(version: string, source: 'origin' | 'additional' = 'origin'): Promise<PatchInfo|null>
    {
        return new Promise(async (resolve, reject) => {
            const patchUri = constants.uri.patch[source];

            // @ts-expect-error
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
                        // @ts-expect-error
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
                                        applied: false
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
                                                    applied: false
                                                };

                                                const originalPlayer = /if \[ "\${sum}" != "([a-z0-9]{32})" \]; then/mg.exec(response);

                                                // If we could get original UnityPlayer.dll hash - then we can
                                                // compare it with actual UnityPlayer.dll hash and say whether the patch
                                                // was applied or not
                                                if (originalPlayer !== null)
                                                    patchInfo.applied = md5(`${constants.paths.gameDir}/UnityPlayer.dll`) != originalPlayer[1];

                                                resolve(patchInfo);
                                            }
                                        });
                                }
                            });
                    }
                });
        });
    }
}
