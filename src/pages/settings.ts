import '../i18n';
import Debug from '../ts/core/Debug';

import App from '../settings.svelte';
import IPC from '../ts/core/IPC';

declare const Neutralino;

Neutralino.init();

Neutralino.events.on('windowClose', async () => {
    await IPC.write({
        type: 'log',
        records: Debug.getRecords()
    });

    Neutralino.app.exit();
});

const app = new App({
    target: document.getElementById('app')!
});

export default app;
