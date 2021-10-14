const path = require('path');
const fs = require('fs');
const { exec, spawn } = require('child_process');
const { ipcRenderer } = require('electron');

import $ from 'cash-dom';

import { Genshinlib } from './Genshinlib';

if (!fs.existsSync(Genshinlib.prefixDir))
    fs.mkdirSync(Genshinlib.prefixDir, { recursive: true });

$(() => {
    if (Genshinlib.version !== null)
        document.title = 'Genshin Impact Linux Launcher - ' + Genshinlib.version;

    $('body').css('background-image', `url(${ Genshinlib.getBackgroundUri() })`);

    // TODO: create LauncherUI class and move a lot of code there
    //       because it becomes a fucking unfunny joke

    Genshinlib.getData().then(data => {
        // Update available
        if (Genshinlib.version != data.game.latest.version)
            $('#launch').text(Genshinlib.version === null ? 'Install' : 'Update');

        // Patch version is incorrect
        else if (Genshinlib.getConfig().patch.version != Genshinlib.getPatchInfo().version)
        {
            // Patch is not available
            if (Genshinlib.getPatchInfo().version !== data.game.latest.version)
            {
                $('#launch').attr('disabled', 'disabled');
                $('#launch').text('Patch required');

                $('#launch').addClass('hint--top');
                $('#launch').addClass('hint--medium');

                $('#launch').attr('data-hint', 'This game version doesn\'t have the anti-cheat patch. Please, wait a few days before it will be created');
            }

            // Patch available
            else if (Genshinlib.getPatchInfo().version === data.game.latest.version)
            {
                // Patch is stable
                if (Genshinlib.getPatchInfo().state == 'stable')
                {
                    console.log(`%c> Applying patch...`, 'font-size: 16px');

                    $('#launch').attr('disabled', 'disabled');
                    $('#launch').text('Applying patch...');

                    let patcherProcess = spawn('bash', [Genshinlib.patchSh], {
                        cwd: Genshinlib.gameDir,
                        env: {
                            ...process.env,
                            WINEPREFIX: Genshinlib.prefixDir
                        }
                    });

                    patcherProcess.stdout.on('data', (data: string) => console.log(data.toString()));
            
                    patcherProcess.on('close', () => {
                        Genshinlib.setConfig({
                            ...Genshinlib.getConfig(),
                            patch: Genshinlib.getPatchInfo()
                        });

                        $('#launch').removeAttr('disabled');
                        $('#launch').text('Launch');
                    });
                }

                // Patch is in testing phase
                else
                {
                    $('#launch').text('Apply test patch');

                    $('#launch').addClass('button-blue');
                    $('#launch').addClass('hint--top');
                    $('#launch').addClass('hint--large');

                    $('#launch').attr('data-hint', 'This game version has the anti-cheat patch, but it is in the test phase. You can wait a few days until it will become stable or apply it on your own risc');
                }
            }
        }

        // Current patch is in testing phase,
        // but stable is available
        else if (Genshinlib.getConfig().patch.version == Genshinlib.getPatchInfo().version && Genshinlib.getConfig().patch.state == 'testing' && Genshinlib.getPatchInfo().state == 'stable')
        {
            console.log(`%c> Applying patch...`, 'font-size: 16px');

            $('#launch').attr('disabled', 'disabled');
            $('#launch').text('Applying patch...');

            let patcherProcess = spawn('bash', [Genshinlib.patchSh], {
                cwd: Genshinlib.gameDir,
                env: {
                    ...process.env,
                    WINEPREFIX: Genshinlib.prefixDir
                }
            });

            patcherProcess.stdout.on('data', (data: string) => console.log(data.toString()));
    
            patcherProcess.on('close', () => {
                Genshinlib.setConfig({
                    ...Genshinlib.getConfig(),
                    patch: Genshinlib.getPatchInfo()
                });

                $('#launch').removeAttr('disabled');
                $('#launch').text('Launch');
            });
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

                $('#launch').attr('disabled', 'disabled');
                $('#launch').text('Applying patch...');

                let patcherProcess = spawn('bash', [Genshinlib.patchSh], {
                    cwd: Genshinlib.gameDir,
                    env: {
                        ...process.env,
                        WINEPREFIX: Genshinlib.prefixDir
                    }
                });

                patcherProcess.stdout.on('data', (data: string) => console.log(data.toString()));
        
                patcherProcess.on('close', () => {
                    Genshinlib.setConfig({
                        ...Genshinlib.getConfig(),
                        patch: Genshinlib.getPatchInfo()
                    });

                    $('#launch').removeClass('button-blue');
                    $('#launch').removeClass('hint--top');
                    $('#launch').removeClass('hint--large');

                    $('#launch').removeAttr('disabled');
                    $('#launch').removeAttr('data-hint');

                    $('#launch').text('Launch');
                });
            }

            // Installing game
            else
            {
                console.log(`%c> Downloading game data...`, 'font-size: 16px');

                $('#launch').css('display', 'none');
                $('#downloader-panel').css('display', 'block');

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

                let beganAt = Date.now(), prevTime = Date.now(), downloaded = 0;

                /**
                 * Downloading game
                 */

                Genshinlib.downloadFile(diff.path, path.join(Genshinlib.launcherDir, diff.name), (current: number, total: number, difference: number) => {
                    $('#downloaded').text(`Downloaded: ${ Math.round(current / total * 100) }% (${ (current / 1024 / 1024 / 1024).toFixed(2) } GB / ${ Math.round(total / 1024 / 1024 / 1024).toFixed(2) } GB)`);

                    downloaded += difference;

                    if (Date.now() - prevTime > 1000)
                    {
                        let eta = Math.round(total / current * (Date.now() - beganAt) / 1000); // seconds

                        let etaHours   = Math.floor(eta / 3600),
                            etaMinutes = Math.floor((eta - etaHours * 3600) / 60),
                            etaSeconds = eta - etaHours * 3600 - etaMinutes * 60;

                        if (etaHours < 10) // @ts-expect-error
                            etaHours = '0' + etaHours.toString();

                        if (etaMinutes < 10) // @ts-expect-error
                            etaMinutes = '0' + etaMinutes.toString();

                        if (etaSeconds < 10) // @ts-expect-error
                            etaSeconds = '0' + etaSeconds.toString();

                        $('#downloader .progress').css('width', `${ Math.round(current / total * 100) }%`);
                        $('#speed').text(`${ (downloaded / (Date.now() - prevTime) * 1000 / 1024 / 1024).toFixed(2) } MB/s`);
                        $('#eta').text(`ETA: ${etaHours}:${etaMinutes}:${etaSeconds}`);

                        prevTime = Date.now();
                        downloaded = 0;
                    }
                }).then(() => {
                    /**
                     * Unpacking downloaded game
                     */

                    console.log(`%c> Unpacking game data...`, 'font-size: 16px');

                    $('#speed').text('');
                    $('#eta').text('');

                    if (!fs.existsSync(Genshinlib.gameDir))
                        fs.mkdirSync(Genshinlib.gameDir, { recursive: true });
                    
                    let beganAt = Date.now(), prevTime = Date.now(), unpacked = 0;

                    Genshinlib.unzip(path.join(Genshinlib.launcherDir, diff.name), Genshinlib.gameDir, (current: number, total: number, difference: number) => {
                        $('#downloaded').text(`Unpacking: ${ Math.round(current / total * 100) }% (${ (current / 1024 / 1024 / 1024).toFixed(2) } GB / ${ Math.round(total / 1024 / 1024 / 1024).toFixed(2) } GB)`);
                        
                        unpacked += difference;

                        if (Date.now() - prevTime > 1000)
                        {
                            let eta = Math.round(total / current * (Date.now() - beganAt) / 1000); // seconds

                            let etaHours   = Math.floor(eta / 3600),
                                etaMinutes = Math.floor((eta - etaHours * 3600) / 60),
                                etaSeconds = eta - etaHours * 3600 - etaMinutes * 60;

                            if (etaHours < 10) // @ts-expect-error
                                etaHours = '0' + etaHours.toString();

                            if (etaMinutes < 10) // @ts-expect-error
                                etaMinutes = '0' + etaMinutes.toString();

                            if (etaSeconds < 10) // @ts-expect-error
                                etaSeconds = '0' + etaSeconds.toString();

                            $('#downloader .progress').css('width', `${ Math.round(current / total * 100) }%`);
                            $('#speed').text(`${ (unpacked / (Date.now() - prevTime) * 1000 / 1024 / 1024).toFixed(2) } MB/s`);
                            $('#eta').text(`ETA: ${etaHours}:${etaMinutes}:${etaSeconds}`);

                            prevTime = Date.now();
                            unpacked = 0;
                        }
                    }).then(() => {
                        console.log(`%c> Downloading voice data...`, 'font-size: 16px');

                        fs.unlinkSync(path.join(Genshinlib.launcherDir, diff.name));

                        let voicePack = diff.voice_packs[1]; // en-us

                        for (let i = 0; i < diff.voice_packs.length; ++i)
                            if (diff.voice_packs[i].language == Genshinlib.lang.voice)
                            {
                                voicePack = diff.voice_packs[i];

                                break;
                            }

                        let beganAt = Date.now(), prevTime = Date.now(), downloaded = 0;

                        /**
                         * Downloading voice data
                         */

                        Genshinlib.downloadFile(voicePack.path, path.join(Genshinlib.launcherDir, voicePack.name), (current: number, total: number, difference: number) => {
                            $('#downloaded').text(`Downloaded: ${ Math.round(current / total * 100) }% (${ (current / 1024 / 1024 / 1024).toFixed(2) } GB / ${ Math.round(total / 1024 / 1024 / 1024).toFixed(2) } GB)`);

                            downloaded += difference;

                            if (Date.now() - prevTime > 1000)
                            {
                                let eta = Math.round(total / current * (Date.now() - beganAt) / 1000); // seconds

                                let etaHours   = Math.floor(eta / 3600),
                                    etaMinutes = Math.floor((eta - etaHours * 3600) / 60),
                                    etaSeconds = eta - etaHours * 3600 - etaMinutes * 60;

                                if (etaHours < 10) // @ts-expect-error
                                    etaHours = '0' + etaHours.toString();

                                if (etaMinutes < 10) // @ts-expect-error
                                    etaMinutes = '0' + etaMinutes.toString();

                                if (etaSeconds < 10) // @ts-expect-error
                                    etaSeconds = '0' + etaSeconds.toString();

                                $('#downloader .progress').css('width', `${ Math.round(current / total * 100) }%`);
                                $('#speed').text(`${ (downloaded / (Date.now() - prevTime) * 1000 / 1024 / 1024).toFixed(2) } MB/s`);
                                $('#eta').text(`ETA: ${etaHours}:${etaMinutes}:${etaSeconds}`);

                                prevTime = Date.now();
                                downloaded = 0;
                            }
                        }).then(() => {
                            /**
                             * Unpacking downloaded game
                             */

                             console.log(`%c> Unpacking voice data...`, 'font-size: 16px');

                            $('#speed').text('');
                            $('#eta').text('');
                            
                            let beganAt = Date.now(), prevTime = Date.now(), unpacked = 0;

                            Genshinlib.unzip(path.join(Genshinlib.launcherDir, voicePack.name), Genshinlib.gameDir, (current: number, total: number, difference: number) => {
                                $('#downloaded').text(`Unpacking: ${ Math.round(current / total * 100) }% (${ (current / 1024 / 1024 / 1024).toFixed(2) } GB / ${ Math.round(total / 1024 / 1024 / 1024).toFixed(2) } GB)`);
                                
                                unpacked += difference;

                                if (Date.now() - prevTime > 1000)
                                {
                                    let eta = Math.round(total / current * (Date.now() - beganAt) / 1000); // seconds

                                    let etaHours   = Math.floor(eta / 3600),
                                        etaMinutes = Math.floor((eta - etaHours * 3600) / 60),
                                        etaSeconds = eta - etaHours * 3600 - etaMinutes * 60;

                                    if (etaHours < 10) // @ts-expect-error
                                        etaHours = '0' + etaHours.toString();

                                    if (etaMinutes < 10) // @ts-expect-error
                                        etaMinutes = '0' + etaMinutes.toString();

                                    if (etaSeconds < 10) // @ts-expect-error
                                        etaSeconds = '0' + etaSeconds.toString();

                                    $('#downloader .progress').css('width', `${ Math.round(current / total * 100) }%`);
                                    $('#speed').text(`${ (unpacked / (Date.now() - prevTime) * 1000 / 1024 / 1024).toFixed(2) } MB/s`);
                                    $('#eta').text(`ETA: ${etaHours}:${etaMinutes}:${etaSeconds}`);

                                    prevTime = Date.now();
                                    unpacked = 0;
                                }
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

                                    $('#downloaded').text('Applying patch...');

                                    let patcherProcess = spawn('bash', [Genshinlib.patchSh], {
                                        cwd: Genshinlib.gameDir,
                                        env: {
                                            ...process.env,
                                            WINEPREFIX: Genshinlib.prefixDir
                                        }
                                    });

                                    patcherProcess.stdout.on('data', (data: string) => console.log(data.toString()));
                            
                                    patcherProcess.on('close', () => {
                                        Genshinlib.setConfig({
                                            ...Genshinlib.getConfig(),
                                            patch: Genshinlib.getPatchInfo()
                                        });

                                        $('#launch').css('display', 'block');
                                        $('#downloader-panel').css('display', 'none');
                
                                        $('#launch').text('Launch');
                                    });
                                }

                                // Patch is not available
                                else
                                {
                                    $('#launch').css('display', 'block');
                                    $('#downloader-panel').css('display', 'none');

                                    $('#launch').attr('disabled', 'disabled');
                                    $('#launch').text('Patch required');

                                    $('#launch').addClass('hint--top');
                                    $('#launch').addClass('hint--medium');

                                    $('#launch').attr('data-hint', 'This game version doesn\'t have the anti-cheat patch. Please, wait a few days before it will be created');
                                }
                            });
                        }).catch(err => console.log(err));
                    }).catch(err => console.log(err));
                });
            }
        });
    });
});
