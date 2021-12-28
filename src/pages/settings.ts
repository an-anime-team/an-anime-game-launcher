import '../i18n';
import Debug from '../ts/core/Debug';

import App from '../settings.svelte';

declare const Neutralino;

Neutralino.init();

Neutralino.events.on('windowClose', async () => {
    await Neutralino.storage.setData('log', JSON.stringify(Debug.getRecords()));

    Neutralino.app.exit();
});

const app = new App({
    target: document.getElementById('app')!
});

export default app;
