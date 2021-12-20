class Dirs
{
    /**
     * Directory where the launcher's executable stored
     * 
     * @returns string
     */
    // @ts-expect-error
    public static readonly app: string = NL_PATH;

    /**
     * Shaders directory
     * 
     * Default is [constants.dirs.app]/public/shaders
     * 
     * @returns string
     */
    public static readonly shaders: string = `${this.app}/public/shaders`;

    /**
     * Launcher data directory
     * 
     * Default is ~/.local/share/anime-game-launcher
     * 
     * @returns Promise<string>
     */
    public static get launcher(): Promise<string>
    {
        // @ts-expect-error
        return new Promise(async (resolve) => resolve(`${await Neutralino.os.getPath('data')}/anime-game-launcher`));
    }

    /**
     * Runners directory
     * 
     * Default is ~/.local/share/anime-game-launcher/runners
     * 
     * @returns Promise<string>
     */
    public static get runners(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(`${await this.launcher}/runners`));
    }

    /**
     * DXVKs directory
     * 
     * Default is ~/.local/share/anime-game-launcher/dxvks
     * 
     * @returns Promise<string>
     */
    public static get dxvks(): Promise<string>
    {
        return new Promise(async (resolve) => resolve(`${await this.launcher}/dxvks`));
    }

    /*public static readonly prefix = new class
    {
        /**
         * Current prefix directory
         * 
         * Default is ~/.local/share/anime-game-launcher/game
         * 
         * @returns string
         */
        /*public get(): string
        {
            return LauncherLib.getConfig('prefix');
        }

        public getDefault(): string
        {
            return path.join(os.homedir(), '.local', 'share', 'anime-game-launcher', 'game');
        }

        public set(location: string)
        {
            if (path.relative(LauncherLib.getConfig('prefix'), location) === '')
                return console.log('Can\'t set already selected prefix as new prefix');

            const dataPath = path.join(location, 'drive_c', 'Program Files', constants.placeholders.uppercase.full, `${constants.placeholders.uppercase.first + constants.placeholders.uppercase.second}_Data`);

            LauncherLib.updateConfig('prefix', location);
            LauncherLib.updateConfig('version', LauncherLib.getGameVersion(dataPath));
        }
    }*/
}

export default class constants
{
    public static readonly placeholders = {
        uppercase:
        {
            first: 'Genshin',
            second: 'Impact',
            full: 'Genshin Impact',
            company: 'miHoYo'
        },

        lowercase:
        {
            first: 'genshin',
            company: 'mihoyo'
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

    public static readonly dirs = Dirs;

    public static readonly versionsUri: string = `${this.uri.api}/resource?key=gcStgarh&launcher_id=10`;
    public static readonly backgroundUri: string = `${this.uri.api}/content?filter_adv=true&launcher_id=10&key=gcStgarh&language=`;

    public static readonly runnersUri: string = `${this.uri.launcher}/raw/main/runners.json`;
    public static readonly dxvksUri: string = `${this.uri.launcher}/raw/main/dxvks.json`;

    /*public static prefixDir = new class
    {
        public get(): string
        {
            return LauncherLib.getConfig('prefix');
        }

        public getDefault(): string
        {
            return path.join(os.homedir(), '.local', 'share', 'anime-game-launcher', 'game');
        }

        public set(location: string)
        {
            if (path.relative(LauncherLib.getConfig('prefix'), location) === '')
                return console.log('Can\'t set already selected prefix as new prefix');

            const dataPath = path.join(location, 'drive_c', 'Program Files', constants.placeholders.uppercase.full, `${constants.placeholders.uppercase.first + constants.placeholders.uppercase.second}_Data`);

            LauncherLib.updateConfig('prefix', location);
            LauncherLib.updateConfig('version', LauncherLib.getGameVersion(dataPath));
        }
    }*/

    /*public static get gameDir(): string
    {
        return path.join(this.prefixDir.get(), 'drive_c', 'Program Files', this.placeholders.uppercase.full);
    }

    public static get fpsunlockerDir(): string
    {
        return path.join(this.prefixDir.get(), 'drive_c', 'Program Files', Buffer.from('R0lfRlBTVW5sb2NrZXI=', 'base64').toString());
    }

    public static get voiceDir(): string
    {
        return path.join(this.gameDir, `${this.placeholders.uppercase.first + this.placeholders.uppercase.second}_Data`, 'StreamingAssets', 'Audio', 'GeneratedSoundBanks', 'Windows');
    }

    public static getPatchUri(source: 'origin' | 'additional'): string
    {
        return `${this.uri.patch[source]}/archive/master.zip`;
    }*/
}
