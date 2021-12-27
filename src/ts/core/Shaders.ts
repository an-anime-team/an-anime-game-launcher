import YAML from 'yaml';

import type { Shader } from '../types/Shaders';

import Configs from '../Configs';
import constants from '../Constants';
import promisify from './promisify';

declare const Neutralino;

export default class Shaders
{
    /**
     * Get or set shaders to the config file
     */
    public static current(shader: Shader|null = null): Promise<Shader|null>
    {
        return new Promise(async (resolve) => {
            if (shader === null)
            {
                Configs.get('shaders').then(async (shader) => {
                    resolve(shader ? await this.get(shader as string) : null);
                });
            }
            
            else
            {
                Configs.set('shaders', typeof shader === 'string' ? shader : shader.folder);

                resolve(typeof shader === 'string' ? await this.get(shader) : shader);
            }
        });
    }

    /**
     * Get list of available shaders
     */
    public static list(): Promise<Shader[]>
    {
        return new Promise((resolve) => {
            Neutralino.filesystem.readDirectory(constants.paths.shadersDir)
                .then((dirs) => {
                    let callbacks: any[] = [];

                    dirs.forEach((dir) => {
                        if (dir.entry != '.' && dir.entry != '..')
                        {
                            callbacks.push((): Promise<Shader|null> => {
                                return new Promise((resolve) => {
                                    Neutralino.filesystem.readFile(`${constants.paths.shadersDir}/${dir.entry}/shaders.yaml`)
                                        .then((shaders) => resolve({
                                            ...YAML.parse(shaders),
                                            folder: dir.entry
                                        }))
                                        .catch(() => resolve(null));
                                });
                            });
                        }
                    });

                    const pipeline = promisify({
                        callbacks,
                        callAtOnce: true
                    });

                    pipeline.then((output: object) => {
                        resolve(Object.values(output).filter((shader) => shader !== null));
                    });
                })
                .catch(() => resolve([]));
        });
    }

    /**
     * Get shader info
     */
    public static get(folder: string): Promise<Shader|null>
    {
        return new Promise((resolve) => {
            Neutralino.filesystem.readFile(`${constants.paths.shadersDir}/${folder}/shaders.yaml`)
                .then((shaders) => {
                    resolve(YAML.parse(shaders));
                })
                .catch(() => resolve(null));
        });
    }
};
