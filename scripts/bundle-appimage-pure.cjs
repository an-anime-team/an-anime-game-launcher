const { params } = require('./bundle-appimage.cjs');

// Require bundler
const { Bundler } = require('neutralino-appimage-bundler');

// Create an object with some params
const bundler = new Bundler({
    ...params,
    
    includeLibraries: false
});

// Bundle project
bundler.bundle();
