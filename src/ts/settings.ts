const fs = require('fs');
const path = require('path');
const { ipcRenderer } = require('electron');
const { exec } = require('child_process');

import $ from 'cash-dom';

import { constants } from './lib/constants';
import { LauncherLib } from './lib/LauncherLib';
import { LauncherUI } from './lib/LauncherUI';
import { Tools } from './lib/Tools';

$(() => {
    // Make sure settings is shown in correct language.
    LauncherUI.updateLang(LauncherLib.getConfig('lang.launcher') ?? 'en-us');

    $('.menu-item').on('click', (e) => {
        $('.settings')[0]!.scrollTop = document.getElementById(e.target.getAttribute('anchor'))!.offsetTop - 16;

        $('.menu-item').removeClass('menu-item-active');
        $(e.target).addClass('menu-item-active');
    });

    $('.settings').on('scroll', () => {
        let anchor = $('.settings-item').filter((index, item) => $(item).offset()!.top < 180).last()[0]!.id; // 264

        $('.menu-item').removeClass('menu-item-active');
        $(`.menu-item[anchor=${anchor}]`).addClass('menu-item-active');
    });

    /**
     * Launcher language
     */

    $(`#language li[value=${LauncherLib.getConfig('lang.launcher')}]`).addClass('selected');
    $('#language .selected-item span').text($(`#language li[value=${LauncherLib.getConfig('lang.launcher')}]`).text());

    $('#language').on('selectionChanged', (e, data: any) => {
        let activeLang = LauncherLib.getConfig('lang.launcher');

        if (activeLang != data.value)
        {
            LauncherLib.updateConfig('lang.launcher', data.value);
            LauncherLib.updateConfig('background.time', null);
            
            LauncherUI.updateLang(data.value);

            // Send language update event
            ipcRenderer.send('change-lang', { 'lang': data.value });
        }
    });

    /**
     * Game voice language
     */

    $(`#voicepack li[value=${LauncherLib.getConfig('lang.voice.active')}]`).addClass('selected');
    $('#voicepack .selected-item span').text($(`#voicepack li[value=${LauncherLib.getConfig('lang.voice.active')}]`).text());

    $('#voicepack').on('selectionChanged', (e, data: any) => {
        let activeLang = LauncherLib.getConfig('lang.voice.active');

        if (activeLang != data.value)
        {
            LauncherLib.updateConfig('lang.voice.active', data.value);

            LauncherUI.setState('game-voice-update-required');

            // Send language update event
            ipcRenderer.send('change-voicepack');
        }
    });

    $('#voicepack .selected-item').attr('data-hint', LauncherUI.i18n.translate('VoiceNotification'));

    /**
     * Discord RPC
     */

    if (LauncherLib.getConfig('rpc'))
        $('#discord-rpc').addClass('checkbox-active');

    $('#discord-rpc').on('classChange', () => {
        LauncherLib.updateConfig('rpc', $('#discord-rpc').hasClass('checkbox-active'));

        ipcRenderer.send('rpc-toggle');
    });

    /**
     * Automatic theme switcher
     */

    if (LauncherLib.getConfig('autotheme'))
        $('#auto-theme').addClass('checkbox-active');

    $('#auto-theme').on('classChange', () => LauncherLib.updateConfig('autotheme', $('#auto-theme').hasClass('checkbox-active')));

    /**
     * Environmental variables manager
     */

    $('#env-list').on('propertyNameChanged', (e, data) => {
        if (data.value != '')
            LauncherLib.updateConfig(`env.${data.name.after}`, data.value);

        if (data.name.before != '')
            LauncherLib.deleteConfig(`env.${data.name.before}`);
    });

    $('#env-list').on('propertyValueChanged', (e, data) => {
        if (data.name != '')
            LauncherLib.updateConfig(`env.${data.name}`, data.value.after);
    });

    $('#env-list').on('propertyDeleted', (e, data) => {
        if (data.name != '')
            LauncherLib.deleteConfig(`env.${data.name}`);
    });

    let env = LauncherLib.getConfig('env');

    Object.keys(env).forEach((property: string) => {
        $('#env-list .button#add')[0]!.click();

        let value = env[property];
        let td = $('#env-list tr').last().children();

        td.first().find('input').val(property);
        td.first().find('span').text(property);

        td.last().find('input').val(value);
        td.last().find('span').text(value);
    });

    /**
     * Statistics
     */

    $('#play-hours').text((LauncherLib.getConfig('playtime') / 3600).toFixed(1).toString());

    // Update this once per two minute
    setInterval(() => {
        $('#play-hours').text((LauncherLib.getConfig('playtime') / 3600).toFixed(1).toString());
    }, 120 * 1000);

    /**
     * Wine versions manager
     */

    let activeRunner = LauncherLib.getConfig('runner');

    LauncherLib.getRunners().then(runners => {
        runners.forEach(category => {
            $(`<h3>${category.title}</h3>`).appendTo('#runners-list');

            category.runners.forEach(runner => {
                let item = $(`<div class="list-item">
                    ${runner.name}
                    <div>
                        <span></span>
                        <img class="item-delete" src="../images/delete.png">
                        <img class="item-download" src="../images/download.png">
                    </div>
                </div>`).appendTo('#runners-list');
            
                if (fs.existsSync(path.join(constants.runnersDir, runner.folder)))
                {
                    // item.find('div').css('display', 'none');
                    item.addClass('list-item-downloaded');

                    // I think we shouldn't set runner as active if it is not installed
                    if (runner.name == activeRunner?.name)
                        item.addClass('list-item-active');
                }

                item.find('img.item-download').on('click', () => {
                    if (!item.hasClass('list-item-disabled'))
                    {
                        item.addClass('list-item-disabled');
                        item.addClass('list-item-downloading');

                        let div = item.find('div'),
                            span = div.find('span');

                        Tools.downloadFile(runner.uri, path.join(constants.launcherDir, runner.name), (current: number, total: number, difference: number) => {
                            span.text(`${ Math.round(current / total * 100) }%`);
                        }).then(() => {
                            let unpacker = runner.archive === 'tar' ?
                                Tools.untar : Tools.unzip;

                            unpacker(
                                path.join(constants.launcherDir, runner.name),
                                runner.makeFolder ?
                                    path.join(constants.runnersDir, runner.folder) :
                                    constants.runnersDir,
                                (current: number, total: number, difference: number) => {
                                    span.text(`${ Math.round(current / total * 100) }%`);
                                }
                            ).then(() => {
                                fs.unlinkSync(path.join(constants.launcherDir, runner.name));

                                span.text('');

                                item.removeClass('list-item-disabled');
                                item.removeClass('list-item-downloading');

                                item.addClass('list-item-downloaded');
                                // div.css('display', 'none');
                            });
                        });
                    }
                });

                item.find('img.item-delete').on('click', () => {
                    if (!item.hasClass('list-item-disabled'))
                    {
                        item.addClass('list-item-disabled');

                        fs.rmdirSync(path.join(constants.runnersDir, runner.folder), { recursive: true });

                        item.removeClass('list-item-disabled');
                        item.removeClass('list-item-downloaded');
                    }
                });

                item.on('click', () => {
                    if (!item.hasClass('list-item-disabled'))
                    {
                        while (!item.hasClass('list-item'))
                            item = item.parent();

                        // if (item.find('div').css('display') === 'none')
                        if (item.hasClass('list-item-downloaded'))
                        {
                            LauncherLib.updateConfig('runner.name', runner.name);
                            LauncherLib.updateConfig('runner.folder', runner.folder);
                            LauncherLib.updateConfig('runner.executable', runner.executable);

                            $('#runners-list > .list-item').removeClass('list-item-active');
                            item.addClass('list-item-active');
                        }
                    }
                });
            });
        });
    });

    /**
     * DXVKs manager
     */
    
    let activeDXVK = LauncherLib.getConfig('dxvk');

    LauncherLib.getDXVKs().then(dxvks => {
        dxvks.forEach(dxvk => {
            let item = $(`<div class="list-item">
                ${dxvk.version}
                <div>
                    <span></span>
                    <img class="item-delete" src="../images/delete.png">
                    <img class="item-download" src="../images/download.png">
                </div>
            </div>`).appendTo('#dxvk-list');

            if (fs.existsSync(path.join(constants.dxvksDir, 'dxvk-' + dxvk.version)))
            {
                // item.find('div').css('display', 'none');
                item.addClass('list-item-downloaded');

                // I think we shouldn't set DXVK as active if it is not installed
                if (dxvk.version == activeDXVK)
                    item.addClass('list-item-active');
            }

            item.find('img.item-download').on('click', () => {
                if (!item.hasClass('list-item-disabled'))
                {
                    item.addClass('list-item-disabled');
                    item.addClass('list-item-downloading');

                    let div = item.find('div'),
                        span = div.find('span');

                    Tools.downloadFile(dxvk.uri, path.join(constants.launcherDir, 'dxvk-' + dxvk.version), (current: number, total: number, difference: number) => {
                        span.text(`${ Math.round(current / total * 100) }%`);
                    }).then(() => {
                        Tools.untar(
                            path.join(constants.launcherDir, 'dxvk-' + dxvk.version),
                            constants.dxvksDir,
                            (current: number, total: number, difference: number) => {
                                span.text(`${ Math.round(current / total * 100) }%`);
                            }
                        ).then(() => {
                            fs.unlinkSync(path.join(constants.launcherDir, 'dxvk-' + dxvk.version));

                            span.text('');

                            item.removeClass('list-item-disabled');
                            item.removeClass('list-item-downloading');

                            item.addClass('list-item-downloaded');
                            // div.css('display', 'none');
                        });
                    });
                }
            });

            item.find('img.item-delete').on('click', () => {
                if (!item.hasClass('list-item-disabled'))
                {
                    item.addClass('list-item-disabled');

                    fs.rmdirSync(path.join(constants.dxvksDir, 'dxvk-' + dxvk.version), { recursive: true });

                    item.removeClass('list-item-disabled');
                    item.removeClass('list-item-downloaded');
                }
            });

            item.on('click', () => {
                if (!item.hasClass('list-item-disabled'))
                {
                    while (!item.hasClass('list-item'))
                        item = item.parent();

                    // if (item.find('div').css('display') === 'none')
                    if (item.hasClass('list-item-downloaded'))
                    {
                        item.addClass('list-item-disabled');
                        item.addClass('list-item-downloading');

                        item.find('div > span').text('Applying...');

                        let installer = exec('./setup_dxvk.sh install', {
                            cwd: path.join(constants.dxvksDir, 'dxvk-' + dxvk.version),
                            env: {
                                ...process.env,
                                WINEPREFIX: constants.prefixDir
                            }
                        });

                        installer.on('close', () => {
                            LauncherLib.updateConfig('dxvk', dxvk.version);

                            item.find('div > span').text('');
    
                            $('#dxvk-list > .list-item').removeClass('list-item-active');

                            item.removeClass('list-item-disabled');
                            item.removeClass('list-item-downloading');

                            item.addClass('list-item-active');
                            item.addClass('list-item-downloaded');

                            // item.find('div').css('display', 'none');
                        });
                    }
                }
            });
        });
    });
});
