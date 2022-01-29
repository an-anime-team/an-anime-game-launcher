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
