import { Downloader, path } from '../empathize';

import constants from './Constants';

declare const Neutralino;

export default class FPSUnlock
{
    /**
     * Check if the FPS Unlock installed
     */
    public static installed(): Promise<boolean>
    {
        return new Promise(async (resolve) => {
            Neutralino.filesystem.getStats(await constants.paths.fpsunlockDir)
                .then(() => resolve(true))
                .catch(() => resolve(false));
        });
    }

    /**
     * Install FPS unlocker
     */
    public static install(): Promise<void>
    {
        return new Promise(async (resolve) => {
            const fpsunlockDir = await constants.paths.fpsunlockDir;

            await Neutralino.filesystem.createDirectory(fpsunlockDir);

            Downloader.download(constants.uri.fpsunlock.unlocker, `${fpsunlockDir}/unlockfps.exe`).then((stream) => {
                stream.finish(async () => {
                    const fpsunlockBat = `${await constants.paths.gameDir}/unlockfps.bat`;

                    Downloader.download(constants.uri.fpsunlock.bat, fpsunlockBat).then((stream) => {
                        stream.finish(async () => {
                            // sed -i 's/start ..\/GI_FPSUnlocker\/unlockfps.exe \%\*/start ..\/fpsunlock\/unlockfps.exe \%\*/g' unlockfps.bat
                            Neutralino.os.execCommand(`sed -i 's/start ..\\/GI_FPSUnlocker\\/unlockfps.exe \\%\\*/start ..\\/fpsunlock\\/unlockfps.exe \\%\\*/g' "${path.addSlashes(fpsunlockBat)}"`)
                                .then(() => resolve());
                        });
                    });
                });
            });
        });
    }
};
