import {
    Runner,
    RunnerFamily
} from '../types/Runners';

import constants from '../Constants';
import Configs from '../Configs';
import AbstractInstaller from './AbstractInstaller';

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
}

export default Runners;

export { Stream };

export type {
    Runner,
    RunnerFamily
};
