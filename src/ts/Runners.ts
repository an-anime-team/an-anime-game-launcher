import {
    Runner,
    RunnerFamily
} from './types/Runners';

import Constants from './Constants';
import AbstractInstaller from './AbstractInstaller';

declare const Neutralino;

class Stream extends AbstractInstaller
{
    public constructor(runner: Runner)
    {
        super(runner.uri, Constants.paths.runners);
    }
}

class Runners
{
    /**
     * Get runners list
     */
    public static get(): Promise<RunnerFamily[]>
    {
        return new Promise((resolve) => {
            Constants.paths.runners.then(async (runnersDir: string) => {
                let list: RunnerFamily[] = JSON.parse(await Neutralino.filesystem.readFile(`${Constants.paths.app}/public/runners.json`));

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
     * Download runner to the [Constants.paths.runners] directory
     * 
     * @param runner runner object or name
     * @returns null if the runner with specified name dosen't exist. Otherwise - installation stream
     */
    public static download(runner: Runner|Runner['name']): Promise<null|Stream>
    {
        return new Promise(async (resolve) => {
            // If we provided runner parameter with a name of a runner
            // then we should find this runner and call this method for it
            if (typeof runner == 'string')
            {
                let foundRunner = null;

                (await this.get()).forEach((family) => {
                    family.runners.forEach((familyRunner) => {
                        if (familyRunner.name == runner)
                            foundRunner = familyRunner;
                    });
                });

                resolve(foundRunner === null ? null : new Stream(foundRunner));
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
