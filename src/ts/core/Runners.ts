import YAML from 'yaml';

import type {
    Runner,
    RunnerFamily
} from '../types/Runners';

import { Configs, Process, path, Cache, fetch } from '../../empathize';
import { DebugThread } from '@empathize/framework/dist/meta/Debug';

import constants from '../Constants';
import AbstractInstaller from './AbstractInstaller';

// If true, runners list will be loaded directly from the file
// instead of the repository
// 
// ! Should always be false for release builds
const LOAD_DIRECTLY = false;


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
     * or set the new one
     */
    public static current(runner: Runner|Runner['name']|null = null): Promise<Runner|null>
    {
        return new Promise(async (resolve) => {
            if (runner === null)
            {
                Configs.get('runner').then((runner) => {
                    if (typeof runner === 'string')
                        Runners.get(runner).then((runner) => resolve(runner));

                    else resolve(null);
                });
            }

            else
            {
                await Configs.set('runner', typeof runner === 'string' ?
                    runner : runner.name);

                await Configs.flush();

                resolve(typeof runner === 'string' ?
                    await this.get(runner) : runner);
            }
        });
    }

    /**
     * Get runners list
     */
    public static list(): Promise<RunnerFamily[]>
    {
        return new Promise(async (resolve) => {
            const runnersDir = await constants.paths.runnersDir;

            Neutralino.filesystem.readDirectory(runnersDir)
                .then((folders) => resolveList(folders))
                .catch(() => resolveList([]));
            
            const resolveList = async (folders: { entry: string, type: string }[]) => {
                let list: RunnerFamily[] = [];
                let runners: RunnerFamily[] = [];

                const runners_list = await Cache.get('Runners.list.remote');

                // If the runners cache is no expired - return it
                if (!LOAD_DIRECTLY && runners_list && !runners_list.expired)
                    list = runners_list.value['list'];

                else
                {
                    // Otherwise fetch remote list
                    const response = await fetch(constants.uri.runners_list, 2000);

                    // If it wasn't fetched - load locally stored one
                    if (!response.ok || LOAD_DIRECTLY)
                        list = YAML.parse(await Neutralino.filesystem.readFile(`${constants.paths.appDir}/public/runners.yaml`));

                    else
                    {
                        // Otherwise if the fetched list have the same content length as cached one
                        // then ignore it and use the cached one because they're the same
                        // otherwise load remote one
                        list = runners_list && runners_list.value['length'] == response.length ?
                            runners_list.value['list'] :
                            YAML.parse(await response.body());

                        // Update the cache record for the next 24 hours
                        Cache.set('Runners.list.remote', {
                            length: response.length,
                            list: list
                        }, 3600 * 24);
                    }
                }

                list.forEach((family) => {
                    let newFamily: RunnerFamily = {
                        title: family.title,
                        runners: []
                    };

                    family.runners.forEach((runner) => {
                        let inst = false;

                        for (let dir of folders)
                            inst ||= dir.entry == runner.name;

                        newFamily.runners.push({
                            ...runner,

                            installed: inst
                        });
                    });

                    runners.push(newFamily);
                });

                resolve(runners);
            };
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
     * Delete specified runner
     */
    public static delete(runner: Runner|Runner['name']): Promise<void>
    {
        const debugThread = new DebugThread('Runners.delete', `Deleting runner ${typeof runner === 'string' ? runner : runner.name}`);
        
        return new Promise(async (resolve) => {
            const name = typeof runner !== 'string' ?
                runner.name : runner;

            Process.run(`rm -rf "${path.addSlashes(await constants.paths.runnersDir + '/' + name)}"`)
                .then((process) => {
                    process.finish(() => {
                        debugThread.log('Runner deleted');

                        resolve();
                    });
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
