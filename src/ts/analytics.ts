const { ipcRenderer } = require('electron');

import $ from 'cash-dom';

import LauncherLib from './lib/LauncherLib';
import LauncherUI from './lib/LauncherUI';

$(() => {
    LauncherUI.updateLang(LauncherLib.getConfig('lang.launcher'));

    $('#participate').on('click', async () => {
        await fetch(`https://an-anime-game-launcher.000webhostapp.com${ !$('#share-country').hasClass('checkbox-active') ? '/?hide-geo' : '' }`);

        // LauncherLib.version can break this property
        // because analytics can be displayed even with the first
        // launcher's run and then of course uninstalled game's version
        // will be "null", which in analytics means that user don't
        // want to see this dialog anymore
        LauncherLib.updateConfig('analytics', (await LauncherLib.getData()).game.latest.version);
        
        ipcRenderer.invoke('hide-analytics-participation');
    });

    $('#skip').on('click', async () => {
        LauncherLib.updateConfig('analytics', (await LauncherLib.getData()).game.latest.version);

        ipcRenderer.invoke('hide-analytics-participation');
    });

    $('#skip-and-ignore').on('click', () => {
        LauncherLib.updateConfig('analytics', null);

        ipcRenderer.invoke('hide-analytics-participation');
    });
});
