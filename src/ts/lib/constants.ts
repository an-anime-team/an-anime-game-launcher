const path = require('path');
const os = require('os');

export class constants
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
        patch: 'https://notabug.org/Krock/GI-on-Linux',
        launcher: 'https://notabug.org/nobody/an-anime-game-launcher',
        telemetry: [
            `log-upload-os.${this.placeholders.lowercase.company}.com`,
            'overseauspider.yuanshen.com'
        ]
    };

    public static readonly launcherDir: string = path.join(os.homedir(), '.local', 'share', 'anime-game-launcher');

    public static readonly prefixDir: string = path.join(this.launcherDir, 'game');
    public static readonly gameDir: string = path.join(this.prefixDir, 'drive_c', 'Program Files', this.placeholders.uppercase.full);
    public static readonly voiceDir: string = path.join(this.gameDir, `${this.placeholders.uppercase.first + this.placeholders.uppercase.second}_Data`, 'StreamingAssets', 'Audio', 'GeneratedSoundBanks', 'Windows');

    public static readonly runnersDir: string = path.join(this.launcherDir, 'runners');
    public static readonly dxvksDir: string = path.join(this.launcherDir, 'dxvks');

    public static readonly versionsUri: string = `${this.uri.api}/resource?key=gcStgarh&launcher_id=10`;
    public static readonly backgroundUri: string = `${this.uri.api}/content?filter_adv=true&launcher_id=10&language=`;
    
    public static readonly patchUri: string = `${this.uri.patch}/archive/master.zip`;

    public static readonly runnersUri: string = `${this.uri.launcher}/raw/main/runners.json`;
    public static readonly dxvksUri: string = `${this.uri.launcher}/raw/main/dxvks.json`;
}
