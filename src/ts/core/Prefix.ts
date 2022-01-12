import constants from '../Constants';
import Process from '../neutralino/Process';
import Debug, { DebugThread } from './Debug';
import Downloader from './Downloader';
import Runners from './Runners';

declare const Neutralino;

export default class Prefix
{
    /**
     * Check if the wine prefix is created in the specified path
     */
    public static exists(path: string|null = null): Promise<boolean>
    {
        return new Promise(async (resolve) => {
            path ??= await constants.paths.prefix.current;

            Neutralino.filesystem.getStats(`${path}/drive_c`)
                .then(() => {
                    Debug.log({
                        function: 'Prefix.exists',
                        message: `Prefix exists here: ${path}`
                    });

                    resolve(true);
                })
                .catch(() => {
                    Debug.log({
                        function: 'Prefix.exists',
                        message: `Prefix doesn't exist here: ${path}`
                    });

                    resolve(false);
                });
        });
    }

    /**
     * Get path to the winetricks.sh file
     * 
     * If this file is not downloaded - then this method will download it
     * and return the path to it
     */
    public static getWinetricks(): Promise<string>
    {
        return new Promise(async (resolve) => {
            const winetricksPath = `${await constants.paths.launcherDir}/winetricks.sh`;

            Neutralino.filesystem.getStats(winetricksPath)
                .then(() => resolve(winetricksPath))
                .catch(() => {
                    Downloader.download(constants.uri.winetricks, winetricksPath).then((stream) => {
                        stream.finish(async () => {
                            await Neutralino.os.execCommand(`chmod +x "${Process.addSlashes(winetricksPath)}"`);

                            resolve(winetricksPath);
                        });
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
    public static create(path: string, progress?: (output: string, current: number, total: number) => void): Promise<boolean>
    {
        const debugThread = new DebugThread('Prefix.create', 'Creating wine prefix');

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
            Runners.current().then((runner) => {
                if (runner === null)
                {
                    debugThread.log('Runner doesn\'t selected');

                    resolve(false);
                }
    
                else
                {
                    debugThread.log(`Using runner: ${runner.title} (${runner.name})`);

                    this.getWinetricks().then(async (winetricks) => {
                        let installationProgress = 0;

                        const process = await Process.run(`"${Process.addSlashes(winetricks)}" corefonts usetakefocus=n`, {
                            env: {
                                WINE: `${await constants.paths.runnersDir}/${runner.name}/${runner.files.wine}`,
                                WINESERVER: `${await constants.paths.runnersDir}/${runner.name}/${runner.files.wineserver}`,
                                WINEPREFIX: path
                            }
                        });

                        process.outputInterval = null;

                        // If progress specified
                        if (progress)
                        {
                            process.outputInterval = 1500;

                            process.output((output) => {
                                for (let i = 0; i < installationSteps.length; ++i)
                                    if (output.includes(installationSteps[i]))
                                    {
                                        installationProgress = i + 1;
                
                                        break;
                                    }

                                if (output != '')
                                {
                                    const lastLine = output.split(/\r\n|\r|\n/gm)
                                        .filter((line) => line.length > 0)
                                        .pop()?.trim();

                                    if (lastLine && !lastLine.includes('------'))
                                        progress(lastLine, installationProgress, installationSteps.length);
                                }
                            });
                        }

                        process.finish(() => {
                            debugThread.log('Prefix creation completed');

                            resolve(true);
                        });
                    });
                }
            });
        });
    }
};
