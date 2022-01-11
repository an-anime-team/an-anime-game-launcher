import type { DomainInfo } from '../types/Domain';

import Process from '../neutralino/Process';
import { DebugThread } from './Debug';

declare const Neutralino;

export default class Domain
{
    public static getInfo(uri: string): Promise<DomainInfo>
    {
        const debugThread = new DebugThread('Domain.getInfo', `Getting info about uri: ${uri}`);

        return new Promise(async (resolve) => {
            const process = await Neutralino.os.execCommand(`ping -n -4 -w 1 -B "${Process.addSlashes(uri)}"`);
            const output = process.stdOut || process.stdErr;

            const resolveInfo = (info: DomainInfo) => {
                debugThread.log({ message: info });

                resolve(info);
            };

            if (output.includes('Name or service not known'))
            {
                resolveInfo({
                    uri: uri,
                    available: false
                });
            }

            else
            {
                const regex = /PING (.*) \(([0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3})\) .* ([0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}) : [\d]+\([\d]+\)/gm.exec(output);

                if (regex !== null)
                {
                    resolveInfo({
                        uri: regex[1],
                        remoteIp: regex[2],
                        localIp: regex[3],
                        available: regex[2] !== regex[3]
                    });
                }
            }
        });
    }
};
