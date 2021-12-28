import type { DebugOptions, LogRecord } from '../types/Debug';

class DebugThread
{
    protected thread: number;
    protected funcName: string|null;

    public constructor(funcName: string|null = null, options: DebugOptions|string|null = null)
    {
        // Generate some random thread id
        this.thread = 1000 + Math.round(Math.random() * 8999);

        this.funcName = funcName;

        if (options !== null)
            this.log(options);
    }

    public log(options: DebugOptions|string)
    {
        Debug.log({
            thread: this.thread,
            function: this.funcName ?? '',

            ...(typeof options === 'string' ? { message: options } : options)
        });
    }
}

class Debug
{
    protected static logOutput: LogRecord[] = [];
    
    protected static formatTime(time: number): string
    {
        const prefixTime = (time: number): string => {
            return time < 10 ? `0${time}` : time.toString();
        };

        const date = new Date(time);

        return `${prefixTime(date.getHours())}:${prefixTime(date.getMinutes())}:${prefixTime(date.getSeconds())}.${date.getMilliseconds()}`;
    }

    public static log(options: DebugOptions|string)
    {
        const time = Date.now();

        let output: LogRecord = {
            time: time,
            log: [
                `[${this.formatTime(time)}]`
            ]
        };

        if (typeof options === 'string')
            output.log[0] += ` ${options}`;

        else
        {
            // Add thread id
            if (options.thread)
                output.log[0] += `[thread: ${options.thread}]`;

            // Add function name
            if (options.function)
                output.log[0] += `[${options.function}]`;

            // Add log message if it is a single line
            if (typeof options.message === 'string')
                output.log[0] += ` ${options.message}`;

            // message: [a, b, c, d]
            else if (Array.isArray(options.message))
                options.message.forEach((line) => {
                    if (line !== '')
                        output.log.push(` - ${line}`);
                });

            // message: { a: b, c: d }
            else Object.keys(options.message).forEach((key) => {
                output.log.push(` - [${key}] ${options.message[key]}`);
            });
        }

        this.logOutput.push(output);
    }

    public static merge(records: LogRecord[])
    {
        this.logOutput.unshift(...records);
        this.logOutput.sort((a, b) => a.time - b.time);
    }

    public static getRecords(): LogRecord[]
    {
        return this.logOutput;
    }

    public static get(): string[]
    {
        let output: string[] = [];

        this.logOutput.forEach((record) => {
            record.log.forEach((line) => output.push(line));
        });

        return output;
    }
}

export default Debug;

export { DebugThread };

export type {
    DebugOptions,
    LogRecord
};
