const path = require('path');
const os = require('os');
const fs = require('fs');

import LauncherLib from "./LauncherLib";

export default class constants
{
    public static readonly placeholders = {
        uppercase:
        {
            first: Buffer.from('R2Vuc2hpbg==', 'base64').toString(),
            second: Buffer.from('SW1wYWN0', 'base64').toString(),
            full: Buffer.from('R2Vuc2hpbiBJbXBhY3Q=', 'base64').toString(),
            company: Buffer.from('bWlIb1lv', 'base64').toString()
        },

        lowercase:
        {
            first: Buffer.from('Z2Vuc2hpbg==', 'base64').toString(),
            company: Buffer.from('bWlob3lv', 'base64').toString()
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

    // TODO: dirs pathes

    public static readonly appDir = path.resolve(__dirname, '..', '..', '..');
    public static readonly shadersDir = path.join(this.appDir, 'public', 'shaders');

    public static readonly launcherDir: string = path.join(os.homedir(), '.local', 'share', 'anime-game-launcher');

    public static readonly runnersDir: string = path.join(this.launcherDir, 'runners');
    public static readonly dxvksDir: string = path.join(this.launcherDir, 'dxvks');

    public static readonly versionsUri: string = `${this.uri.api}/resource?key=gcStgarh&launcher_id=10`;
    public static readonly backgroundUri: string = `${this.uri.api}/content?filter_adv=true&launcher_id=10&key=gcStgarh&language=`;
    
    public static readonly patchUri: string = `${this.uri.patch}/archive/master.zip`;

    public static readonly runnersUri: string = `${this.uri.launcher}/raw/main/runners.json`;
    public static readonly dxvksUri: string = `${this.uri.launcher}/raw/main/dxvks.json`;

    public static prefixDir = new class
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
    }

    public static get gameDir(): string
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
}
