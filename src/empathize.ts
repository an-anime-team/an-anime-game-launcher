import {
    // Paths API
    path, dir,

    // Filesystem API
    fs,

    // Windows API
    Windows,

    // OS API
    Process, Tray, IPC, Notification, Archive, Package,

    // Network API
    fetch, Domain, Downloader,

    // Async API
    promisify,

    // Meta classes
    Cache, Configs, Debug
} from '@empathize/framework';

import YAML from 'yaml';

import constants from './ts/Constants';
import Launcher from './ts/Launcher';

Configs.file = constants.paths.config;
Cache.file = constants.paths.cache;

Configs.serialize = YAML.stringify;
Configs.unserialize = YAML.parse;

Configs.autoFlush = false;

const openWindow = Windows.open;

// Add handler on window opener to show windows in fullscreen mode on steam deck
// FIXME: what about `fullscreen: true`? Can't check it
Windows.open = async (name, options) => {
    return openWindow(name, options && await Launcher.isSteamOs() ?
        { ...options, width: window.screen.width, height: window.screen.height } : options);
};

export {
    // Paths API
    path, dir,

    // Filesystem API
    fs,

    // Windows API
    Windows,

    // OS API
    Process, Tray, IPC, Notification, Archive, Package,

    // Network API
    fetch, Domain, Downloader,

    // Async API
    promisify,

    // Meta classes
    Cache, Configs, Debug
};
