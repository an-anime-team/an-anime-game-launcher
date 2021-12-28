import type { DomainInfo } from '../types/Domain';

import Process from '../neutralino/Process';
import { DebugThread } from './Debug';

export default class Domain
{
    public static getInfo(uri: string): Promise<DomainInfo>
    {
        const debugThread = new DebugThread('Domain.getInfo', `Getting info about uri: ${uri}`);

        return new Promise(async (resolve) => {
            const process = await Process.run(`ping -n 1 -w 1 -B ${uri}`);

            // If something will be wrong - at least we'll have
            // to wait 1.5 seconds instread of 2
            process.runningInterval = 500;
            process.outputInterval = 500;

            let output = '';

            process.output((outputPart) => {
                output += outputPart;

                const regex = /PING (.*) \(([0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3})\) from ([0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}) : [\d]+\([\d]+\) bytes of data/gm.exec(output);
                
                if (regex !== null)
                {
                    process.outputInterval = null;
                    process.runningInterval = null;
                    
                    const info: DomainInfo = {
                        uri: regex[1],
                        remoteIp: regex[2],
                        localIp: regex[3],
                        available: regex[2] !== regex[3]
                    };

                    debugThread.log({ message: info });

                    resolve(info);
                }
            });
        });
    }
};
