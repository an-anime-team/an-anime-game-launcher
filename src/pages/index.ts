import '../i18n';
import App from '../index.svelte';
import constants from '../ts/Constants';
import Archive from '../ts/core/Archive';
import Debug from '../ts/core/Debug';
import Downloader from '../ts/core/Downloader';

declare const Neutralino;

Neutralino.init();

Neutralino.events.on('ready', () => import('../defaultSettings'));

Neutralino.events.on('windowClose', () => {
    Downloader.closeStreams(true);
    Archive.closeStreams(true);

    constants.paths.launcherDir.then(async (path) => {
        const time = new Date;

        Neutralino.filesystem.getStats(`${path}/logs`)
            .then(() => saveLog())
            .catch(async () => {
                await Neutralino.filesystem.createDirectory(`${path}/logs`);

                saveLog();
            });

        const saveLog = async () => {
            const log = Debug.get().join("\r\n");

            if (log != '')
                await Neutralino.filesystem.writeFile(`${path}/logs/${time.getDate()}-${time.getMonth() + 1}-${time.getFullYear()}-${time.getHours()}-${time.getMinutes()}-${time.getSeconds()}.log`, log);

            Neutralino.app.exit();
        };
    });
});

const app = new App({
    target: document.getElementById('app')!
});

export default app;
