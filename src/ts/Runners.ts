import {
    Runner,
    RunnerFamily
} from './types/Runners';

import constants from './Constants';
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
     * Get runners list
     */
    public static get(): Promise<RunnerFamily[]>
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
     * Download runner to the [constants.paths.runners] directory
     * 
     * @param runner runner object or name
     * @returns null if the runner with specified name is not exists. Otherwise - installation stream
     */
    public static download(runner: Runner|Runner['name']): Promise<null|Stream>
    {
        return new Promise(async (resolve) => {
            // If we provided runner property as a name of the runner
            // then we should find this runner and call this method from it
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
