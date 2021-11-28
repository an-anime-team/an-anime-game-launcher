const fs = require('fs');
import LauncherLib from "./LauncherLib";
import constants from "./constants";
const path = require('path');
const os = require('os');

export default class PrefixSelector
{
    protected static prefix: string = LauncherLib.getConfig('prefix');

    public static set(location: string) {
        if (this.prefix == location) return console.log('Can\'t set already selected prefix as new prefix');

        if (fs.existsSync(path.join(location, 'drive_c', 'Program Files', 'Genshin Impact', 'GenshinImpact_Data', 'Persistent'))) {
            const version = fs.readFileSync(path.join(location, 'drive_c', 'Program Files', 'Genshin Impact', 'GenshinImpact_Data', 'Persistent', 'ScriptVersion'), { encoding: 'UTF-8' }).toString();

            LauncherLib.updateConfig('version', version);
            LauncherLib.updateConfig('prefix', location);
            constants.prefixDir = location;
            this.prefix = location;
        } else if (fs.existsSync(path.join(location, 'drive_c', 'Program Files', 'Genshin Impact', 'GenshinImpact_Data', 'globalgamemanagers'))) {
            const config = fs.readFileSync(path.join(location, 'drive_c', 'Program Files', 'Genshin Impact', 'GenshinImpact_Data', 'globalgamemanagers'), { encoding: 'ascii' });
            const version = /([1-9]+\.[0-9]+\.[0-9]+)_[\d]+_[\d]+/.exec(config)![1];

            LauncherLib.updateConfig('version', version);
            LauncherLib.updateConfig('prefix', location);
            constants.prefixDir = location;
            this.prefix = location;
        } else {
            console.log('Game not found.');

            // Unset version if game is not found.
            LauncherLib.updateConfig('version', null);
            LauncherLib.updateConfig('prefix', location);
            constants.prefixDir = location;
            this.prefix = location;
        }
    }

    public static Default() {
        const dp = path.join(os.homedir(), '.local', 'share', 'anime-game-launcher', 'game');

        if (this.prefix == dp) return console.log('Can\'t set already selected prefix as new prefix');

        if (fs.existsSync(path.join(dp, 'drive_c', 'Program Files', 'Genshin Impact', 'GenshinImpact_Data', 'Persistent'))) {
            const version = fs.readFileSync(path.join(dp, 'drive_c', 'Program Files', 'Genshin Impact', 'GenshinImpact_Data', 'Persistent', 'ScriptVersion'), { encoding: 'UTF-8' }).toString();
            
            LauncherLib.updateConfig('version', version);
            LauncherLib.updateConfig('prefix', dp);
            constants.prefixDir = dp;
            this.prefix = dp;
        } else if (fs.existsSync(path.join(dp, 'drive_c', 'Program Files', 'Genshin Impact', 'GenshinImpact_Data', 'globalgamemanagers'))) {
            const config = fs.readFileSync(path.join(dp, 'drive_c', 'Program Files', 'Genshin Impact', 'GenshinImpact_Data', 'globalgamemanagers'), { encoding: 'ascii' });
            const version = /([1-9]+\.[0-9]+\.[0-9]+)_[\d]+_[\d]+/.exec(config)![1];

            LauncherLib.updateConfig('version', version);
            LauncherLib.updateConfig('prefix', dp);
            constants.prefixDir = dp;
            this.prefix = dp;
        } else {
            console.log('Game not found.');

            // Unset version if game is not found.
            LauncherLib.updateConfig('version', null);
            LauncherLib.updateConfig('prefix', dp);
            constants.prefixDir = dp;
            this.prefix = dp;
        }
    }

}