import type { VoicePack } from './types/GameData';
import type { InstalledVoice, VoiceLang } from './types/Voice';

import constants from './Constants';
import Game from './Game';
import AbstractInstaller from './core/AbstractInstaller';
import Configs from './Configs';

declare const Neutralino;

class Stream extends AbstractInstaller
{
    public constructor(uri: string)
    {
        super(uri, constants.paths.gameDir);
    }
}

export default class Voice
{
    /**
     * Get current installed voice data info
     */
    public static get current(): Promise<InstalledVoice>
    {
        return new Promise(async (resolve) => {
            const persistentPath = `${await constants.paths.gameDataDir}/Persistent/audio_lang_14`;

            const langs = {
                'English(US)': 'en-us',
                'Japanese': 'ja-jp',
                'Korean': 'ko-kr',
                'Chinese': 'zn-cn'
            };

            let installedVoice: InstalledVoice = {
                installed: [],
                active: null
            };
            
            // Parse installed voice packages
            Neutralino.filesystem.readDirectory(await constants.paths.voiceDir)
                .then((files) => {
                    files = files
                        .filter((file) => file.type == 'DIRECTORY')
                        .map((file) => file.entry);

                    Object.keys(langs).forEach((folder) => {
                        if (files.includes(folder) && langs[folder] !== undefined)
                            installedVoice.installed.push(langs[folder]);
                    });
                })
                .catch(() => {});

            // Parse active voice package
            Neutralino.filesystem.readFile(persistentPath)
                .then((lang) => installedVoice.active = langs[lang] ?? null)
                .catch(() => {});

            resolve(installedVoice);
        });
    }

    /**
     * Get currently selected voice package language accotring to the config file
     */
    public static get selected(): Promise<VoiceLang>
    {
        return Configs.get('lang.voice') as Promise<VoiceLang>;
    }

    /**
     * Get latest voice data info
     * 
     * @returns rejects Error object if company's servers are unreachable or they responded with an error
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
     * @returns rejects Error object if company's servers are unreachable or they responded with an error
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
    public static update(lang: string|null = null, version: string|null = null): Promise<Stream|null>
    {
        return new Promise((resolve, reject) => {
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
        });
    }
}

export { Stream };
