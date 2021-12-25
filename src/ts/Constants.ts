import Configs from './Configs';

declare const Neutralino;
declare const NL_PATH;

class Prefix
{
    /**
     * Current prefix directory
     * 
     * @default "~/.local/share/anime-game-launcher/game"
     */
    public static get current(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(await Configs.get('prefix') as string));
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
     * @returns promise that indicates when the prefix path will be changed in configs
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
    public static readonly appDir: string = NL_PATH;

    /**
     * Shaders directory
     * 
     * @default "[constants.paths.app]/public/shaders"
     */
    public static readonly shadersDir: string = `${this.appDir}/public/shaders`;

    /**
     * Locales directory
     * 
     * @default "[constants.paths.app]/public/locales"
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
     * @default "~/.local/share/anime-game-launcher/cache.json"
     */
    public static get cache(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(`${await this.launcherDir}/cache.json`));
    }

    public static readonly prefix = Prefix;

    /**
     * Game directory
     * 
     * @default "~/.local/share/anime-game-launcher/game/drive_c/Program Files/[An Anime Game]"
     * 
     * @returns "[constants.paths.prefix.current]/drive_c/Program Files/[An Anime Game]"
     */
    public static get gameDir(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(`${await this.prefix.current}/drive_c/Program Files/${constants.placeholders.uppercase.full}`));
    }

    /**
     * Game data directory
     * 
     * @default "~/.local/share/anime-game-launcher/game/drive_c/Program Files/[An Anime Game]/[An Anime Game]_Data"
     * 
     * @returns "[constants.paths.gameDir]/[An Anime Game]_Data"
     */
    public static get gameDataDir(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(`${await this.gameDir}/${constants.placeholders.uppercase.first + constants.placeholders.uppercase.second}_Data`));
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
}

export default class constants
{
    public static readonly placeholders = {
        uppercase:
        {
            first: atob('R2Vuc2hpbg=='),
            second: atob('SW1wYWN0'),
            full: atob('R2Vuc2hpbiBJbXBhY3Q='),
            company: atob('bWlIb1lv')
        },

        lowercase:
        {
            first: atob('Z2Vuc2hpbg=='),
            company: atob('bWlob3lv')
        }
    };

    public static readonly uri = {
        api: `https://sdk-os-static.${this.placeholders.lowercase.company}.com/hk4e_global/mdk/launcher/api`,
        patch: {
            origin: 'https://notabug.org/Krock/dawn',
            additional: 'https://dev.kaifa.ch/Maroxy/dawn'
        },
        launcher: 'https://gitlab.com/KRypt0n_/an-anime-game-launcher',
        telemetry: [
            `log-upload-os.${this.placeholders.lowercase.company}.com`,
            'overseauspider.yuanshen.com'
        ],
        winetricks: 'https://raw.githubusercontent.com/Winetricks/winetricks/master/src/winetricks'
    };

    // TODO: cache drops at that dates instead of the 7 days period
    /*public static readonly cacheDropDates = [
        new Date('November 24, 2021').getTime(), // 2.3.0 half 1 release
        new Date('December 15, 2021').getTime(), // 2.3.0 half 2 release
        new Date('January 5, 2022').getTime() // 2.4.0 half 1 release
    ];*/

    public static readonly paths = Paths;

    public static readonly versionsUri: string = `${this.uri.api}/resource?key=gcStgarh&launcher_id=10`;
    public static readonly backgroundUri: string = `${this.uri.api}/content?filter_adv=true&launcher_id=10&key=gcStgarh&language=`;

    /**
     * Get a URI to the specified patch repository archive
     */
    public static getPatchUri(source: 'origin' | 'additional'): string
    {
        return `${this.uri.patch[source]}/archive/master.zip`;
    }
}
