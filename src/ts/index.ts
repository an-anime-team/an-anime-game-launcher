const path = require('path');
const fs = require('fs');
const { exec } = require('child_process');
const { ipcRenderer } = require('electron');
import $ from 'cash-dom';

import { Genshinlib } from './lib/Genshinlib';
import { LauncherUI } from './lib/LauncherUI';
import { Tools } from './lib/Tools';
import { DiscordRPC } from './lib/DiscordRPC';

if (!fs.existsSync(Genshinlib.prefixDir))
    fs.mkdirSync(Genshinlib.prefixDir, { recursive: true });

if (!fs.existsSync(Genshinlib.runnersDir))
    fs.mkdirSync(Genshinlib.runnersDir, { recursive: true });

if (!fs.existsSync(Genshinlib.dxvksDir))
    fs.mkdirSync(Genshinlib.dxvksDir, { recursive: true });

$(() => {
    if (Genshinlib.version !== null)
        document.title = 'Genshin Impact Linux Launcher - ' + Genshinlib.version;

    LauncherUI.updateLang(Genshinlib.getConfig('lang.launcher') ?? 'en-us');
    LauncherUI.setState('game-launch-available');
    LauncherUI.updateBackground();
    LauncherUI.updateSocial();

    ipcRenderer.on('change-lang', (event: void, data: any) => {
        LauncherUI.updateLang(data.lang);
        LauncherUI.updateBackground();
        LauncherUI.updateSocial();
    });

    if (Genshinlib.getConfig('rpc'))
        DiscordRPC.init();

    ipcRenderer.on('rpc-toggle', () => {
        DiscordRPC.isActive() ?
            DiscordRPC.init() :
            DiscordRPC.close();
    });

    // FIXME
    ipcRenderer.on('updateVP', (event: void, remotedata: any) => {
        Genshinlib.getData().then(data => {
            LauncherUI.initProgressBar();

            let voicePack = data.game.latest.voice_packs[1]; // en-us
            let old;

            for (let i = 0; i < data.game.latest.voice_packs.length; ++i)
                if (data.game.latest.voice_packs[i].language == Genshinlib.getConfig('lang.voice'))
                {
                    voicePack = data.game.latest.voice_packs[i];

                    break;
                }

            for (let i = 0; i < data.game.latest.voice_packs.length; ++i)
                if (data.game.latest.voice_packs[i].language == remotedata.oldvp)
                {
                    old = data.game.latest.voice_packs[i];

                    break;
                }

            let oldstring = old.name.replace(`_${data.game.latest.version}.zip`, '');

            // Check if the directory and file exists to prevent errors.
            if (fs.existsSync(path.join(Genshinlib.gameDir, oldstring + '_pkg_version')))
                fs.rmSync(path.join(Genshinlib.gameDir, oldstring + '_pkg_version'));
            if (fs.existsSync(path.join(Genshinlib.gameDir, 'GenshinImpact_Data', 'StreamingAssets', 'Audio', 'GeneratedSoundBanks', 'Windows', oldstring.replace('Audio_', ''))))
                fs.rmSync(path.join(Genshinlib.gameDir, 'GenshinImpact_Data', 'StreamingAssets', 'Audio', 'GeneratedSoundBanks', 'Windows', oldstring.replace('Audio_', '')), { recursive: true });

            console.log(`%c> Downloading voice data...`, 'font-size: 16px');

            // For some reason this keeps breaking and locking up most of the time.
            Tools.downloadFile(voicePack.path, path.join(Genshinlib.launcherDir, voicePack.name), (current: number, total: number, difference: number) => {
                LauncherUI.updateProgressBar(LauncherUI.i18n.translate('Downloading'), current, total, difference);
            }).then(() => {
                console.log(`%c> Unpacking voice data...`, 'font-size: 16px');
                            
                LauncherUI.initProgressBar();

                Tools.unzip(path.join(Genshinlib.launcherDir, voicePack.name), Genshinlib.gameDir, (current: number, total: number, difference: number) => {
                    LauncherUI.updateProgressBar(LauncherUI.i18n.translate('Unpack'), current, total, difference);
                }).then(() => {
                    fs.unlinkSync(path.join(Genshinlib.launcherDir, voicePack.name));
                    LauncherUI.setState('game-launch-available');
                })
            });
        });
    });

    Genshinlib.getData().then(data => {
        // Update available
        if (Genshinlib.version != data.game.latest.version)
            LauncherUI.setState(Genshinlib.version === null ? 'game-installation-available' : 'game-update-available');

        // Patch version is incorrect
        else if (Genshinlib.getConfig('patch') && Genshinlib.getConfig('patch.version') != Genshinlib.getPatchInfo().version)
        {
            // Patch is not available
            if (Genshinlib.getPatchInfo().version !== data.game.latest.version)
                LauncherUI.setState('patch-unavailable');

            // Patch available
            else if (Genshinlib.getPatchInfo().version === data.game.latest.version)
            {
                // Patch is stable
                if (Genshinlib.getPatchInfo().state == 'stable')
                {
                    console.log(`%c> Applying patch...`, 'font-size: 16px');

                    LauncherUI.setState('patch-applying');

                    Genshinlib.patchGame(data.game.latest.version, () => {
                        LauncherUI.setState('game-launch-available');
                    }, (data) => console.log(data.toString()));
                }

                // Patch is in testing phase
                else LauncherUI.setState('test-patch-available');
            }
        }

        // Current patch is in testing phase,
        // but stable is available
        else if (Genshinlib.getConfig('patch') && Genshinlib.getConfig('patch.version') == Genshinlib.getPatchInfo().version && Genshinlib.getConfig('patch.state') == 'testing' && Genshinlib.getPatchInfo().state == 'stable')
        {
            console.log(`%c> Applying patch...`, 'font-size: 16px');

            LauncherUI.setState('patch-applying');

            Genshinlib.patchGame(data.game.latest.version, () => {
                LauncherUI.setState('game-launch-available');
            }, (data) => console.log(data.toString()));
        }

        $('#launch').on('click', async () => {
            // Creating wine prefix
            if (!Genshinlib.isPrefixInstalled(Genshinlib.prefixDir))
            {
                console.log(`%c> Creating wineprefix...`, 'font-size: 16px');

                $('#launch').css('display', 'none');
                $('#downloader-panel').css('display', 'block');

                await Genshinlib.installPrefix(Genshinlib.prefixDir, (output: string, current: number, total: number) => {
                    output = output.trim();

                    console.log(output);

                    if (!output.includes('\n') && !output.includes('\r'))
                        $('#downloaded').text(output);

                    $('#downloader .progress').css('width', `${ Math.round(current / total * 100) }%`);
                });

                $('#launch').css('display', 'block');
                $('#downloader-panel').css('display', 'none');
            }

            // Launching game
            if ($('#launch').text() == LauncherUI.i18n.translate('Launch'))
            {
                console.log(`%c> Starting the game...`, 'font-size: 16px');

                if (!await Genshinlib.isTelemetryDisabled())
                {
                    console.log('miHoYo\'s telemetry servers doesn\'t disabled!');

                    ipcRenderer.send('notification', {
                        title: document.title,
                        body: 'miHoYo\'s telemetry servers doesn\'t disabled!',
                        icon: path.join(__dirname, '..', 'images', 'baal64-transparent.png')
                    });
                }

                else
                {
                    let wineExeutable = 'wine';

                    if (Genshinlib.getConfig('runner') !== null)
                    {
                        wineExeutable = path.join(
                            Genshinlib.runnersDir,
                            Genshinlib.getConfig('runner.folder'),
                            Genshinlib.getConfig('runner.executable')
                        );

                        if (!fs.existsSync(wineExeutable))
                        {
                            wineExeutable = 'wine';

                            Genshinlib.updateConfig('runner', null);
                        }
                    }

                    console.log(`Wine executable: ${wineExeutable}`);

                    if (DiscordRPC.isActive())
                    {
                        DiscordRPC.setActivity({
                            details: 'In-Game',
                            largeImageKey: 'game',
                            largeImageText: 'An Anime Game Launcher'
                        });
                    }

                    exec(`${wineExeutable} launcher.bat`, {
                        cwd: Genshinlib.gameDir,
                        env: {
                            ...process.env,
                            WINEPREFIX: Genshinlib.prefixDir
                        }
                    }, (err: any, stdout: any, stderr: any) => {
                        console.log(`%c> Game closed`, 'font-size: 16px');

                        ipcRenderer.invoke('show-window');

                        if (DiscordRPC.isActive())
                        {
                            DiscordRPC.setActivity({
                                details: 'Preparing to launch',
                                largeImageKey: 'launcher',
                                largeImageText: 'An Anime Game Launcher'
                            });
                        }

                        console.log(err);
                        console.log(stdout);
                        console.log(stderr);
                    });

                    ipcRenderer.invoke('hide-window');
                }
            }

            // Apply test patch
            else if ($('#launch').text() == LauncherUI.i18n.translate('TestPatch'))
            {
                console.log(`%c> Applying patch...`, 'font-size: 16px');

                LauncherUI.setState('patch-applying');

                Genshinlib.patchGame(data.game.latest.version, () => {
                    LauncherUI.setState('game-launch-available');
                }, (data) => console.log(data.toString()));
            }

            // Installing game
            else
            {
                console.log(`%c> Downloading game data...`, 'font-size: 16px');

                let diff = {
                    path: data.game.latest.path,
                    name: `latest-${data.game.latest.version}.zip`,
                    voice_packs: data.game.latest.voice_packs
                };
                
                for (let i = 0; i < data.game.diffs.length; ++i)
                    if (data.game.diffs[i].version == Genshinlib.version)
                    {
                        diff = data.game.diffs[i];

                        break;
                    }

                if (fs.existsSync(path.join(Genshinlib.gameDir, diff.name)))
                    fs.unlinkSync(path.join(Genshinlib.gameDir, diff.name));

                /**
                 * Downloading game
                 */

                LauncherUI.initProgressBar();

                Tools.downloadFile(diff.path, path.join(Genshinlib.launcherDir, diff.name), (current: number, total: number, difference: number) => {
                    LauncherUI.updateProgressBar(LauncherUI.i18n.translate('Downloading'), current, total, difference);
                }).then(() => {
                    /**
                     * Unpacking downloaded game
                     */

                    console.log(`%c> Unpacking game data...`, 'font-size: 16px');

                    if (!fs.existsSync(Genshinlib.gameDir))
                        fs.mkdirSync(Genshinlib.gameDir, { recursive: true });

                    LauncherUI.initProgressBar();

                    Tools.unzip(path.join(Genshinlib.launcherDir, diff.name), Genshinlib.gameDir, (current: number, total: number, difference: number) => {
                        LauncherUI.updateProgressBar(LauncherUI.i18n.translate('Unpack'), current, total, difference);
                    }).then(() => {
                        /**
                         * Downloading voice data
                         */

                        console.log(`%c> Downloading voice data...`, 'font-size: 16px');

                        fs.unlinkSync(path.join(Genshinlib.launcherDir, diff.name));

                        let voicePack = diff.voice_packs[1]; // en-us

                        for (let i = 0; i < diff.voice_packs.length; ++i)
                            if (diff.voice_packs[i].language == Genshinlib.getConfig('voice'))
                            {
                                voicePack = diff.voice_packs[i];

                                break;
                            }

                        LauncherUI.initProgressBar();

                        Tools.downloadFile(voicePack.path, path.join(Genshinlib.launcherDir, voicePack.name), (current: number, total: number, difference: number) => {
                            LauncherUI.updateProgressBar(LauncherUI.i18n.translate('Downloading'), current, total, difference);
                        }).then(() => {
                            /**
                             * Unpacking downloaded game
                             */

                            console.log(`%c> Unpacking voice data...`, 'font-size: 16px');
                            
                            LauncherUI.initProgressBar();

                            Tools.unzip(path.join(Genshinlib.launcherDir, voicePack.name), Genshinlib.gameDir, (current: number, total: number, difference: number) => {
                                LauncherUI.updateProgressBar(LauncherUI.i18n.translate('Unpack'), current, total, difference);
                            }).then(() => {
                                fs.unlinkSync(path.join(Genshinlib.launcherDir, voicePack.name));

                                Genshinlib.updateConfig('version', data.game.latest.version);

                                // Patch available
                                if (Genshinlib.getPatchInfo().version === data.game.latest.version)
                                {
                                    // TODO: check the patch state

                                    console.log(`%c> Applying patch...`, 'font-size: 16px');

                                    // patch-applying state changes only button text
                                    $('#downloaded').text(LauncherUI.i18n.translate('ApplyPatch'));

                                    Genshinlib.patchGame(data.game.latest.version, () => {
                                        LauncherUI.setState('game-launch-available');

                                        ipcRenderer.send('notification', {
                                            title: document.title,
                                            body: LauncherUI.i18n.translate('GameDownloaded')
                                        });
                                    }, (data) => console.log(data.toString()));
                                }

                                // Patch is not available
                                else LauncherUI.setState('patch-unavailable');
                            });
                        }).catch(err => console.log(err));
                    }).catch(err => console.log(err));
                });
            }
        });

        $('#settings').on('click', () => ipcRenderer.invoke('open-settings'));
    });
});
