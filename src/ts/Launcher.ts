import constants from './Constants';
import Configs from './Configs';

import Background from './launcher/Background';
import ProgressBar from './launcher/ProgressBar';
import State from './launcher/State';

export default class Launcher
{
    public app;

    public state: State;
    public progressBar: ProgressBar;

    public constructor(app)
    {
        this.app = app;

        this.state = new State(this);
        this.progressBar = new ProgressBar(this);

        // Progress bar test
        this.progressBar.init({
            label: 'Abobus',
            showSpeed: true,
            showEta: true,
            showPercents: true,
            showTotals: true,

            finish: () => this.progressBar.hide()
        });

        this.progressBar.show();

        const t = (curr) => {
            if (curr <= 3000)
            {
                this.progressBar.update(curr, 3000, 1);

                setTimeout(() => t(curr + 1), 10);
            }
        };

        t(0);
    }

    /**
     * Update launcher background picture
     */
    public updateBackground(): Promise<void>
    {
        return new Promise(async (resolve) => {
            this.app.uri.background = await Background.get();

            resolve();
        });
    }

    /**
     * Update launcher social buttons
     */
    public updateSocial(): Promise<void>
    {
        return new Promise(async (resolve) => {
            this.app.uri.social = `https://${constants.placeholders.lowercase.first}.${constants.placeholders.lowercase.company}.com/launcher/10/${await Configs.get('lang.launcher')}?api_url=https%3A%2F%2Fapi-os-takumi.${constants.placeholders.lowercase.company}.com%2Fhk4e_global&key=gcStgarh&prev=false`;

            const iframe = <HTMLElement>document.getElementById('launcher-content')!.children[0];

            iframe.onload = () => {
                // Buttons are not working and we can't edit
                // them inside iframe because of Neutralino restrictions
                
                /*console.log(window.frames['launcher-content-frame']);

                window.frames['launcher-content-frame'].document.querySelectorAll('a[href]').forEach((link) => {
                    console.log(link);

                    link.addEventListener('click', () => {
                        Neutralino.os.open(link.getAttribute('href'));
                    });
                });*/

                resolve();
            };
        });
    }
};
