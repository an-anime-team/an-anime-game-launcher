import Window from './neutralino/Window';
import Process from './neutralino/Process';

import constants from './Constants';
import Configs from './Configs';

import ProgressBar from './launcher/ProgressBar';
import State from './launcher/State';
import Debug from './core/Debug';
import IPC from './core/IPC';

declare const Neutralino;

export default class Launcher
{
    public state?: State;
    public progressBar?: ProgressBar;

    protected settingsMenu?: Process;

    public constructor(onMount)
    {
        onMount(() => {
            this.progressBar = new ProgressBar(this);
            this.state = new State(this);
        });
    }

    public showSettings(): Promise<boolean>
    {
        return new Promise(async (resolve) => {
            if (this.settingsMenu)
                resolve(false);
            
            else
            {
                this.settingsMenu = undefined;

                const window = await Window.open('settings', {
                    title: 'Settings',
                    width: 900,
                    height: 600,
                    // enableInspector: true,
                    exitProcessOnClose: false
                });

                if (window.status)
                {
                    this.settingsMenu = new Process(window.data!.pid);

                    this.settingsMenu.finish(() => {
                        this.settingsMenu = undefined;

                        IPC.read().then((records) => {
                            records.forEach((record) => {
                                if (record.data.type !== undefined && record.data.type === 'log')
                                    Debug.merge(record.pop().data.records);
                            });
                        });
                        
                        Window.current.show();
                    })

                    Window.current.hide();
                }

                resolve(window.status);
            }
        });
    }

    /**
     * Get launcher social buttons uri
     */
    public getSocial(): Promise<string>
    {
        return new Promise(async (resolve) => {
            resolve(`https://${constants.placeholders.lowercase.first}.${constants.placeholders.lowercase.company}.com/launcher/10/${await Configs.get('lang.launcher')}?api_url=https%3A%2F%2Fapi-os-takumi.${constants.placeholders.lowercase.company}.com%2Fhk4e_global&key=gcStgarh&prev=false`);
        });
    }
};
