import type { Params } from '../types/DiscordRPC';

import Process from '../neutralino/Process';

declare const NL_CWD;

export default class DiscordRPC
{
    protected params: Params;
    protected process?: Process;

    public constructor(params: Params)
    {
        this.params = params;

        let exec = [
            `${NL_CWD}/public/discord-rpc/discord-rpc`,
            `-a ${params.id}`
        ];

        if (params.details)
            exec = [...exec, `-d "${Process.addSlashes(params.details)}"`];

        if (params.state)
            exec = [...exec, `-s "${Process.addSlashes(params.state)}"`];

        if (params.icon)
        {
            if (params.icon.large)
                exec = [...exec, `-li "${params.icon.large}"`];

            if (params.icon.small)
                exec = [...exec, `-si "${params.icon.small}"`];
        }

        if (params.time)
        {
            if (params.time.start)
                exec = [...exec, `-st ${params.time.start}`];

            if (params.time.end)
                exec = [...exec, `-et ${params.time.end}`];
        }

        Process.run(exec.join(' ')).then((process) => this.process = process);
    }

    /**
     * Stop the discord rpc
     */
    public stop(forced: boolean = false): Promise<void>
    {
        return this.process!.kill(forced);
    }
};
