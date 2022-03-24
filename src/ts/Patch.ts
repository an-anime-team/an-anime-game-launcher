import type { PatchInfo } from './types/Patch';

import { fetch, promisify, Debug, Cache, path, fs } from '../empathize';
import { DebugThread } from '@empathize/framework/dist/meta/Debug';

import constants from './Constants';
import Game from './Game';
import Launcher from './Launcher';
import md5 from './core/md5';

declare const Neutralino;

/**
 * I made a copy of AbstractInstaller here
 * because really don't want to make changes in AbstractInstaller
 * 
 * I tried and it became overcomplicated and messy
 */
class Stream
{
    protected onDownloadStart?: () => void;
    protected onApplyingStart?: () => void;

    protected onDownloadFinish?: () => void;
    protected onApplyingFinish?: (result: boolean) => void;

    protected downloadStarted: boolean = false;
    protected applyingStarted: boolean = false;

    protected downloadFinished: boolean = false;
    protected applyingFinished: boolean = false;

    protected patchResult: boolean = false;

    /**
     * @throws Error if the patch can't (or shouldn't) be applied
     */
    public constructor(patch: PatchInfo)
    {
        if (patch.applied || patch.source === undefined || patch.state == 'preparation')
            throw new Error('The patch is either already applied, can\'t be found or in the preparation state');
        
        const patchUri = constants.uri.patch[patch.source];

        const debugThread = new DebugThread('Patch/Stream', {
            message: {
                'patch uri': patchUri,
                'source': patch.source,
                'version': patch.version,
                'state': patch.state
            }
        });

        constants.paths.launcherDir.then(async (launcherDir) => {
            debugThread.log('Fetching patch repository...');

            this.downloadStarted = true;

            if (this.onDownloadStart)
                this.onDownloadStart();

            // Run `git clone` if the patch repo is not downloaded
            // or `git pull` to fetch changes
            await fs.exists(`${launcherDir}/patch`) ?
                await Neutralino.os.execCommand(`cd "${path.addSlashes(launcherDir)}/patch" && git fetch --all && git reset --hard origin/master`) :
                await Neutralino.os.execCommand(`git clone "${path.addSlashes(patchUri)}" "${path.addSlashes(launcherDir)}/patch"`);

            this.downloadFinished = true;

            if (this.onDownloadFinish)
                this.onDownloadFinish();

            debugThread.log('Patch repository fetched');

            const patchDir = `${launcherDir}/.patch-applying`;

            if (await fs.exists(patchDir))
                await fs.remove(patchDir);
            
            await fs.copy(`${launcherDir}/patch/${patch.version.replaceAll('.', '')}`, patchDir);

            this.applyingStarted = true;

            if (this.onApplyingStart)
                this.onApplyingStart();

            const gameDir = await constants.paths.gameDir;
            const isFlatpak = await Launcher.isFlatpak();

            debugThread.log('Applying patch...');

            /**
             * Patch out the testing phase content from the shell files
             * if active and make sure the shell files are executable
             */
            const pipeline = promisify({
                callbacks: [
                    /**
                     * Remove test version restrictions from the main patch
                     */
                    () => Neutralino.os.execCommand(`cd "${path.addSlashes(patchDir)}" && sed -i '/^echo " === !! UNTESTED PATCH. CHECK FOR BANS USING A TRASH ACCOUNT !! ==="/,+5d' patch.sh`),

                    /**
                     * Remove /etc/hosts editing if running in Flatpak
                     */
                    () => isFlatpak ? Neutralino.os.execCommand(`cd "${path.addSlashes(patchDir)}" && sed -i '/^# ===========================================================/,+79d' patch.sh`) : null,
                    
                    /**
                     * Remove test version restrictions from the anti-login crash patch
                     */
                    () => Neutralino.os.execCommand(`cd "${path.addSlashes(patchDir)}" && sed -i '/^#echo "       edit this script file to comment\\/remove the line below."/,+2d' patch_anti_logincrash.sh`),

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
            pipeline.then(async (outputs) => {
                this.applyingFinished = true;

                debugThread.log({
                    message: [
                        'Patch script output:',
                        ...outputs[5].stdOut.split(/\r\n|\r|\n/)
                    ]
                });

                // Remove temp patch dir
                await fs.remove(patchDir);

                this.patchResult = outputs[5].stdOut.includes('==> Patch applied! Enjoy the game');

                debugThread.log(`Patch applying result: ${this.patchResult ? 'success' : 'error'}`);

                if (this.onApplyingFinish)
                    this.onApplyingFinish(this.patchResult);
            });
        });
    }

    /**
     * Specify event that will be called after download has begun
     * 
     * @param callback
     */
    public downloadStart(callback: () => void)
    {
        this.onDownloadStart = callback;

        if (this.downloadStarted)
            callback();
    }

    /**
     * Specify event that will be called after the patch applying has begun
     * 
     * @param callback
     */
    public applyingStart(callback: () => void)
    {
        this.onApplyingStart = callback;

        if (this.applyingStarted)
            callback();
    }

    /**
     * Specify event that will be called after download has finished
     * 
     * @param callback
     */
    public downloadFinish(callback: () => void)
    {
        this.onDownloadFinish = callback;

        if (this.downloadFinished)
            callback();
    }

    /**
     * Specify event that will be called after the patch applying has finished
     * 
     * @param callback
     */
    public applyingFinish(callback: (result: boolean) => void)
    {
        this.onApplyingFinish = callback;

        if (this.applyingFinished)
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
                        const playerHash = await md5(`${await constants.paths.gameDir}/UnityPlayer.dll`);

                        if (playerHash !== null)
                            cache.value['output']['applied'] = playerHash != cache.value['playerHash'];
                    }

                    resolve(cache.value['output']);
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
                                .then(async (response) => {
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
                                            source: source,
                                            server: await Game.server
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
                                                    .then(async (response) => {
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
                                                                source: source,
                                                                server: await Game.server
                                                            };

                                                            const hashesMatches = [...response.matchAll(/if \[ "\${sum}" == "([a-z0-9]{32})" \]; then/mg)];

                                                            // If we could get original UnityPlayer.dll hash - then we can
                                                            // compare it with actual UnityPlayer.dll hash and say whether the patch
                                                            // was applied or not
                                                            if (hashesMatches.length === 2)
                                                            {
                                                                const originalPlayer = {
                                                                    global: hashesMatches[0][1],
                                                                    cn: hashesMatches[1][1]
                                                                }[patchInfo.server];

                                                                const playerHash = await md5(`${await constants.paths.gameDir}/UnityPlayer.dll`);

                                                                if (playerHash !== null)
                                                                    patchInfo.applied = playerHash != originalPlayer;

                                                                resolveOutput(patchInfo, originalPlayer);
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
                    
                    else resolve(new Stream(patch));
                })
                .catch((err) => reject(err));
        });
    }

    /**
     * Try to revert applied patch
     * 
     * @returns false if the locally installed patch repository is not available or the patch is not applied
     */
    public static revert(patch: PatchInfo): Promise<boolean>
    {
        return new Promise(async (resolve) => {
            const patchRevertFile = `${await constants.paths.launcherDir}/patch/${patch.version.replaceAll('.', '')}/patch_revert.sh`;

            if (!await fs.exists(patchRevertFile))
                resolve(false);

            else
            {
                const result = await Neutralino.os.execCommand(`chmod +x "${path.addSlashes(patchRevertFile)}" && cd "${path.addSlashes(await constants.paths.gameDir)}" && yes yes yes | bash "${path.addSlashes(patchRevertFile)}"`);

                resolve(result.stdOut.includes('==> Patch reverted'));
            }
        });
    }
}

export { Stream };
