import constants from '../Constants';
import Debug, { DebugThread } from "../core/Debug";

declare const Neutralino;
declare const NL_CWD;

type ProcessOptions = {
    /**
     * Environment variables
     */
    env?: object;

    /**
     * Current working directory for the running process
     */
    cwd?: string;

    /**
     * Interval between tries to find started process id
     * 
     * @default 50
     */
    childInterval?: number;
};

class Process
{
    /**
     * Process ID
     */
    public readonly id: number;

    /**
     * Interval in ms between process status update
     * 
     * null if you don't want to update process status
     * 
     * @default 200
     */
    public runningInterval: number|null = 200;

    /**
     * Interval in ms between process output update
     * 
     * null if you don't want to update process output
     * 
     * @default 500
     */
    public outputInterval: number|null = 500;

    protected outputFile: string|null;
    protected outputOffset: number = 0;

    protected _finished: boolean = false;

    /**
     * Whether the process was finished
     */
    public get finished(): boolean
    {
        return this._finished;
    };

    protected onOutput?: (output: string, process: Process) => void;
    protected onFinish?: (process: Process) => void;

    public constructor(pid: number, outputFile: string|null = null)
    {
        const debugThread = new DebugThread('Process/Stream', `Opened process ${pid} stream`);

        this.id = pid;
        this.outputFile = outputFile;

        const updateStatus = () => {
            this.running().then((running) => {
                // The process is still running
                if (running)
                {
                    if (this.runningInterval)
                        setTimeout(updateStatus, this.runningInterval);
                }

                // Otherwise the process was stopped
                else
                {
                    this._finished = true;

                    debugThread.log('Process stopped');

                    if (this.onFinish)
                        this.onFinish(this);
                }
            });
        };

        if (this.runningInterval)
            setTimeout(updateStatus, this.runningInterval);

        if (this.outputFile)
        {
            const updateOutput = () => {
                Neutralino.filesystem.readFile(this.outputFile)
                    .then((output: string) => {
                        if (this.onOutput)
                            this.onOutput(output.substring(this.outputOffset), this);

                        this.outputOffset = output.length;

                        if (this._finished)
                        {
                            if (output !== '')
                            {
                                debugThread.log({
                                    message: [
                                        'Process output:',
                                        ...output.split(/\r\n|\r|\n/)
                                    ]
                                });
                            }

                            Neutralino.filesystem.removeFile(this.outputFile);
                        }

                        else if (this.outputInterval)
                            setTimeout(updateOutput, this.outputInterval);
                    })
                    .catch(() => {
                        if (this.outputInterval && !this._finished)
                            setTimeout(updateOutput, this.outputInterval);
                    });
            };

            if (this.outputInterval)
                setTimeout(updateOutput, this.outputInterval);
        }
    }

    /**
     * Specify callback to run when the process will be finished
     */
    public finish(callback: (process: Process) => void)
    {
        this.onFinish = callback;

        if (this._finished)
            callback(this);

        // If user stopped process status auto-checking
        // then we should check it manually when this method was called
        else if (this.runningInterval === null)
        {
            this.running().then((running) => {
                if (!running)
                {
                    this._finished = true;

                    callback(this);
                }
            });
        }
    }

    public output(callback: (output: string, process: Process) => void)
    {
        this.onOutput = callback;
    }

    /**
     * Kill process
     */
    public kill(forced: boolean = false): Promise<void>
    {
        return Process.kill(this.id, forced);
    }

    /**
     * Returns whether the process is running
     * 
     * This method doesn't call onFinish event
     */
    public running(): Promise<boolean>
    {
        return new Promise((resolve) => {
            Neutralino.os.execCommand(`ps -p ${this.id} -S`).then((output) => {
                resolve(output.stdOut.includes(this.id) && !output.stdOut.includes('Z   '));
            });
        });
    }

    /**
     * Run shell command
     */
    public static run(command: string, options: ProcessOptions = {}): Promise<Process>
    {
        return new Promise(async (resolve) => {
            const tmpFile = `${await constants.paths.launcherDir}/${10000 + Math.round(Math.random() * 89999)}.tmp`;

            // Set env variables
            if (options.env)
            {
                Object.keys(options.env).forEach((key) => {
                    command = `${key}="${this.addSlashes(options.env![key].toString())}" ${command}`;
                });
            }

            // Set output redirection to the temp file
            command = `${command} > "${this.addSlashes(tmpFile)}" 2>&1`;

            // Set current working directory
            if (options.cwd)
                command = `cd "${this.addSlashes(options.cwd)}" && ${command}`;

            // And run the command
            const process = await Neutralino.os.execCommand(command, {
                background: true
            });

            const childFinder = async () => {
                const childProcess = await Neutralino.os.execCommand(`pgrep -P ${process.pid}`);

                // Child wasn't found
                if (childProcess.stdOut == '')
                    setTimeout(childFinder, options.childInterval ?? 50);

                // Otherwise return its id
                else
                {
                    const processId = parseInt(childProcess.stdOut.substring(0, childProcess.stdOut.length - 1));

                    Debug.log({
                        function: 'Process.run',
                        message: {
                            'running command': command,
                            'cwd': options.cwd,
                            'initial process id': process.pid,
                            'real process id': processId,
                            ...options.env
                        }
                    });
        
                    resolve(new Process(processId, tmpFile));
                }
            };

            setTimeout(childFinder, options.childInterval ?? 50);
        });
    }

    public static kill(id: number, forced: boolean = false): Promise<void>
    {
        return new Promise((resolve) => {
            Neutralino.os.execCommand(`kill ${forced ? '-9' : '-15'} ${id}`).then(() => resolve());
        });
    }

    /**
     * Replace '\a\b' to '\\a\\b'
     * And replace ''' to '\''
     */
    public static addSlashes(str: string): string
    {
        return str.replaceAll('\\', '\\\\').replaceAll('"', '\\"');
    }
}

export type { ProcessOptions };

export default Process;
