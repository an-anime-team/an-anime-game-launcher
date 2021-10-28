const { ipcRenderer } = require('electron');

import $ from 'cash-dom';
import { Genshinlib } from './lib/Genshinlib';
import { LauncherUI } from './lib/LauncherUI';

$(() => {
    LauncherUI.updateLang(Genshinlib.getConfig('lang.launcher'));

    $('#participate').on('click', async () => {
        await fetch(`https://an-anime-game-launcher.000webhostapp.com${ !$('#share-country').hasClass('checkbox-active') ? '/?hide-geo' : '' }`);

        Genshinlib.updateConfig('analytics', Genshinlib.version);
        
        ipcRenderer.invoke('hide-analytics-participation');
    });

    $('#skip').on('click', () => {
        Genshinlib.updateConfig('analytics', Genshinlib.version);

        ipcRenderer.invoke('hide-analytics-participation');
    });

    $('#skip-and-ignore').on('click', () => {
        Genshinlib.updateConfig('analytics', null);

        ipcRenderer.invoke('hide-analytics-participation');
    });
});
