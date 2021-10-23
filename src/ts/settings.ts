const fs = require('fs');
const path = require('path');
const { exec } = require('child_process');

import $ from 'cash-dom';
import { Genshinlib } from './Genshinlib';

$(() => {
    $('.menu-item').on('click', (e) => {
        // @ts-expect-error
        $('.settings')[0].scrollTop = document.getElementById(e.target.getAttribute('anchor')).offsetTop - 16;

        $('.menu-item').removeClass('menu-item-active');
        $(e.target).addClass('menu-item-active');
    });

    $('.settings').on('scroll', () => {
        // @ts-expect-error
        let anchor = $('.settings-item').filter((index, item) => $(item).offset().top < 264).last()[0].id;

        $('.menu-item').removeClass('menu-item-active');
        $(`.menu-item[anchor=${anchor}]`).addClass('menu-item-active');
    });

    let activeRunner = Genshinlib.getConfig().runner;

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

                        Genshinlib.downloadFile(runner.uri, path.join(Genshinlib.launcherDir, runner.name), (current: number, total: number, difference: number) => {
                            div.text(`${ Math.round(current / total * 100) }%`);
                        }).then(() => {
                            let unpacker = runner.archive === 'tar' ?
                                Genshinlib.untar : Genshinlib.unzip;

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
                            Genshinlib.updateConfig({
                                runner: {
                                    name: runner.name,
                                    folder: runner.folder,
                                    executable: runner.executable
                                }
                            });

                            $('#runners-list > .list-item').removeClass('list-item-active');
                            item.addClass('list-item-active');
                        }
                    }
                });
            });
        });
    });

    let activeDXVK = Genshinlib.getConfig().dxvk;

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

                    Genshinlib.downloadFile(dxvk.uri, path.join(Genshinlib.launcherDir, 'dxvk-' + dxvk.version), (current: number, total: number, difference: number) => {
                        div.text(`${ Math.round(current / total * 100) }%`);
                    }).then(() => {
                        Genshinlib.untar(
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
                            Genshinlib.updateConfig({
                                dxvk: dxvk.version
                            });
    
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
