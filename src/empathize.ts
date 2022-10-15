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

Configs.file = constants.paths.config;
Cache.file = constants.paths.cache;

Configs.serialize = YAML.stringify;
Configs.unserialize = YAML.parse;

Configs.autoFlush = false;

const openWindow = Windows.open
Windows.open = function (name, options) {
    if (isSteamOs && options) {
        // return openWindow(name, { ...options, width: window.screen.width, height: window.screen.height })
        return openWindow(name, { ...options, width: 1280, height: 800 })
    }
    return openWindow(name, options)
}

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
