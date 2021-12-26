import type {
    Runner,
    RunnerFamily
} from '../types/Runners';

import constants from '../Constants';
import Configs from '../Configs';
import AbstractInstaller from './AbstractInstaller';
import Downloader from './Downloader';
import Process from '../neutralino/Process';

declare const Neutralino;

class Stream extends AbstractInstaller
{
    public constructor(runner: Runner)
    {
        super(runner.uri, constants.paths.runnersDir);
    }
}

class Runners
{
    /**
     * Get the current using runner according to the config file
     */
    public static get current(): Promise<Runner|null>
    {
        return new Promise((resolve) => {
            Configs.get('runner').then((runner) => {
                if (typeof runner === 'string')
                    Runners.get(runner).then((runner) => resolve(runner));

                else resolve(null);
            });
        });
    }

    /**
     * Get runners list
     */
    public static list(): Promise<RunnerFamily[]>
    {
        return new Promise((resolve) => {
            constants.paths.runnersDir.then(async (runnersDir: string) => {
                let list: RunnerFamily[] = JSON.parse(await Neutralino.filesystem.readFile(`${constants.paths.appDir}/public/runners.json`));

                const installed: { entry: string, type: string }[] = await Neutralino.filesystem.readDirectory(runnersDir);

                let runners: RunnerFamily[] = [];

                list.forEach((family) => {
                    let newFamily: RunnerFamily = {
                        title: family.title,
                        runners: []
                    };

                    family.runners.forEach((runner) => {
                        let inst = false;

                        for (let dir of installed)
                            inst ||= dir.entry == runner.name;

                        newFamily.runners.push({
                            ...runner,

                            installed: inst
                        });
                    });

                    runners.push(newFamily);
                });

                resolve(runners);
            });
        });
    }

    /**
     * Get the runner with a specified name
     * 
     * @returns null if the runner with this name is not found
     */
    public static get(name: string): Promise<Runner|null>
    {
        return new Promise((resolve) => {
            this.list().then((list) => {
                for (const family of list)
                    for (const runner of family.runners)
                        if (runner.name == name)
                        {
                            resolve(runner);

                            return;
                        }

                resolve(null);
            });
        });
    }

    /**
     * Download runner to the [constants.paths.runners] directory
     * 
     * @param runner runner object or name
     * @returns null if the runner with specified name dosen't exist. Otherwise - installation stream
     */
    public static download(runner: Runner|Runner['name']): Promise<null|Stream>
    {
        return new Promise((resolve) => {
            // If we provided runner parameter with a name of a runner
            // then we should find this runner and call this method for it
            if (typeof runner == 'string')
            {
                this.get(runner).then((foundRunner) => {
                    resolve(foundRunner === null ? null : new Stream(foundRunner));
                });
            }

            // Otherwise we can use runner.uri and so on to download runner
            else resolve(new Stream(runner));
        });
    }

    /**
     * Get path to the winetricks.sh file
     * 
     * If this file is not downloaded - then this method will download it
     * and return the path after it
     */
    public static getWinetricks(): Promise<string>
    {
        return new Promise(async (resolve) => {
            const winetricksPath = `${await constants.paths.launcherDir}/winetricks.sh`;

            Neutralino.filesystem.getStats(winetricksPath)
                .then(() => resolve(winetricksPath))
                .catch(() => {
                    Downloader.download(constants.uri.winetricks, winetricksPath).then((stream) => {
                        stream.finish(() => resolve(winetricksPath));
                    });
                });
        });
    }

    /**
     * Create wine prefix using the current selected wine
     * 
     * @param path folder to create prefix in
     * @param progress function that will be called with every creation step
     * 
     * @returns false if there's no selected wine version. Otherwise true
     */
    public static createPrefix(path: string, progress?: (output: string, current: number, total: number) => void): Promise<boolean>
    {
        const installationSteps = [
            // corefonts
            'Executing w_do_call corefonts',
            'Executing load_corefonts',
            'Executing load_andale',
            'Executing load_arial',
            'Executing load_comicsans',
            'Executing load_courier',
            'Executing load_georgia',
            'Executing load_impact',
            'Executing load_times',
            'Executing load_trebuchet',
            'Executing load_verdana',
            'Executing load_webdings',

            // usetakefocus=n (fullscreen input issues fix)
            'Executing load_usetakefocus n'
        ];

        return new Promise((resolve) => {
            this.current.then((runner) => {
                if (runner === null)
                    resolve(false);
    
                else
                {
                    this.getWinetricks().then(async (winetricks) => {
                        // let installationProgress = 0;

                        const process = await Process.run(`bash '${Process.addSlashes(winetricks)}' corefonts usetakefocus=n`, {
                            env: {
                                WINE: `${await constants.paths.runnersDir}/${runner.name}/${runner.files.wine}`,
                                WINESERVER: `${await constants.paths.runnersDir}/${runner.name}/${runner.files.wineserver}`,
                                WINEPREFIX: path
                            }
                        });

                        // todo: add process output reading

                        process.finish(() => resolve(true));
            
                        /*installerProcess.stdout.on('data', (data: string) => {
                            let str = data.toString();
            
                            for (let i = 0; i < installationSteps.length; ++i)
                                if (str.includes(installationSteps[i]))
                                {
                                    installationProgress = i + 1;
            
                                    break;
                                }
            
                            progress(str, installationProgress, installationSteps.length);
                        });*/
                    });
                }
            });
        });
    }
}

export default Runners;

export { Stream };

export type {
    Runner,
    RunnerFamily
};
