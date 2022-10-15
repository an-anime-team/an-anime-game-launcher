import '../i18n';

declare const Neutralino;

export default async function doInit(App: any)
{
    Neutralino.init();

    window.isSteamOs = (await Neutralino.os.getEnv('SteamOS')) === '1';

    new App({
        target: document.getElementById('app')!
    });

    Neutralino.events.dispatch('ready');
};