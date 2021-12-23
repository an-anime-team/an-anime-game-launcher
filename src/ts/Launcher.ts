import constants from './Constants';
import Configs from './Configs';
import Background from './launcher/Background';
import ProgressBar from './launcher/ProgressBar';

export default class Launcher
{
    public app;
    public progressBar: ProgressBar;

    public constructor(app)
    {
        this.app = app;
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

            resolve();
        });
    }
};
