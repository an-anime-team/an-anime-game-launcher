declare const Neutralino;

type ProcessOptions = {
    input?: string;
    env?: object;
    cwd?: string;
};

class Process
{
    public readonly id: number;

    /**
     * Interval between process status update
     */
    public interval: number = 200;

    protected _finished: boolean = false;

    /**
     * Whether the process was finished
     */
    public get finished(): boolean
    {
        return this._finished;
    };

    protected onFinish?: (process: Process) => void;

    public constructor(pid: number)
    {
        this.id = pid;

        const updateStatus = async () => {
            Neutralino.os.execCommand(`ps -p ${this.id}`).then((output) => {
                // The process is still running
                if (output.stdOut.includes(this.id))
                    setTimeout(updateStatus, this.interval);

                // Otherwise the process was stopped
                else
                {
                    this._finished = true;

                    if (this.onFinish)
                        this.onFinish(this);
                }
            });
        };

        setTimeout(updateStatus, this.interval);
    }

    /**
     * Specify callback to run when the process will be finished
     */
    public finish(callback: (process: Process) => void)
    {
        this.onFinish = callback;

        if (this._finished)
            callback(this);
    }

    /**
     * Run shell command
     */
    public static run(command: string, options: ProcessOptions = {}): Promise<Process>
    {
        // Replace '\a\b' to '\\a\\b'
        // And replace ''' to '\''
        const addSlashes = (str: string) => str.replaceAll('\\', '\\\\').replaceAll('\'', '\\\'');

        return new Promise(async (resolve) => {
            // Set env variables
            if (options.env)
            {
                Object.keys(options.env).forEach((key) => {
                    command = `${key}='${addSlashes(options.env![key])}' ${command}`;
                });
            }

            // Set current working directory
            if (options.cwd)
                command = `cd '${addSlashes(options.cwd)}' && ${command} && cd -`;

            // And run the command
            const process = await Neutralino.os.execCommand(command, {
                background: true,
                stdin: options.input ?? ''
            });

            resolve(new Process(process.pid));
        });
    }
}

export type { ProcessOptions };

export default Process;
