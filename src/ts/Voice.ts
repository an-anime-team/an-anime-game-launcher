import type { VoicePack } from './types/GameData';
import type { VoiceLang, InstalledVoice } from './types/Voice';

import type { Stream as DownloadingStream } from '@empathize/framework/dist/network/Downloader';

import { Configs, Debug, Downloader, promisify, path } from '../empathize';
import { DebugThread } from '@empathize/framework/dist/meta/Debug';

import constants from './Constants';
import Game from './Game';
import AbstractInstaller from './core/AbstractInstaller';

declare const Neutralino;

class Stream extends AbstractInstaller
{
    public constructor(uri: string, predownloaded: boolean = false)
    {
        super(uri, constants.paths.gameDir, predownloaded);
    }
}

export default class Voice
{
    public static readonly langs = {
        'en-us': 'English(US)',
        'ja-jp': 'Japanese',
        'ko-kr': 'Korean',
        'zh-cn': 'Chinese'
    };

    /**
     * Get the list of the installed voice packages
     */
    public static get installed(): Promise<InstalledVoice[]>
    {
        return new Promise((resolve) => {
            Game.getLatestData()
                .then(async (data) => {
                    const voiceDir = await constants.paths.voiceDir;

                    let installedVoices: InstalledVoice[] = [];

                    // Parse installed voice packages
                    Neutralino.filesystem.readDirectory(voiceDir)
                        .then(async (files) => {
                            files = files.filter((file) => file.type == 'DIRECTORY')
                                .map((file) => file.entry);

                            for (const folder of Object.values(this.langs))
                                if (files.includes(folder))
                                {
                                    // Since anime company changed the way they store voice packages data
                                    // now to identify its version I want to calculate the actual
                                    // size of the voice package directory and compare it with all the
                                    // remotely available voice packages sizes. The closest one is the actual version of the package

                                    const actualSize = parseInt((await Neutralino.os.execCommand(`du -b "${path.addSlashes(`${voiceDir}/${folder}`)}"`))
                                        .stdOut.split('\t')[0]);

                                    const locale = Object.keys(this.langs).find((lang) => this.langs[lang] === folder);

                                    // This constant found its origin in the change of the voice packages format.
                                    // When the Anime Company decided that they know better how their game should work
                                    // and changed voice files names to some random numbers it caused issue when
                                    // old files aren't being replaced by the new ones, obviously because now they have
                                    // different names. When you download new voice package - its size will be something like 9 GB.
                                    // But Company's API returns double of this size, so like 18 GB, because their API also
                                    // messed folder where they store unpacked voice packages.
                                    // That's why we have to substract this approximate value from all the packages sizes
                                    const CONSTANT_OF_STUPIDITY = 9.37 * 1024.0 * 1024.0 * 1024.0;

                                    let version: { version: string | null, size: number, diff: number } = {
                                        version: null,
                                        size: 0,
                                        diff: actualSize
                                    };

                                    for (const voicePackage of data.game.latest.voice_packs)
                                        if (voicePackage.language == locale)
                                        {
                                            const packageSize = parseInt(voicePackage.size) - CONSTANT_OF_STUPIDITY;

                                            version = {
                                                version: data.game.latest.version,
                                                size: packageSize,
                                                diff: Math.abs(packageSize - actualSize)
                                            };

                                            break;
                                        }

                                    for (const diff of data.game.diffs)
                                        for (const voicePackage of diff.voice_packs)
                                            if (voicePackage.language == locale)
                                            {
                                                const packageSize = parseInt(voicePackage.size) - CONSTANT_OF_STUPIDITY;
                                                const sizesDiff = Math.abs(packageSize - actualSize);

                                                // If this version size closer to the actual size
                                                if (sizesDiff < version.diff)
                                                {
                                                    version = {
                                                        version: diff.version,
                                                        size: packageSize,
                                                        diff: sizesDiff
                                                    };
                                                }
                                                
                                                break;
                                            }

                                    installedVoices.push({
                                        lang: locale,

                                        // If the difference is too big - we expect this voice package
                                        // to be like really old, and we can't predict its version
                                        // for now this difference is 8 GB. Idk which value is better
                                        // This one should work fine for 2.5.0 - 2.8.0 versions window
                                        version: version.diff < 8 * 1024 * 1024 * 1024 ? version.version : null
                                    } as InstalledVoice);
                                }

                                resolveVoices();
                        })
                        .catch(() => resolveVoices());

                    // Parse active voice package
                    const resolveVoices = () => {
                        Debug.log({
                            function: 'Voice.current',
                            message: `Installed voices: ${installedVoices.map((voice) => `${voice.lang} (${voice.version})`).join(', ')}`
                        });

                        resolve(installedVoices);
                    };
                })
                .catch(() => resolve([]));
        });
    }

    /**
     * Get currently selected voice packages according to the config file
     */
    public static get selected(): Promise<VoiceLang[]>
    {
        return Configs.get('lang.voice') as Promise<VoiceLang[]>;
    }

    /**
     * Get latest voice data info
     * 
     * @returns Latest Voice Pack information else throws Error if company's servers are unreachable or if they responded with an error
     */
    public static get latest(): Promise<VoicePack[]>
    {
        return new Promise((resolve, reject) => {
            Game.getLatestData()
                .then((data) => resolve(data.game.latest.voice_packs))
                .catch((error) => reject(error));
        });
    }

    /**
     * Get updated voice data from the specified version to the latest
     * 
     * @returns null if the difference can't be calculated
     * @returns Error object if company's servers are unreachable or they responded with an error
     */
    public static getDiff(version: string): Promise<VoicePack[]|null>
    {
        return new Promise((resolve, reject) => {
            Game.getDiff(version)
                .then((data) => resolve(data?.voice_packs ?? null))
                .catch((error) => reject(error));
        });
    }

    /**
     * Get the voice data installation stream
     * 
     * @returns null if the language or the version can't be found
     * @returns rejects Error object if company's servers are unreachable or they responded with an error
     */
    public static update(lang: VoiceLang, version: string|null = null): Promise<Stream|null>
    {
        Debug.log({
            function: 'Voice.update',
            message: version !== null ?
                `Updating ${lang} voice package from the ${version} version` :
                `Installing ${lang} voice package`
        });

        return new Promise((resolve, reject) => {
            this.isUpdatePredownloaded(lang).then(async (predownloaded) => {
                if (predownloaded)
                {
                    Debug.log({
                        function: 'Voice.update',
                        message: 'Voice package update is already pre-downloaded. Unpacking started'
                    });

                    resolve(new Stream(`${await constants.paths.launcherDir}/voice-${lang}-predownloaded.zip`, true));
                }

                else
                {
                    (version === null ? this.latest : this.getDiff(version))
                        .then((data: VoicePack[]|null) => {
                            if (data === null)
                                resolve(null);

                            else
                            {
                                const voice = data.filter(voice => voice.language === lang);

                                resolve(voice.length === 1 ? new Stream(voice[0].path) : null);
                            }
                        })
                        .catch((error) => reject(error));
                }
            });
        });
    }

    /**
     * Delete specified voice package
     */
    public static delete(lang: VoiceLang): Promise<void>
    {
        const debugThread = new DebugThread('Voice.delete', `Deleting ${this.langs[lang]} (${lang}) voice package`);
        
        return new Promise(async (resolve) => {
            const voiceDir = await constants.paths.voiceDir;

            const pipeline = promisify({
                callbacks: [
                    () => Neutralino.os.execCommand(`rm -rf "${path.addSlashes(`${voiceDir}/${this.langs[lang]}`)}"`),

                    (): Promise<void> => new Promise(async (resolve) => {
                        Neutralino.os.execCommand(`rm -f "${path.addSlashes(`${await constants.paths.gameDir}/Audio_${this.langs[lang]}_pkg_version`)}"`)
                            .then(() => resolve());
                    }),

                    (): Promise<void> => new Promise(async (resolve) => {
                        Neutralino.os.execCommand(`sed -i '/${this.langs[lang]}/d' "${path.addSlashes(`${await constants.paths.gameDataDir}/Persistent/audio_lang_14`)}"`)
                            .then(() => resolve());
                    })
                ],
                interval: 200,
                callAtOnce: true
            });
            
            pipeline.then(() => {
                debugThread.log('Voice package deleted');

                resolve();
            });
        });
    }

    /**
     * Pre-download the game's voice update
     * 
     * @param version current game version to download difference from
     * 
     * @returns null if the game pre-downloading is not available or the language wasn't found. Otherwise - downloading stream
     * @returns Error if company's servers are unreachable or they responded with an error
     */
    public static predownloadUpdate(lang: string, version: string|null = null): Promise<DownloadingStream|null>
    {
        const debugThread = new DebugThread('Voice.predownloadUpdate', `Predownloading ${lang} game voice data...`)

        return new Promise((resolve) => {
            Game.getLatestData()
                .then((data) => {
                    if (data.pre_download_game)
                    {
                        let voicePack = data.pre_download_game.latest.voice_packs.filter(voice => voice.language === lang);

                        if (version !== null)
                            for (const diff of data.pre_download_game.diffs)
                                if (diff.version == version)
                                {
                                    voicePack = diff.voice_packs.filter(voice => voice.language === lang);

                                    break;
                                }

                        if (voicePack.length === 1)
                        {
                            debugThread.log(`Downloading update from the path: ${voicePack[0].path}`);

                            constants.paths.launcherDir.then((dir) => {
                                Downloader.download(voicePack[0].path, `${dir}/voice-${lang}-predownloaded.zip`)
                                    .then((stream) => resolve(stream));
                            });
                        }

                        else resolve(null);
                    }

                    else resolve(null);
                })
                .catch((error) => resolve(error));
        });
    }

    /**
     * Checks whether the update was downloaded or not
     */
    public static isUpdatePredownloaded(lang: VoiceLang|VoiceLang[]): Promise<boolean>
    {
        return new Promise(async (resolve) => {
            if (typeof lang === 'string')
            {
                Neutralino.filesystem.getStats(`${await constants.paths.launcherDir}/voice-${lang}-predownloaded.zip`)
                    .then(() => resolve(true))
                    .catch(() => resolve(false));
            }

            else
            {
                let predownloaded = true;

                for (const voiceLang of lang)
                    predownloaded &&= await this.isUpdatePredownloaded(voiceLang);

                resolve(predownloaded);
            }
        });
    }
}

export { Stream };
