const fs = require('fs');
const path = require('path');
const { ipcRenderer } = require('electron');
const { exec } = require('child_process');

import $ from 'cash-dom';
import { Genshinlib } from './lib/Genshinlib';
import { LauncherUI } from './lib/LauncherUI';
import { Tools } from './lib/Tools';

$(() => {
    // Make sure settings is shown in correct language.
    LauncherUI.updateLang(Genshinlib.getConfig('lang.launcher') ?? 'en-us');

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

    $('#language').on('selectionChanged', (e, data: any) => {
        let activeLang = Genshinlib.getConfig('lang.launcher');

        if (activeLang != data.value)
        {
            Genshinlib.updateConfig('lang.launcher', data.value);
            Genshinlib.updateConfig('background.time', null);

            LauncherUI.updateLang(data.value);

            // Send language updates
            ipcRenderer.send('change-lang', { 'lang': data.value });
        }
    });

    // Select the saved options in launcher.json on load
    // $(`#voice-list option[value="${Genshinlib.getConfig('lang.voice')}"]`).prop('selected', true);

    $(`#language li[value=${Genshinlib.getConfig('lang.launcher')}]`).addClass('selected');
    $('#language .selected-item span').text($(`#language li[value=${Genshinlib.getConfig('lang.launcher')}]`).text());

    if (Genshinlib.getConfig('rpc'))
        $('#discord-rpc').addClass('checkbox-active');

    $('#discord-rpc').on('classChange', () => {
        Genshinlib.updateConfig('rpc', $('#discord-rpc').hasClass('checkbox-active'));

        ipcRenderer.send('rpc-toggle');
    });

    /*$('#voice-list').on('change', (e) => {
        let activeVP = Genshinlib.getConfig('voice');

        if (activeVP != e.target.value)
        {
            Genshinlib.updateConfig('lang.voice', e.target.value);
            
            ipcRenderer.send('updateVP', { 'oldvp': activeVP });

            $(`#voice-list option[value="${activeVP}"]`).removeProp('selected');
            $(`#voice-list option[value="${e.target.value}"]`).prop('selected', true);
        }

        else console.log('VP can\' be changed to the already set language');
    });*/

    $('#env-list').on('propertyNameChanged', (e, data) => {
        if (data.value != '')
            Genshinlib.updateConfig(`env.${data.name.after}`, data.value);

        if (data.name.before != '')
            Genshinlib.deleteConfig(`env.${data.name.before}`);
    });

    $('#env-list').on('propertyValueChanged', (e, data) => {
        if (data.name != '')
            Genshinlib.updateConfig(`env.${data.name}`, data.value.after);
    });

    $('#env-list').on('propertyDeleted', (e, data) => {
        if (data.name != '')
            Genshinlib.deleteConfig(`env.${data.name}`);
    });

    let env = Genshinlib.getConfig('env');

    Object.keys(env).forEach((property: string) => {
        $('#env-list .button#add')[0]!.click();

        let value = env[property];
        let td = $('#env-list tr').last().children();

        td.first().find('input').val(property);
        td.first().find('span').text(property);

        td.last().find('input').val(value);
        td.last().find('span').text(value);
    });

    let activeRunner = Genshinlib.getConfig('runner');

    Genshinlib.getRunners().then(runners => {
        runners.forEach(category => {
            $(`<h3>${category.title}</h3>`).appendTo('#runners-list');

            category.runners.forEach(runner => {
                let item = $(`<div class="list-item">${runner.name}<div><img src="../images/download.png"></div></div>`).appendTo('#runners-list');
            
                if (fs.existsSync(path.join(Genshinlib.runnersDir, runner.folder)))
                {
                    item.find('div').css('display', 'none');

                    // I think we shouldn't set runner as active if it is not installed
                    if (runner.name == activeRunner?.name)
                        item.addClass('list-item-active');
                }

                item.find('div').on('click', () => {
                    if (!item.hasClass('list-item-disabled'))
                    {
                        item.addClass('list-item-disabled');

                        let div = item.find('div');

                        Tools.downloadFile(runner.uri, path.join(Genshinlib.launcherDir, runner.name), (current: number, total: number, difference: number) => {
                            div.text(`${ Math.round(current / total * 100) }%`);
                        }).then(() => {
                            let unpacker = runner.archive === 'tar' ?
                                Tools.untar : Tools.unzip;

                            unpacker(
                                path.join(Genshinlib.launcherDir, runner.name),
                                runner.makeFolder ?
                                    path.join(Genshinlib.runnersDir, runner.folder) :
                                    Genshinlib.runnersDir,
                                (current: number, total: number, difference: number) => {
                                    div.text(`${ Math.round(current / total * 100) }%`);
                                }
                            ).then(() => {
                                fs.unlinkSync(path.join(Genshinlib.launcherDir, runner.name));

                                item.removeClass('list-item-disabled');
                                div.css('display', 'none');
                            });
                        });
                    }
                });

                item.on('click', () => {
                    if (!item.hasClass('list-item-disabled'))
                    {
                        while (!item.hasClass('list-item'))
                            item = item.parent();

                        if (item.find('div').css('display') === 'none')
                        {
                            Genshinlib.updateConfig('runner.name', runner.name);
                            Genshinlib.updateConfig('runner.folder', runner.folder);
                            Genshinlib.updateConfig('runner.executable', runner.executable);

                            $('#runners-list > .list-item').removeClass('list-item-active');
                            item.addClass('list-item-active');
                        }
                    }
                });
            });
        });
    });

    let activeDXVK = Genshinlib.getConfig('dxvk');

    Genshinlib.getDXVKs().then(dxvks => {
        dxvks.forEach(dxvk => {
            let item = $(`<div class="list-item">${dxvk.version}<div><img src="../images/download.png"></div></div>`).appendTo('#dxvk-list');

            if (fs.existsSync(path.join(Genshinlib.dxvksDir, 'dxvk-' + dxvk.version)))
            {
                item.find('div').css('display', 'none');

                // I think we shouldn't set DXVK as active if it is not installed
                if (dxvk.version == activeDXVK)
                    item.addClass('list-item-active');
            }

            item.find('div').on('click', () => {
                if (!item.hasClass('list-item-disabled'))
                {
                    item.addClass('list-item-disabled');

                    let div = item.find('div');

                    Tools.downloadFile(dxvk.uri, path.join(Genshinlib.launcherDir, 'dxvk-' + dxvk.version), (current: number, total: number, difference: number) => {
                        div.text(`${ Math.round(current / total * 100) }%`);
                    }).then(() => {
                        Tools.untar(
                            path.join(Genshinlib.launcherDir, 'dxvk-' + dxvk.version),
                            Genshinlib.dxvksDir,
                            (current: number, total: number, difference: number) => {
                                div.text(`${ Math.round(current / total * 100) }%`);
                            }
                        ).then(() => {
                            fs.unlinkSync(path.join(Genshinlib.launcherDir, 'dxvk-' + dxvk.version));

                            item.removeClass('list-item-disabled');
                            div.css('display', 'none');
                        });
                    });
                }
            });

            item.on('click', () => {
                if (!item.hasClass('list-item-disabled'))
                {
                    while (!item.hasClass('list-item'))
                        item = item.parent();

                    if (item.find('div').css('display') === 'none')
                    {
                        item.find('div')
                            .css('display', 'flex')
                            .text('Applying...');

                        let installer = exec('./setup_dxvk.sh install', {
                            cwd: path.join(Genshinlib.dxvksDir, 'dxvk-' + dxvk.version),
                            env: {
                                ...process.env,
                                WINEPREFIX: Genshinlib.prefixDir
                            }
                        });

                        installer.on('close', () => {
                            Genshinlib.updateConfig('dxvk', dxvk.version);
    
                            $('#dxvk-list > .list-item').removeClass('list-item-active');
                            item.addClass('list-item-active');
                            item.find('div').css('display', 'none');
                        });
                    }
                }
            });
        });
    });
});
