import type Launcher from '../Launcher';

type InitOptions = {
    label: string|((current: number, total: number, difference: number) => string);

    update?: (current: number, total: number, difference: number) => void;
    finish?: () => void;

    showSpeed: boolean;
    showEta: boolean;
    showPercents: boolean;
    showTotals: boolean;
};

export default class ProgressBar
{
    public launcher: Launcher;

    public downloaderElement: HTMLElement;
    public progressElement: HTMLElement;

    public downloadedLabelElement: HTMLElement;
    public speedLabelElement: HTMLElement;
    public etaLabelElement: HTMLElement;

    public options?: InitOptions;

    protected progress = {
        beganAt: 0,
        prevTime: 0,
        temp: 0
    };

    public constructor(launcher: Launcher)
    {
        this.launcher = launcher;

        this.downloaderElement = <HTMLElement>document.getElementsByClassName('downloader-panel')[0];

        this.progressElement = <HTMLElement>this.downloaderElement.children[1].children[0];

        [
            this.downloadedLabelElement,
            this.speedLabelElement,
            this.etaLabelElement
        ] = <HTMLCollectionOf<HTMLElement>>this.downloaderElement.children[0].children;
    }

    /**
     * Show all the progress bar
     */
    public show()
    {
        this.downloaderElement.style['display'] = 'block';
    }

    /**
     * Hide all the progress bar
     */
    public hide()
    {
        this.downloaderElement.style['display'] = 'none';
    }

    /**
     * Init progress bar with some options
     */
    public init(options: InitOptions)
    {
        this.options = options;

        this.speedLabelElement.style['display'] = options.showSpeed ? 'inline-block' : 'none';
        this.etaLabelElement.style['display'] = options.showEta ? 'inline-block' : 'none';

        this.speedLabelElement.textContent = '';
        this.etaLabelElement.textContent = '';

        this.downloadedLabelElement.textContent = typeof options.label === 'string' ?
            options.label : '';

        this.progress = {
            beganAt: Date.now(),
            prevTime: Date.now(),
            temp: 0
        };
    }

    /**
     * Update progress bar
     */
    public update(current: number, total: number, difference: number): void
    {
        // Update progress label if it is not a static text
        if (typeof this.options!.label === 'function')
            this.downloadedLabelElement.textContent = this.options!.label(current, total, difference);

        // Otherwise update percents and totals if we should
        else if (this.options!.showPercents || this.options!.showPercents)
        {
            this.downloadedLabelElement.textContent = this.options!.label;

            if (this.options!.showPercents)
                this.downloadedLabelElement.textContent += ` ${Math.round(current / total * 100)}%`;

            if (this.options!.showTotals)
                this.downloadedLabelElement.textContent += ` (${ProgressBar.prettifyBytes(current)} / ${ProgressBar.prettifyBytes(total)})`;
        }

        // Update progress width
        this.progressElement.style['width'] = `${(current / total * 100).toFixed(3)}%`;

        this.progress.temp += difference;

        // If the delay between update calls was more than 1 second - then update some stats
        if (Date.now() - this.progress.prevTime > 1000)
        {
            // Update speed if we need
            if (this.options!.showSpeed)
                this.speedLabelElement.textContent = `${ProgressBar.prettifyBytes(this.progress.temp / (Date.now() - this.progress.prevTime) * 1000)}/s`;
            
            // Update ETA if we need
            if (this.options!.showEta)
            {
                type etaType = string | number;
            
                let elapsed = (Date.now() - this.progress.beganAt) / 1000;
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

                this.etaLabelElement.textContent = `ETA: ${etaHours}:${etaMinutes}:${etaSeconds}`;
            }

            this.progress.prevTime = Date.now();
            this.progress.temp = 0;
        }

        // Call user-provided update callback
        if (this.options!.update)
            this.options!.update(current, total, difference);

        if (current === total && this.options!.finish)
            this.options!.finish();
    }

    /**
     * Prettify bytes
     * 
     * @param bytes - amount of bytes
     * 
     * @returns prettified string ("10 B", "7 KB", etc)
     */
    public static prettifyBytes (bytes: number): string
    {
        const types = [
            {
                name: 'B',
                multiplier: 1
            },
            {
                name: 'KB',
                multiplier: 1024
            },
            {
                name: 'MB',
                multiplier: 1024 * 1024
            },
            {
                name: 'GB',
                multiplier: 1024 * 1024 * 1024
            }
        ].filter(type => type.multiplier < bytes);

        return types.length == 0 ?
            `${bytes} B` :
            `${(bytes / types[types.length - 1].multiplier).toFixed(2)} ${types[types.length - 1].name}`;
    }
};

export type { InitOptions };
