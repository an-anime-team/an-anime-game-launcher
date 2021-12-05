const fs = require('fs');
const path = require('path');
const { ipcRenderer } = require('electron');
const { exec, spawn } = require('child_process');

const commandExists = require('command-exists').sync;

import $ from 'cash-dom';

import constants from './lib/constants';
import LauncherLib from './lib/LauncherLib';
import LauncherUI from './lib/LauncherUI';
import Tools from './lib/Tools';

import SwitcherooControl from './lib/SwitcherooControl';

$(() => {
    // Make sure settings is shown in correct language.
    LauncherUI.updateLang(LauncherLib.getConfig('lang.launcher') ?? 'en-us');

    $('body').attr('theme', LauncherUI.theme);

    $('.menu-item').on('click', (e) => {
        $('.settings')[0]!.scrollTop = document.getElementById(e.target.getAttribute('anchor'))!.offsetTop - 16;

        $('.menu-item').removeClass('menu-item-active');
        $(e.target).addClass('menu-item-active');
    });

    $('.settings').on('scroll', () => {
        const anchor = $('.settings-item').filter((index, item) => $(item).offset()!.top < 180).last()[0]!.id; // 264

        $('.menu-item').removeClass('menu-item-active');
        $(`.menu-item[anchor=${anchor}]`).addClass('menu-item-active');
    });

    /**
     * Statistics
     */

    const playedHours = Math.floor(LauncherLib.getConfig('playtime') / 3600);
    const playedMinutes = Math.floor((LauncherLib.getConfig('playtime') - playedHours * 3600) / 60);

    $('#play-hours').text(playedHours.toString());
    $('#play-minutes').text(playedMinutes.toString());

    const levelHours = (level: number) => 0.000441332 * Math.pow(level + 10, 3.10628);

    let level = 1;

    while (level < 91 && levelHours(level) < playedHours)
        ++level;

    $('.launcher-stats .level').text(level.toString());
    $('.launcher-stats .level').attr('data-hint', LauncherUI.i18n.translate('YourLauncherLevel'));

    /**
     * Launcher language
     */

    $(`#language li[value=${LauncherLib.getConfig('lang.launcher')}]`).addClass('selected');
    $('#language .selected-item span').text($(`#language li[value=${LauncherLib.getConfig('lang.launcher')}]`).text());

    $('#language').on('selectionChanged', (e, data: any) => {
        if (LauncherLib.getConfig('lang.launcher') != data.value)
        {
            LauncherLib.updateConfig('lang.launcher', data.value);
            LauncherLib.updateConfig('background.time', null);

            LauncherUI.updateLang(data.value);

            // Send language update event
            ipcRenderer.send('change-lang', { 'lang': data.value });
        }
    });

    /**
     * Prefix
     */

    $('#prefix input').val(constants.prefixDir.get());

    ipcRenderer.on('prefix-changed', () => {
        $('#prefix input').val(constants.prefixDir.get());
    });

    $('#prefix #choose-location').on('click', () => ipcRenderer.send('prefix-select'));
    $('#prefix #reset-location').on('click', () => ipcRenderer.send('prefix-reset'));

    $('#prefix #choose-location').attr('data-hint', LauncherUI.i18n.translate('SelectDir'));
    $('#prefix #reset-location').attr('data-hint', LauncherUI.i18n.translate('ResetDir'));

    /**
     * Game voice language
     */

    $(`#voicepack li[value=${LauncherLib.getConfig('lang.voice.active')}]`).addClass('selected');
    $('#voicepack .selected-item span').text($(`#voicepack li[value=${LauncherLib.getConfig('lang.voice.active')}]`).text());

    $('#voicepack').on('selectionChanged', (e, data: any) => {
        if (LauncherLib.getConfig('lang.voice.active') != data.value)
        {
            LauncherLib.updateConfig('lang.voice.active', data.value);

            LauncherUI.setState('game-voice-update-required');

            // Send language update event
            ipcRenderer.send('change-voicepack');
        }
    });

    $('#voicepack .selected-item').attr('data-hint', LauncherUI.i18n.translate('VoiceNotification'));

    /**
     * Theme
     */

    $(`#theme li[value=${LauncherLib.getConfig('theme')}]`).addClass('selected');
    $('#theme .selected-item span').text($(`#theme li[value=${LauncherLib.getConfig('theme')}]`).text());

    $('#theme').on('selectionChanged', (e, data: any) => {
        if (LauncherLib.getConfig('theme') != data.value)
        {
            LauncherLib.updateConfig('theme', data.value);
            
            // Not `data.value` because we don't have "system" theme
            $('body').attr('theme', LauncherUI.theme);
        }
    });

    /**
     * Action buttons
     */

    $('#general-action-buttons #launcher-folder').on('click', () => {
        spawn('xdg-open', [constants.launcherDir]);
    });

    /**
     * winetricks button
     */
    if (commandExists('winetricks'))
    {
        $('#general-action-buttons #winetricks').on('click', () => {
            exec('winetricks', {
                env: {
                    ...process.env,
                    WINEPREFIX: constants.prefixDir.get()
                }
            });
        });
    }

    else
    {
        $('#general-action-buttons #winetricks')
            .addClass('hint--top hint--small')
            .attr('data-hint', LauncherUI.i18n.translate('IsNotInstalled', ['winetricks']))
            .attr('disabled', 'disabled');
    }

    /**
     * winecfg button
     */
    if (commandExists('winecfg'))
    {
        $('#general-action-buttons #winecfg').on('click', () => {
            exec('winecfg', {
                env: {
                    ...process.env,
                    WINEPREFIX: constants.prefixDir.get()
                }
            });
        });
    }

    else
    {
        $('#general-action-buttons #winecfg')
            .addClass('hint--top hint--small')
            .attr('data-hint', LauncherUI.i18n.translate('IsNotInstalled', ['winecfg']))
            .attr('disabled', 'disabled');
    }

    /**
     * HUD
     */

    $(`#hud li[value=${LauncherLib.getConfig('hud')}]`).addClass('selected');
    $('#hud .selected-item span').text($(`#hud li[value=${LauncherLib.getConfig('hud')}]`).text());

    $('#hud').on('selectionChanged', (e, data: any) => {
        LauncherLib.updateConfig('hud', data.value);
    });

    if (!commandExists('mangohud'))
    {
        $('#hud li[value=mangohud]')
            .attr('disabled', '')
            .addClass('hint--top hint--small')
            .attr('data-hint', LauncherUI.i18n.translate('PreInstallationRequired'));
    }

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
     * Auto-delete DXVK logs
     */

    if (LauncherLib.getConfig('autodelete_dxvk_logs'))
        $('#autodelete-dxvk-logs').addClass('checkbox-active');

    $('#autodelete-dxvk-logs').on('classChange', () => {
        LauncherLib.updateConfig('autodelete_dxvk_logs', $('#autodelete-dxvk-logs').hasClass('checkbox-active'));
    });

    /**
     * GameMode
     */

    let gameModeAvailable = fs.existsSync('/usr/share/gamemoderun');

    process.env.PATH?.split(':').forEach(path => gameModeAvailable ||= fs.existsSync(`${path}/gamemoderun`));

    if (!gameModeAvailable)
    {
        $('#gamemode')
            .addClass('checkbox-disabled')
            .addClass('hint--top hint--medium')
            .attr('data-hint', LauncherUI.i18n.translate('GameModeNotInstalled'));
    }

    if (LauncherLib.getConfig('gamemode'))
        $('#gamemode').addClass('checkbox-active');

    $('#gamemode').on('classChange', () => {
        LauncherLib.updateConfig('gamemode', $('#gamemode').hasClass('checkbox-active'));
    });

    /**
     * GPU selection
     */

    SwitcherooControl.waitReady().then(async () => {
        const gpus = await SwitcherooControl.getGpus();

        if (gpus)
        {
            console.log(gpus);

            for (const gpu of gpus.value)
                $(`<li value="${gpu.Name.value}">${gpu.Name.value}</li>`).appendTo('#gpu .select-options ul');

            SwitcherooControl.getGpuByName(LauncherLib.getConfig('gpu')).then((gpu) => {
                if (gpu)
                {
                    $(`#gpu li[value=${gpu.Name.value}]`).addClass('selected');
                    $('#gpu .selected-item span').text(gpu.Name.value);
                }
            });
        }
    }, () => {
        console.log('switcheroo-control not running');

        $('#gpu .selected-item')
          .addClass('hint--top hint--medium')
          .attr('data-hint', LauncherUI.i18n.translate('SwitcherooNotInstalled'));
    });

    $('#gpu').on('selectionChanged', (e, data: any) => {
        LauncherLib.updateConfig('gpu', data.value);
    });

    /**
     * Shaders
     */

    let reshadeAvailable = fs.existsSync('/usr/share/reshade');

    if (!reshadeAvailable)
        process.env.PATH?.split(':').forEach(path => reshadeAvailable ||= fs.existsSync(`${path}/reshade`));

    if (!reshadeAvailable)
        $(`<p>⚠️ ${LauncherUI.i18n.translate('ReshadeNotInstalled')}</p>`).appendTo('#shaders');

    fs.readdirSync(constants.shadersDir).forEach((folder: string) => {
        const shaders: any = JSON.parse(fs.readFileSync(path.join(constants.shadersDir, folder, 'shaders.json')));

        // Selectable item
        let li = $(`<li value="${folder}">${shaders.name}</li>`).appendTo('#shaders-list ul');

        if (!reshadeAvailable)
            li.attr('disabled', '')
              .addClass('hint--top hint--small')
              .attr('data-hint', LauncherUI.i18n.translate('PreInstallationRequired'));

        // Shaders description
        $(`<h3>${shaders.name}</h3>`).appendTo('#shaders');

        $(`<p>${LauncherUI.i18n.translate('Author')}: ${shaders.author}</p>`).appendTo('#shaders');

        if (shaders.images.length == 0)
            $(`<p>${LauncherUI.i18n.translate('NoImages')}</p>`).appendTo('#shaders');

        else shaders.images.forEach((image: any) => {
            const img = $(`<img src="${ path.join(constants.shadersDir, folder, image.file) }">`).appendTo('#shaders');
            
            const imageCaption = typeof image.caption === 'string' ?
                image.caption : (image.caption[LauncherUI.i18n.language] ?? image.caption['en']);

            const p = $(`<p>${imageCaption}</p>`).appendTo('#shaders');

            img.css('width', '100%');

            p.css('text-align', 'center');
            p.css('margin-top', '8px');
        });
    });

    $(`#shaders-list li[value=${LauncherLib.getConfig('shaders')}]`).addClass('selected');
    $('#shaders-list .selected-item span').text($(`#shaders-list li[value=${LauncherLib.getConfig('shaders')}]`).text());

    if (LauncherLib.getConfig('shaders') != 'none')
    {
        const selectedItem = $('#shaders-list .selected-item');

        selectedItem.removeClass('hint--small');
        selectedItem.addClass('hint--medium');
    }

    $('#shaders-list').on('selectionChanged', (e, data: any) => {
        LauncherLib.updateConfig('shaders', data.value);

        const selectedItem = $('#shaders-list div.selected-item');

        if (data.value == 'none')
        {
            selectedItem.removeClass('hint--medium');
            selectedItem.addClass('hint--small');
        }

        else if (!selectedItem.hasClass('hint--medium'))
        {
            selectedItem.removeClass('hint--small');
            selectedItem.addClass('hint--medium');
        }
    });

    $('#shaders-list .selected-item').attr('data-hint', LauncherUI.i18n.translate('ToggleShadersText'));

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
     * Wine recommendable only
     */

    const wineRecomOnly = LauncherLib.getConfig('lists_filters.wine');

    if (wineRecomOnly)
        $('#wine-recommendable').addClass('checkbox-active');

    $('#wine-recommendable').on('classChange', () => {
        const recommendableOnly = $('#wine-recommendable').hasClass('checkbox-active');

        LauncherLib.updateConfig('lists_filters.wine', recommendableOnly);

        if (recommendableOnly)
        {
            $(`#runners-list .list-item[recommendable]`).css('display', 'flex');
            $(`#runners-list .list-item:not([recommendable])`).css('display', 'none');
        }

        else $(`#runners-list .list-item`).css('display', 'flex');
    });

    /**
     * Wine versions manager
     */

    const activeRunner = LauncherLib.getConfig('runner');

    LauncherLib.getRunners().then(runners => {
        runners.forEach(category => {
            $(`<h3>${category.title}</h3>`).appendTo('#runners-list');

            category.runners.forEach(runner => {
                let item = $(`<div class="list-item"${runner.recommendable ? ' recommendable' : ''}>
                    ${runner.name}
                    <div>
                        <span></span>
                        <img class="item-delete" src="../images/delete.png">
                        <img class="item-download" src="../images/download.png">
                    </div>
                </div>`).appendTo('#runners-list');

                if (wineRecomOnly && !runner.recommendable)
                    item.css('display', 'none');
            
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
                            const unpacker = runner.archive === 'tar' ?
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
     * DXVK recommendable only
     */

     const dxvkRecomOnly = LauncherLib.getConfig('lists_filters.dxvk');

     if (dxvkRecomOnly)
         $('#dxvk-recommendable').addClass('checkbox-active');
 
     $('#dxvk-recommendable').on('classChange', () => {
         const recommendableOnly = $('#dxvk-recommendable').hasClass('checkbox-active');
 
         LauncherLib.updateConfig('lists_filters.dxvk', recommendableOnly);
 
         if (recommendableOnly)
         {
             $(`#dxvk-list .list-item[recommendable]`).css('display', 'flex');
             $(`#dxvk-list .list-item:not([recommendable])`).css('display', 'none');
         }
 
         else $(`#dxvk-list .list-item`).css('display', 'flex');
     });

    /**
     * DXVKs manager
     */
    
    const activeDXVK = LauncherLib.getConfig('dxvk');

    LauncherLib.getDXVKs().then(dxvks => {
        dxvks.forEach(dxvk => {
            let item = $(`<div class="list-item"${dxvk.recommendable ? ' recommendable' : ''}>
                ${dxvk.version}
                <div>
                    <span></span>
                    <img class="item-delete" src="../images/delete.png">
                    <img class="item-download" src="../images/download.png">
                </div>
            </div>`).appendTo('#dxvk-list');

            if (dxvkRecomOnly && !dxvk.recommendable)
                item.css('display', 'none');

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
                                WINEPREFIX: constants.prefixDir.get()
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
