import { Process, Windows, Configs, Notification, path, Package } from '../../../empathize';
import { DebugThread } from '@empathize/framework/dist/meta/Debug';

import constants from '../../Constants';
import Runners from '../../core/Runners';
import Game from '../../Game';
import Locales from '../Locales';
import DXVK from '../../core/DXVK';

import type Launcher from '../../Launcher';

declare const Neutralino;

export default (launcher: Launcher|null): Promise<void> => {
    return new Promise(async (resolve) => {
        const debugThread = new DebugThread('State/Launch', 'Starting the game');

        // Check if telemetry servers are disabled
        Game.isTelemetryDisabled()
            .then(async (telemetryDisabled) => {
                // If telemetry servers are not disabled
                if (!telemetryDisabled)
                {
                    Notification.show({
                        ...Locales.translate('notifications.telemetry_not_disabled'),
                        icon: `${constants.paths.appDir}/public/images/baal64-transparent.png`,
                        importance: 'critical'
                    });

                    debugThread.log('Telemetry is not disabled!');

                    resolve();
                }
                
                // Otherwise run the game
                else
                {
                    Windows.current.hide();

                    launcher?.updateDiscordRPC('in-game');

                    launcher?.tray.update([
                        { text: 'Starting the game...', disabled: true }
                    ]);

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

                    // If we're running dxvk-async
                    if ((await DXVK.current())?.version.includes('async'))
                        env['DXVK_ASYNC'] = 1;

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
                     * Wine synchronizations
                     * 
                     * @link https://github.com/AdelKS/LinuxGamingGuide#wine-tkg
                     */
                    switch (await Configs.get('wine.sync'))
                    {
                        case 'esync':
                            env['WINEESYNC'] = 1;

                            break;

                        case 'fsync':
                            env['WINEESYNC'] = 1;
                            env['WINEFSYNC'] = 1;

                            break;

                        case 'futex2':
                            env['WINEESYNC'] = 1;
                            env['WINEFSYNC'] = 1;
                            env['WINEFSYNC_FUTEX2'] = 1;

                            break;
                    }

                    /**
                     * AMD FSR
                     * 
                     * It should be disabled when wine virtual desktop is enabled
                     */
                    if (await Configs.get('wine.fsr') && !await Configs.get('wine.virtual_desktop.enabled'))
                    {
                        env['WINE_FULLSCREEN_FSR'] = 1;
                        env['WINE_FULLSCREEN_FSR_STRENGTH'] = 3;
                    }

                    /**
                     * Shaders
                     */
                    const shaders = await Configs.get('shaders');

                    if (shaders !== 'none' && await Package.exists('reshade'))
                    {
                        const launcherShadersFile = `${await constants.paths.launcherDir}/vkBasalt.conf`;

                        env['ENABLE_VKBASALT'] = 1;
                        env['VKBASALT_CONFIG_FILE'] = launcherShadersFile;

                        if (shaders !== 'custom')
                        {
                            const userShadersFile = `${constants.paths.shadersDir}/${shaders}/vkBasalt.conf`;
                            
                            await Neutralino.filesystem.writeFile(launcherShadersFile, await Neutralino.filesystem.readFile(userShadersFile));

                            /**
                             * Small workaround for notahuman's shaders
                             * because they require a file with an absolute path
                             * and we have to update it
                             */
                            if (shaders == 'notahuman')
                                await Neutralino.os.execCommand(`sed -i 's/\\/the\\/absolute\\/path\\/to\\/NFAA.fx/${constants.paths.shadersDir.replaceAll('/', '\\/')}\\/${shaders}\\/NFAA.fx/g' "${path.addSlashes(await constants.paths.launcherDir)}/vkBasalt.conf"`);
                        }
                    }

                    /**
                     * GPU selection
                     */
                    // TODO
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

                    const virtual_desktop = await Configs.get('wine.virtual_desktop') as object;
                    const borderless_window = await Configs.get('borderless_window') as boolean;

                    let command: string = [
                        `"${path.addSlashes(wineExeutable)}"`,
                        `${virtual_desktop['enabled'] ? `explorer /desktop=animegame,${virtual_desktop['width']}x${virtual_desktop['height']}` : ''}`,
                        `${await Configs.get('fps_unlocker') ? 'unlockfps.bat' : 'launcher.bat'}`,
                        `${borderless_window ? '-screen-fullscreen 0 -popupwindow' : ''}`,
                        `${await Configs.get('wine.fsr') && (!borderless_window || !virtual_desktop['enabled']) ? '-window-mode exclusive' : ''}`
                    ].join(' ');

                    /**
                     * Gamemode integration
                     */
                    if (await Configs.get('gamemode') && await Package.exists('gamemoderun'))
                        command = `gamemoderun ${command}`;

                    /**
                     * Use terminal
                     * 
                     * bash -c "<command> && bash" is required to keep terminal open
                     */
                    if (await Configs.get('use_terminal'))
                    {
                        // Gnome
                        if (await Package.exists('gnome-terminal'))
                            command = `gnome-terminal -- bash -c "${path.addSlashes(command)} && bash"`;

                        // KDE Plasma
                        else if (await Package.exists('konsole'))
                            command = `konsole --hold -e "${path.addSlashes(`bash -c "${path.addSlashes(command)} && bash"`)}"`;

                        // XFCE
                        else if (await Package.exists('xfce4-terminal'))
                            command = `xfce4-terminal --hold -e "${path.addSlashes(`bash -c "${path.addSlashes(command)} && bash"`)}"`;
                    }

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

                    // Stop monitoring of the process output
                    process.outputInterval = null;

                    // Game was started by the launcher.bat file
                    // so we just need to wait until AnimeGame.e process or unlockfps.exe process
                    // will be closed
                    const processName = await Configs.get('fps_unlocker') ? `unlockfps.exe` : `${constants.placeholders.uppercase.first + constants.placeholders.uppercase.second}.e`;

                    let closeGameCounter = 0;

                    const waiter = async () => {
                        const processes: string = (await Neutralino.os.execCommand('ps -A')).stdOut;

                        // Game is still running
                        if (processes.includes(processName))
                        {
                            const playtime = Math.round((Date.now() - startTime) / 1000);

                            let hours: string|number = Math.floor(playtime / 3600);
                            let minutes: string|number = Math.floor((playtime - hours * 3600) / 60);
                            let seconds: string|number = playtime - hours * 3600 - minutes * 60;

                            if (hours < 10)
                                hours = `0${hours}`;

                            if (minutes < 10)
                                minutes = `0${minutes}`;

                            if (seconds < 10)
                                seconds = `0${seconds}`;

                            // FIXME: tray doesn't work in AppImage
                            launcher?.tray.update([
                                { text: `Playing for ${hours}:${minutes}:${seconds}`, disabled: true },
                                {
                                    text: `Close game${closeGameCounter > 0 ? ` (${closeGameCounter})` : ''}`,

                                    click: () => Neutralino.os.execCommand(`kill ${++closeGameCounter < 3 ? '-15' : '-9'} $(pidof ${processName})`)
                                }
                            ]);

                            setTimeout(waiter, 3000);
                        }

                        // Game was closed
                        else
                        {
                            // const stopTime = Date.now();

                            Windows.current.show();
                            Windows.current.center(1280, 700);

                            launcher?.updateDiscordRPC('in-launcher');
                            launcher?.tray.hide();

                            // Purge game logs
                            Configs.get('purge_logs.game').then(async (purge_logs) => {
                                if (purge_logs)
                                {
                                    const gameDir = path.addSlashes(await constants.paths.gameDir);

                                    // Delete .log files (e.g. "ZFGameBrowser_xxxx.log")
                                    Neutralino.os.execCommand(`find "${gameDir}" -maxdepth 1 -type f -name "*.log" -delete`);

                                    // Delete .dmp files (e.g. "DumpFile-zfbrowser-xxxxxx.dmp")
                                    Neutralino.os.execCommand(`find "${gameDir}" -maxdepth 1 -type f -name "*.dmp" -delete`);
                                }
                            });

                            // TODO

                            resolve();
                        }
                    };

                    process.finish(() => waiter());
                }
            })
            .catch(() => {
                Notification.show({
                    ...Locales.translate('notifications.iputils_package_required'),
                    icon: `${constants.paths.appDir}/public/images/baal64-transparent.png`,
                    importance: 'critical'
                });

                resolve();
            });
    });
};
