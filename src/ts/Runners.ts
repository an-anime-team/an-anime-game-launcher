import type {
    Runner,
    RunnerFamily
} from './types/Runners';

import constants from './Constants';

class Runners
{
    /**
     * Get runners list
     * 
     * @returns Promise<Runner[]>
     */
    public static get(): Promise<Runner[]>
    {
        return new Promise((resolve) => {
            constants.paths.runners.then(async (runnersDir: string) => {
                // @ts-expect-error
                let list: RunnerFamily[] = JSON.parse(await Neutralino.filesystem.readFile(`${constants.dirs.app}/public/runners.json`));

                // @ts-expect-error
                const installed: { entry: string, type: string }[] = await Neutralino.filesystem.readDirectory(runnersDir);

                let runners = [];

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

    public static download(runner: Runner|Runner['name']): Promise<boolean>
    {
        return new Promise((resolve) => {
            
        });
    }
}

export default Runners;

export type {
    Runner,
    RunnerFamily
};
