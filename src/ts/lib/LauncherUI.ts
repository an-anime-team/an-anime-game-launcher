import $ from 'cash-dom';

import constants from './constants';
import LauncherLib from './LauncherLib';
import i18n from './i18n';
import Tools from './Tools';
import Colors from './Colors';

type LauncherState =
    'patch-unavailable' |
    'test-patch-available' |
    'patch-applying' |
    'game-update-available' |
    'game-installation-available' |
    'game-voice-update-required' |
    'game-launch-available';

export default class LauncherUI
{
    protected static _launcherState: LauncherState = 'game-launch-available';
    protected static _i18n: any;

    public static get launcherState(): LauncherState
    {
        return this._launcherState;
    }

    public static get i18n(): any
    {
        if (!this._i18n)
            this._i18n = i18n;
        
        return this._i18n;
    }

    public static setState (state: LauncherState)
    {
        $('#downloader-panel').css('display', 'none');
        $('#launch').css('display', 'block');

        switch (state)
        {
            case 'patch-unavailable':
                $('#launch').text(this.i18n.translate('PatchRequired'));
                $('#launch').attr('disabled', 'disabled');

                $('#launch').addClass('hint--top')
                            .addClass('hint--medium');

                $('#launch').attr('data-hint', this.i18n.translate('PatchRequiredHint'));

                break;

            case 'test-patch-available':
                $('#launch').text(this.i18n.translate('TestPatch'));

                $('#launch').addClass('button-blue')
                            .addClass('hint--top')
                            .addClass('hint--large');

                $('#launch').attr('data-hint', this.i18n.translate('TestPatchHint'));

                break;

            case 'patch-applying':
                $('#launch').text(this.i18n.translate('ApplyPatch'));
                $('#launch').attr('disabled', 'disabled');

                break;

            case 'game-update-available':
                $('#launch').text(this.i18n.translate('Update'));

                break;

            case 'game-installation-available':
                $('#launch').text(this.i18n.translate('Install'));

                break;

            case 'game-voice-update-required':
                $('#launch').text(this.i18n.translate('Update'));

                break;

            case 'game-launch-available':
                $('#launch').removeAttr('disabled')
                    .removeAttr('data-hint');

                $('#launch').removeClass('button-blue')
                    .removeClass('hint--top')
                    .removeClass('hint--medium')
                    .removeClass('hint--large');

                $('#launch').text(this.i18n.translate('Launch'));

                break;
        }

        this._launcherState = state;
    }

    public static async updateLauncherState(data: any = null)
    {
        const gameData  = data ?? await LauncherLib.getData();
        const patchInfo = await LauncherLib.getPatchInfo();

        // Update available
        if (LauncherLib.version != gameData.game.latest.version)
            this.setState(LauncherLib.version === null ? 'game-installation-available' : 'game-update-available');

        // Voice pack update required
        else if (LauncherLib.getConfig('lang.voice.active') != LauncherLib.getConfig('lang.voice.installed'))
            this.setState('game-voice-update-required');

        // Patch version is incorrect
        else if (LauncherLib.getConfig('patch') && LauncherLib.getConfig('patch.version') != patchInfo.version)
        {
            // Patch is not available
            if (patchInfo.version !== gameData.game.latest.version)
                this.setState('patch-unavailable');

            // Patch available
            else if (patchInfo.version === gameData.game.latest.version)
            {
                // Patch is stable
                if (patchInfo.state == 'stable')
                {
                    console.log(`%c> Applying patch...`, 'font-size: 16px');

                    this.setState('patch-applying');

                    LauncherLib.patchGame(() => {
                        this.setState('game-launch-available');
                    }, data => console.log(data.toString()));
                }

                // Patch is in testing phase
                else this.setState('test-patch-available');
            }
        }

        // Current patch is in testing phase,
        // but stable is available
        else if (LauncherLib.getConfig('patch') && LauncherLib.getConfig('patch.version') == patchInfo.version && LauncherLib.getConfig('patch.state') == 'testing' && patchInfo.state == 'stable')
        {
            console.log(`%c> Applying patch...`, 'font-size: 16px');

            this.setState('patch-applying');

            LauncherLib.patchGame(() => {
                this.setState('game-launch-available');
            }, data => console.log(data.toString()));
        }

        else this.setState('game-launch-available');
    }

    protected static progressBar = {
        beganAt: 0,
        prevTime: 0,
        temp: 0
    };

    public static initProgressBar (): void
    {
        this.progressBar = {
            beganAt: Date.now(),
            prevTime: Date.now(),
            temp: 0
        };

        $('#downloaded').text('');
        $('#speed').text('');
        $('#eta').text('');

        $('#downloader .progress').css('width', '0');

        $('#downloader-panel').css('display', 'block');
        $('#launch').css('display', 'none');
    }

    public static updateProgressBar (prefix: string, current: number, total: number, difference: number): void
    {
        $('#downloaded').text(`${prefix}: ${ Math.round(current / total * 100) }% (${ (current / 1024 / 1024 / 1024).toFixed(2) } GB / ${ Math.round(total / 1024 / 1024 / 1024).toFixed(2) } GB)`);
                        
        this.progressBar.temp += difference;

        if (Date.now() - this.progressBar.prevTime > 1000)
        {
            type etaType = string | number;
            
            let elapsed = (Date.now() - this.progressBar.beganAt) / 1000;
            let eta = Math.round(total * elapsed / current - elapsed);
            
            let etaHours: etaType   = Math.floor(eta / 3600),
                etaMinutes: etaType = Math.floor((eta - etaHours * 3600) / 60),
                etaSeconds: etaType = eta - etaHours * 3600 - etaMinutes * 60;

            if (etaHours < 10)
                etaHours = '0' + etaHours.toString();

            if (etaMinutes < 10)
                etaMinutes = '0' + etaMinutes.toString();

            if (etaSeconds < 10)
                etaSeconds = '0' + etaSeconds.toString();

            $('#downloader .progress').css('width', `${ Math.round(current / total * 100) }%`);
            $('#speed').text(`${ (this.progressBar.temp / (Date.now() - this.progressBar.prevTime) * 1000 / 1024 / 1024).toFixed(2) } MB/s`);
            $('#eta').text(`ETA: ${etaHours}:${etaMinutes}:${etaSeconds}`);

            this.progressBar.prevTime = Date.now();
            this.progressBar.temp = 0;
        }
    }

    public static clearProgressBar(): void
    {
        $('#downloader-panel').css('display', 'none');
        $('#launch').css('display', 'block');
        
        $('#downloaded').text('');
        $('#speed').text('');
        $('#eta').text('');

        $('#downloader .progress').css('width', '0');
    }

    public static updateBackground (): void
    {
        LauncherLib.getBackgroundUri().then(uri => {
            const style = `url(${uri})`;

            if ($('body').css('background-image') != style)
            {
                $('body').css('background-image', style);

                /**
                 * Calculating background's left-bottom corner mean brightness
                 * to make the progress bar's theme dark or light
                */
                Tools.getImagePixels(uri).then(pixels => {
                    const sector = pixels.filter(pixel => pixel.y > 514 && pixel.x < 720);

                    // For some strange reasons we can't
                    // make an object of r-g-b properties
                    let meanColor = [0, 0, 0];

                    sector.forEach(pixel => {
                        meanColor[0] += pixel.color.r;
                        meanColor[1] += pixel.color.g;
                        meanColor[2] += pixel.color.b;
                    });

                    meanColor = meanColor.map(_ => _ / sector.length);

                    const meanBrightness = Colors.rgb2brightness({
                        r: meanColor[0],
                        g: meanColor[1],
                        b: meanColor[2]
                    });

                    console.log(`Mean color: rgb(${Math.round(meanColor[0])},${Math.round(meanColor[1])},${Math.round(meanColor[2])})`);
                    console.log(`Background mean brightness is ${meanBrightness}`);

                    // Image is dark so we make the progress bar light
                    if (meanBrightness < 50)
                        $('#downloader-panel').removeClass('dark');

                    // Otherwise image is bright so the progress bar should be dark
                    else if (!$('#downloader-panel').hasClass('dark'))
                        $('#downloader-panel').addClass('dark');
                });
            }
        });
    }

    public static updateSocial (): void
    {
        const socialUri = `https://${constants.placeholders.lowercase.first}.${constants.placeholders.lowercase.company}.com/launcher/10/${LauncherLib.getConfig('lang.launcher')}?api_url=https%3A%2F%2Fapi-os-takumi.${constants.placeholders.lowercase.company}.com%2Fhk4e_global&prev=false`;

        fetch(socialUri)
            .then(res => res.text())
            .then(body => {
                $('#__layout').remove();
                $(body).find('#__layout').appendTo('#launchcontent');

                // Next banner button
                // TODO
                /*$('#launchcontent .home__main .swiper-button-prev').on('click', () => {

                });*/

                $('#launchcontent .home__main .home-swiper-wrap').remove();
                $('#launchcontent .home__main .home-news').remove();
            });
    }

    public static updateLang (lang: string|null = null): void
    {
        if (lang !== null)
            this.i18n.setLang(lang);

        $('*[i18id]').each((i, element) => {
            element.innerText = this.i18n.translate(element.getAttribute('i18id')!);
        });
    }
}
