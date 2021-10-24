import $ from 'cash-dom';
import { Genshinlib } from './Genshinlib';
import { i18n } from './i18n';

type LauncherState =
    'patch-unavailable' |
    'test-patch-available' |
    'patch-applying' |
    'game-update-available' |
    'game-installation-available' |
    'game-launch-available';

export class LauncherUI
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
        Genshinlib.getBackgroundUri().then(uri => $('body').css('background-image', `url(${uri})`));
    }

    public static updateSocial (): void
    {
        fetch(`https://genshin.mihoyo.com/launcher/10/${Genshinlib.getConfig('lang.launcher')}?api_url=https%3A%2F%2Fapi-os-takumi.mihoyo.com%2Fhk4e_global&prev=false`)
            .then(res => res.text())
            .then(body => {
                $('#__layout').remove();
                $(body).find('#__layout').appendTo('#launchcontent');

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