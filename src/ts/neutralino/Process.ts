declare const Neutralino;

type ProcessOptions = {
    input?: string;
    env?: object;
    cwd?: string;
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
    public interval: number|null;

    protected _finished: boolean = false;

    /**
     * Whether the process was finished
     */
    public get finished(): boolean
    {
        return this._finished;
    };

    protected onFinish?: (process: Process) => void;

    public constructor(pid: number, interval: number|null = 200)
    {
        this.id = pid;
        this.interval = interval;

        const updateStatus = () => {
            this.running().then((running) => {
                // The process is still running
                if (running)
                {
                    if (this.interval)
                        setTimeout(updateStatus, this.interval);
                }

                // Otherwise the process was stopped
                else
                {
                    this._finished = true;

                    if (this.onFinish)
                        this.onFinish(this);
                }
            });
        };

        if (this.interval)
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

        // If user stopped process status auto-checking
        // then we should check it manually when this method was called
        else if (this.interval === null)
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

    /**
     * Kill process
     */
    public kill(forced: boolean = false): Promise<void>
    {
        return new Promise((resolve) => {
            Neutralino.os.execCommand(`kill ${forced ? '-9' : '-15'} ${this.id}`).then(() => resolve());
        });
    }

    /**
     * Returns whether the process is running
     * 
     * This method doesn't call onFinish event
     */
    public running(): Promise<boolean>
    {
        return new Promise((resolve) => {
            Neutralino.os.execCommand(`ps -p ${this.id}`).then((output) => {
                resolve(output.stdOut.includes(this.id));
            });
        });
    }

    /**
     * Run shell command
     */
    public static run(command: string, options: ProcessOptions = {}): Promise<Process>
    {
        
        return new Promise(async (resolve) => {
            // Set env variables
            if (options.env)
            {
                Object.keys(options.env).forEach((key) => {
                    command = `${key}='${this.addSlashes(options.env![key])}' ${command}`;
                });
            }

            // Set current working directory
            if (options.cwd)
                command = `cd '${this.addSlashes(options.cwd)}' && ${command} && cd -`;

            // And run the command
            const process = await Neutralino.os.execCommand(command, {
                background: true,
                stdin: options.input ?? ''
            });

            resolve(new Process(process.pid));
        });
    }

    /**
     * Replace '\a\b' to '\\a\\b'
     * And replace ''' to '\''
     */
    public static addSlashes(str: string): string
    {
        return str.replaceAll('\\', '\\\\').replaceAll('\'', '\\\'');
    }
}

export type { ProcessOptions };

export default Process;
