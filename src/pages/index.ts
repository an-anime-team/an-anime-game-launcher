import '../i18n';
import App from '../index.svelte';
import Archive from '../ts/core/Archive';
import Downloader from '../ts/core/Downloader';

declare const Neutralino;

Neutralino.init();

Neutralino.events.on('ready', () => import('../defaultSettings'));

Neutralino.events.on('windowClose', () => {
    Downloader.closeStreams(true);
    Archive.closeStreams(true);

    Neutralino.app.exit();
});

const app = new App({
    target: document.getElementById('app')!
});

export default app;
