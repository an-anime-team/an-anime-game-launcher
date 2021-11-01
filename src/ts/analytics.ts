const { ipcRenderer } = require('electron');

import $ from 'cash-dom';
import { LauncherLib } from './lib/LauncherLib';
import { LauncherUI } from './lib/LauncherUI';

$(() => {
    LauncherUI.updateLang(LauncherLib.getConfig('lang.launcher'));

    $('#participate').on('click', async () => {
        await fetch(`https://an-anime-game-launcher.000webhostapp.com${ !$('#share-country').hasClass('checkbox-active') ? '/?hide-geo' : '' }`);

        LauncherLib.updateConfig('analytics', LauncherLib.version);
        
        ipcRenderer.invoke('hide-analytics-participation');
    });

    $('#skip').on('click', () => {
        LauncherLib.updateConfig('analytics', LauncherLib.version);

        ipcRenderer.invoke('hide-analytics-participation');
    });

    $('#skip-and-ignore').on('click', () => {
        LauncherLib.updateConfig('analytics', null);

        ipcRenderer.invoke('hide-analytics-participation');
    });
});
