import Configs from '../../Configs';
import constants from '../../Constants';
import Runners from '../../core/Runners';
import Launcher from '../../Launcher';
import Process from '../../neutralino/Process';

declare const Neutralino;

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        /**
         * Selecting wine executable
         */
        let wineExeutable = 'wine';

        const runner = await Runners.current;

        if (runner !== null)
        {
            wineExeutable = `${await constants.paths.runnersDir}/${runner.name}/${runner.files.wine}`;

            try
            {
                Neutralino.filesystem.getStats(wineExeutable);
            }

            catch
            {
                wineExeutable = 'wine';

                await Configs.set('runner', null);
            }
        }

        console.log(`Wine executable: ${wineExeutable}`);

        // Some special variables
        let env: any = {};

        /**
         * HUD
         */
        switch (await Configs.get('hud'))
        {
            case 'dxvk':
                env['DXVK_HUD'] = 'fps,frametimes';

                break;

            case 'mangohud':
                env['MANGOHUD'] = 1;

                break;
        }

        /**
         * Shaders
         */
        const shaders = await Configs.get('shaders');

        if (shaders !== null)
        {
            const userShadersFile = `${constants.paths.shadersDir}/${shaders}/vkBasalt.conf`;
            const launcherShadersFile = `${await constants.paths.launcherDir}/vkBasalt.conf`;

            env['ENABLE_VKBASALT'] = 1;
            env['VKBASALT_CONFIG_FILE'] = launcherShadersFile;

            await Neutralino.filesystem.writeFile(launcherShadersFile, await Neutralino.filesystem.readFile(userShadersFile));
        }

        /**
         * GPU selection
         */
        /*if (LauncherLib.getConfig('gpu') != 'default')
        {
            const gpu = await SwitcherooControl.getGpuByName(LauncherLib.getConfig('gpu'));

            if (gpu)
            {
                env = {
                    ...env,
                    ...SwitcherooControl.getEnvAsObject(gpu)
                };
            }
            
            else console.warn(`GPU ${LauncherLib.getConfig('gpu')} not found. Launching on the default GPU`);
        }*/

        // let command = `${wineExeutable} ${LauncherLib.getConfig('fpsunlock') ? 'fpsunlock.bat' : 'launcher.bat'}`;

        /**
         * Gamemode integration
         */
        /*if (LauncherLib.getConfig('gamemode'))
            command = `gamemoderun ${command}`;*/

        const command = `${wineExeutable} launcher.bat`;

        console.log(`Execution command: ${command}`);

        /**
         * Starting the game
         */
        const startTime = Date.now();

        const process = await Process.run(command, {
            env: {
                ...env,

                WINEPREFIX: await constants.paths.prefix.current
            },
            cwd: await constants.paths.gameDir
        });

        // Game closed event
        process.finish(() => {
            const stopTime = Date.now();

            // todo

            resolve();
        });
    });
};
