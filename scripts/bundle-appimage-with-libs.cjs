const path = require('path');

const { params } = require('./bundle-appimage.cjs');

// Require bundler
const { Bundler } = require('neutralino-appimage-bundler');

// Create an object with some params
const bundler = new Bundler({
    ...params,
    
    includeLibraries: true,

    output: path.join(__dirname, '../dist/An Anime Game Launcher Prebundled.AppImage')
});

// Bundle project
bundler.bundle();
