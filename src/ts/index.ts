const path = require('path');
const fs = require('fs');
const { exec } = require('child_process');
const { ipcRenderer } = require('electron');

import $ from 'cash-dom';

import { Genshinlib } from './Genshinlib';
import { LauncherUI } from './LauncherUI';

if (!fs.existsSync(Genshinlib.prefixDir))
    fs.mkdirSync(Genshinlib.prefixDir, { recursive: true });

$(() => {
    if (Genshinlib.version !== null)
        document.title = 'Genshin Impact Linux Launcher - ' + Genshinlib.version;

    Genshinlib.getBackgroundUri().then(uri => $('body').css('background-image', `url(${ uri })`));

    // Don't really think we need this, but maybe in future~~

    /*fetch(`https://genshin.mihoyo.com/launcher/10/${ Genshinlib.getConfig().lang.launcher }?api_url=https%3A%2F%2Fapi-os-takumi.mihoyo.com%2Fhk4e_global&prev=false`)
        .then(res => res.text())
        .then(body => {
            $(body).appendTo('#launchcontent');

            // $('.home__main').detach('.home-swiper-wrap')

            // Get the home container to remove the functions that don't work currently.
            // let home = container!.getElementsByClassName('home__main')[0];
            // home.removeChild(home.getElementsByClassName('home-swiper-wrap')[0]);
            // home.removeChild(home.getElementsByClassName('home-news')[0]);
        });*/

    Genshinlib.getData().then(data => {
        // Update available
        if (Genshinlib.version != data.game.latest.version)
            LauncherUI.setState(Genshinlib.version === null ? 'game-installation-available' : 'game-update-available');

        // Patch version is incorrect
        else if (Genshinlib.getConfig().patch.version != Genshinlib.getPatchInfo().version)
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

                    Genshinlib.applyPatch(() => {
                        LauncherUI.setState('game-launch-available');
                    }, (data) => console.log(data.toString()));
                }

                // Patch is in testing phase
                else LauncherUI.setState('test-patch-available');
            }
        }

        // Current patch is in testing phase,
        // but stable is available
        else if (Genshinlib.getConfig().patch.version == Genshinlib.getPatchInfo().version && Genshinlib.getConfig().patch.state == 'testing' && Genshinlib.getPatchInfo().state == 'stable')
        {
            console.log(`%c> Applying patch...`, 'font-size: 16px');

            LauncherUI.setState('patch-applying');

            Genshinlib.applyPatch(() => {
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
            if ($('#launch').text() == 'Launch')
            {
                console.log(`%c> Starting the game...`, 'font-size: 16px');

                exec('wine launcher.bat', {
                    cwd: Genshinlib.gameDir,
                    env: {
                        ...process.env,
                        WINEPREFIX: Genshinlib.prefixDir
                    }
                }, (err: any, stdout: any, stderr: any) => {
                    console.log(`%c> Game closed`, 'font-size: 16px');

                    ipcRenderer.invoke('show-window');

                    console.log(err);
                    console.log(stdout);
                    console.log(stderr);
                });

                ipcRenderer.invoke('hide-window');
            }

            // Apply test patch
            else if ($('#launch').text() == 'Apply test patch')
            {
                console.log(`%c> Applying patch...`, 'font-size: 16px');

                LauncherUI.setState('patch-applying');

                Genshinlib.applyPatch(() => {
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

                Genshinlib.downloadFile(diff.path, path.join(Genshinlib.launcherDir, diff.name), (current: number, total: number, difference: number) => {
                    LauncherUI.updateProgressBar('Downloaded', current, total, difference);
                }).then(() => {
                    /**
                     * Unpacking downloaded game
                     */

                    console.log(`%c> Unpacking game data...`, 'font-size: 16px');

                    if (!fs.existsSync(Genshinlib.gameDir))
                        fs.mkdirSync(Genshinlib.gameDir, { recursive: true });

                    LauncherUI.initProgressBar();

                    Genshinlib.unzip(path.join(Genshinlib.launcherDir, diff.name), Genshinlib.gameDir, (current: number, total: number, difference: number) => {
                        LauncherUI.updateProgressBar('Unpacking', current, total, difference);
                    }).then(() => {
                        /**
                         * Downloading voice data
                         */

                        console.log(`%c> Downloading voice data...`, 'font-size: 16px');

                        fs.unlinkSync(path.join(Genshinlib.launcherDir, diff.name));

                        let voicePack = diff.voice_packs[1]; // en-us

                        for (let i = 0; i < diff.voice_packs.length; ++i)
                            if (diff.voice_packs[i].language == Genshinlib.lang.voice)
                            {
                                voicePack = diff.voice_packs[i];

                                break;
                            }

                        LauncherUI.initProgressBar();

                        Genshinlib.downloadFile(voicePack.path, path.join(Genshinlib.launcherDir, voicePack.name), (current: number, total: number, difference: number) => {
                            LauncherUI.updateProgressBar('Downloaded', current, total, difference);
                        }).then(() => {
                            /**
                             * Unpacking downloaded game
                             */

                            console.log(`%c> Unpacking voice data...`, 'font-size: 16px');
                            
                            LauncherUI.initProgressBar();

                            Genshinlib.unzip(path.join(Genshinlib.launcherDir, voicePack.name), Genshinlib.gameDir, (current: number, total: number, difference: number) => {
                                LauncherUI.updateProgressBar('Unpacking', current, total, difference);
                            }).then(() => {
                                fs.unlinkSync(path.join(Genshinlib.launcherDir, voicePack.name));

                                Genshinlib.setConfig({
                                    ...Genshinlib.getConfig(),
                                    version: data.game.latest.version
                                });

                                // Patch available
                                if (Genshinlib.getPatchInfo().version === data.game.latest.version)
                                {
                                    // TODO: check the patch state

                                    console.log(`%c> Applying patch...`, 'font-size: 16px');

                                    // patch-applying state changes only button text
                                    $('#downloaded').text('Applying patch...');

                                    Genshinlib.applyPatch(() => {
                                        LauncherUI.setState('game-launch-available');

                                        ipcRenderer.send('notification', {
                                            title: document.title,
                                            content: 'Game was succesfully installed'
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
    });
});
