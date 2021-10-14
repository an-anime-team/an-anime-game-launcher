const path = require('path');
const fs = require('fs');
const { exec } = require('child_process');
const { ipcRenderer } = require('electron');

import $ from 'cash-dom';

import { Genshinlib } from './Genshinlib';

if (!fs.existsSync(Genshinlib.prefixDir))
    fs.mkdirSync(Genshinlib.prefixDir, { recursive: true });

$(() => {
    if (Genshinlib.version !== null)
        document.title = 'Genshin Impact Linux Launcher - ' + Genshinlib.version;

    $('body').css('background-image', `url(${ Genshinlib.getBackgroundUri() })`);

    Genshinlib.getData().then(data => {
        if (Genshinlib.version != data.game.latest.version)
            $('#launch').text(Genshinlib.version === null ? 'Install' : 'Update');

        $('#launch').on('click', async () => {
            // Creating wine prefix
            if (!Genshinlib.isPrefixInstalled(Genshinlib.prefixDir))
            {
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
                exec(`wine "${path.join(Genshinlib.gameDir, 'GenshinImpact.exe')}"`, {
                    env: {
                        ...process.env,
                        WINEPREFIX: Genshinlib.prefixDir
                    }
                }, () => {
                    ipcRenderer.invoke('show-window');
                });

                ipcRenderer.invoke('hide-window');
            }

            // Installing game
            else
            {
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

                                Genshinlib.setLauncherInfo({
                                    ...Genshinlib.getLauncherInfo(),
                                    version: data.game.latest.version
                                });

                                $('#launch').css('display', 'block');
                                $('#downloader-panel').css('display', 'none');
        
                                $('#launch').text('Launch');
                            });
                        }).catch(err => console.log(err));
                    }).catch(err => console.log(err));
                });
            }
        });
    });
});
