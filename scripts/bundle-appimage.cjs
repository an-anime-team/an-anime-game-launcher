const path = require('path');

exports.params = {
    // .desktop file properties
    desktop: {
        // Name field
        name: 'An Anime Game Launcher',

        // Path to the icon
        icon: path.join(__dirname, '../public/icons/64x64.png'),

        // Categories (defult is Utilities)
        categories: ['Game']
    },

    // Neutralino binary info
    binary: {
        // Name of the binary (cli.binaryName)
        name: 'an-anime-game-launcher',

        // Dist folder path
        dist: path.join(__dirname, '../dist')
    },

    // Should AppImage contain Neutralino's dependencies or not
    // If true, then AppImage will contain webkit2gtk
    includeLibraries: false,

    // Some files or folders to copy inside of the the AppImage
    copy: {
        'public': path.join(__dirname, '../dist/an-anime-game-launcher/public')
    },

    // Path to the appimage to save
    output: path.join(__dirname, '../dist/An Anime Game Launcher.AppImage'),

    // Application version
    version: '2.0.0-beta-2'
};
