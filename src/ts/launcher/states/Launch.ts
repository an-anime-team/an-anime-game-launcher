import type Launcher from '../../Launcher';

import Configs from '../../Configs';
import constants from '../../Constants';
import { DebugThread } from '../../core/Debug';
import Notifications from '../../core/Notifications';
import Runners from '../../core/Runners';
import Game from '../../Game';
import Process from '../../neutralino/Process';
import Window from '../../neutralino/Window';

import Baal from '../../../assets/images/baal64-transparent.png';

declare const Neutralino;

export default (launcher: Launcher): Promise<void> => {
    return new Promise(async (resolve) => {
        const debugThread = new DebugThread('State/Launch', 'Starting the game');

        const telemetry = await Game.isTelemetryDisabled();

        // If telemetry servers are not disabled
        if (!telemetry)
        {
            Notifications.show({
                title: 'An Anime Game Launcher',
                body: 'Telemetry servers are not disabled',
                icon: Baal,
                importance: 'critical'
            });

            debugThread.log('Telemetry is not disabled!');
        }
        
        // Otherwise run the game
        else
        {
            Window.current.hide();

            launcher.updateDiscordRPC('in-game');

            /**
             * Selecting wine executable
             */
            let wineExeutable = 'wine';

            const runner = await Runners.current();

            if (runner !== null)
                wineExeutable = `${await constants.paths.runnersDir}/${runner.name}/${runner.files.wine}`;

            debugThread.log(`Wine executable path: ${wineExeutable}`);

            // Some special variables
            let env: any = {};

            /**
             * HUD
             */
            switch (await Configs.get('hud'))
            {
                case 'dxvk':
                    env['DXVK_HUD'] = 'fps,frametimes,version,gpuload';

                    break;

                case 'mangohud':
                    env['MANGOHUD'] = 1;

                    break;
            }

            /**
             * Shaders
             */
            const shaders = await Configs.get('shaders');

            if (shaders !== 'none')
            {
                const launcherShadersFile = `${await constants.paths.launcherDir}/vkBasalt.conf`;

                env['ENABLE_VKBASALT'] = 1;
                env['VKBASALT_CONFIG_FILE'] = launcherShadersFile;

                if (shaders !== 'custom')
                {
                    const userShadersFile = `${constants.paths.shadersDir}/public/${shaders}/vkBasalt.conf`;
                    
                    await Neutralino.filesystem.writeFile(launcherShadersFile, await Neutralino.filesystem.readFile(userShadersFile));
                }
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

            let command = `"${Process.addSlashes(wineExeutable)}" ${await Configs.get('fps_unlocker') ? 'unlockfps.bat' : 'launcher.bat'}`;

            /**
             * Gamemode integration
             */
            if (await Configs.get('gamemode'))
                command = `gamemoderun ${command}`;

            /**
             * Starting the game
             */
            const startTime = Date.now();

            const process = await Process.run(command, {
                env: {
                    WINEPREFIX: await constants.paths.prefix.current,
                    ...env,
                    ...((await Configs.get('env') as object|null) ?? {})
                },
                cwd: await constants.paths.gameDir
            });

            // Game was started by the launcher.bat file
            // so we just need to wait until GenshinImpact process
            // will be closed
            process.finish(() => {
                const waiter = async () => {
                    const processes: string = (await Neutralino.os.execCommand('ps -A')).stdOut;

                    // Game is still running
                    if (processes.includes('GenshinImpact'))
                        setTimeout(waiter, 3000);

                    // Game was closed
                    else
                    {
                        const stopTime = Date.now();

                        Window.current.show();

                        launcher.updateDiscordRPC('in-launcher');

                        // TODO

                        resolve();
                    }
                };

                setTimeout(waiter, 5000);
            });
        }
    });
};
