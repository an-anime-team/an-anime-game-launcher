import YAML from 'yaml';

import type { Shader } from '../types/Shaders';

import { Configs, promisify } from '../../empathize';

import constants from '../Constants';



export default class Shaders
{
    // Required due to the Neutralino restrictions
    protected static readonly shaders = {
        'notahuman': {
            config: this.get('notahuman'),
            images: {
                'preview.png': import('../../assets/shaders/notahuman/preview.png')
            }
        },
        'yagocl': {
            config: this.get('yagocl'),
            images: {}
        }
    };

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
            const pipeline = promisify({
                callbacks: Object.values(this.shaders).map((value) => value.config),
                callAtOnce: true
            });

            pipeline.then((output: object) => {
                resolve(Object.values(output));
            });
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
                    resolve({
                        ...YAML.parse(shaders),
                        folder: folder
                    });
                })
                .catch(() => resolve(null));
        });
    }

    /**
     * Get path to the shader picture
     * 
     * Required due to the Neutralino restrictions
     */
    public static getPicture(folder: string, file: string): Promise<string|null>
    {
        return new Promise((resolve) => {
            if (this.shaders[folder] === undefined || this.shaders[folder].images[file] === undefined)
                resolve(null);

            else this.shaders[folder].images[file].then((image) => resolve(image.default));
        })
    }
};
