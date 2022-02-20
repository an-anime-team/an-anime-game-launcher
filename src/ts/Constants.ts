import type { AvailableLocales } from './launcher/Locales';

import { Configs } from '../empathize';

import Game from './Game';

declare const Neutralino;
declare const NL_CWD;

class Prefix
{
    /**
     * Current prefix directory
     * 
     * @default "~/.local/share/anime-game-launcher/game"
     */
    public static get current(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(await Configs.get('folders.prefix') as string));
    }

    /**
     * Default prefix directory
     * 
     * @default "~/.local/share/anime-game-launcher/game"
     */
    public static get default(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(`${await Paths.launcherDir}/game`));
    }

    /**
     * Change prefix directory
     * 
     * @returns promise that indicates that the prefix path has been changed in config
     */
    public static set(location: string): Promise<void>
    {
        return Configs.set('prefix', location);
    }
}

class Paths
{
    /**
     * Directory where the launcher's executable stored
     */
    public static readonly appDir: string = NL_CWD;

    /**
     * Shaders directory
     * 
     * @default "[constants.paths.appDir]/public/shaders"
     */
    public static readonly shadersDir: string = `${this.appDir}/public/shaders`;

    /**
     * Locales directory
     * 
     * @default "[constants.paths.appDir]/public/locales"
     */
    public static readonly localesDir: string = `${this.appDir}/public/locales`;

    /**
     * Launcher data directory
     * 
     * @default "~/.local/share/anime-game-launcher"
     */
    public static get launcherDir(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(`${await Neutralino.os.getPath('data')}/anime-game-launcher`));
    }

    /**
     * Runners directory
     * 
     * @default "~/.local/share/anime-game-launcher/runners"
     */
    public static get runnersDir(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(`${await this.launcherDir}/runners`));
    }

    /**
     * DXVKs directory
     * 
     * @default "~/.local/share/anime-game-launcher/dxvks"
     */
    public static get dxvksDir(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(`${await this.launcherDir}/dxvks`));
    }

    /**
     * Config file
     * 
     * @default "~/.local/share/anime-game-launcher/config.yaml"
     */
    public static get config(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(`${await this.launcherDir}/config.yaml`));
    }

    /**
     * Cache file
     * 
     * @default "~/.local/share/anime-game-launcher/.cache.json"
     */
    public static get cache(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(`${await this.launcherDir}/.cache.json`));
    }

    public static readonly prefix = Prefix;

    /**
     * Temp directory
     * 
     * @default "~/.local/share/anime-game-launcher"
     * 
     * @returns "[folders.temp] config field"
     */
    public static get tempDir(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(await Configs.get('folders.temp') as string));
    }

    /**
     * Game directory
     * 
     * @default "~/.local/share/anime-game-launcher/game/drive_c/Program Files/[An Anime Game]"
     * 
     * @returns "[folders.game] config field"
     */
    public static get gameDir(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(await Configs.get('folders.game') as string));
    }

    /**
     * Game data directory
     * 
     * @default "~/.local/share/anime-game-launcher/game/drive_c/Program Files/[An Anime Game]/[An Anime Game]_Data"
     * 
     * @returns "[folders.game]/[An Anime Game]_Data"
     */
    public static get gameDataDir(): Promise<string>
    {
        return new Promise(async (resolve) => {
            const folder = await Game.server === 'global' ?
                constants.placeholders.uppercase.first + constants.placeholders.uppercase.second :
                constants.placeholders.uppercase.full.cn;

            resolve(`${await this.gameDir}/${folder}_Data`);
        });
    }

    /**
     * Game voice data directory
     * 
     * @default "~/.local/share/anime-game-launcher/game/drive_c/Program Files/[An Anime Game]/[An Anime Game]_Data/StreamingAssets/Audio/GeneratedSoundBanks/Windows"
     * 
     * @returns "[constants.paths.gameDataDir]/StreamingAssets/Audio/GeneratedSoundBanks/Windows"
     */
    public static get voiceDir(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(`${await this.gameDataDir}/StreamingAssets/Audio/GeneratedSoundBanks/Windows`));
    }

    /**
     * FPS Unlock directory
     * 
     * @default "~/.local/share/anime-game-launcher/game/drive_c/Program Files/fpsunlock"
     * 
     * @returns "[constants.paths.prefix.current]/drive_c/Program Files/fpsunlock"
     */
    public static get fpsunlockDir(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(`${await this.prefix.current}/drive_c/Program Files/fpsunlock`));
    }
}

export default class constants
{
    public static readonly placeholders = {
        uppercase:
        {
            /**
             * Anime
             */
            first: atob('R2Vuc2hpbg=='),

            /**
             * Game
             */
            second: atob('SW1wYWN0'),

            /**
             * Anime Game
             */
            full: {
                global: atob('R2Vuc2hpbiBJbXBhY3Q='),
                cn: atob('WXVhblNoZW4=')
            },

            /**
             * anAnimeCompany
             */
            company: atob('bWlIb1lv'),

            /**
             * NOTAREALANIMECOMPANY
             */
            company_alterego: atob('Q09HTk9TUEhFUkU=')
        },

        lowercase:
        {
            /**
             * anime
             */
            first: atob('Z2Vuc2hpbg=='),

            /**
             * animecompany
             */
            company: atob('bWlob3lv')
        }
    };

    public static readonly api = {
        key: {
            global: 'gcStgarh',
            cn: 'eYd89JmJ'
        },
        launcher_id: {
            global: 10,
            cn: 18
        }
    };

    public static readonly uri = {
        api: {
            global: `https://sdk-os-static.${this.placeholders.lowercase.company}.com/hk4e_global/mdk/launcher/api`,
            cn: `https://sdk-static.${this.placeholders.lowercase.company}.com/hk4e_cn/mdk/launcher/api`
        },
        patch: {
            origin: 'https://notabug.org/Krock/dawn',
            additional: 'https://dev.kaifa.ch/Maroxy/dawn'
        },
        telemetry: {
            global: [
                atob('bG9nLXVwbG9hZC1vcy5taWhveW8uY29t'),
                atob('b3ZlcnNlYXVzcGlkZXIueXVhbnNoZW4uY29t')
            ],
            cn: [
                atob('bG9nLXVwbG9hZC5taWhveW8uY29t'),
                atob('dXNwaWRlci55dWFuc2hlbi5jb20=')
            ]
        },
        winetricks: 'https://raw.githubusercontent.com/Winetricks/winetricks/master/src/winetricks',
        fpsunlock: {
            unlocker: `https://github.com/34736384/${this.placeholders.lowercase.first}-fps-unlock/releases/download/v1.4.2/unlockfps.exe`,
            bat: 'https://dev.kaifa.ch/Maroxy/an-anime-game-aur/raw/branch/fpsunlock/fpsunlock.bat'
        },
        launcher: 'https://gitlab.com/KRypt0n_/an-anime-game-launcher',
        discord: 'https://discord.gg/ck37X6UWBp',
        analytics: 'https://aagl.launcher.moe/stat/'
    };

    public static readonly paths = Paths;

    public static versionsUri(server: 'global' | 'cn'): string
    {
        return `${this.uri.api[server]}/resource?key=${this.api.key[server]}&launcher_id=${this.api.launcher_id[server]}`;
    }

    public static backgroundUri(server: 'global' | 'cn', lang: AvailableLocales): string
    {
        return `${this.uri.api[server]}/content?filter_adv=true&key=${this.api.key[server]}&launcher_id=${this.api.launcher_id[server]}&language=${lang}`;
    }
}
