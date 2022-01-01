import constants from '../Constants';

declare const Neutralino;

class IPCRecord
{
    public readonly id: number;
    public readonly time: number;
    public readonly data: any;

    public constructor(id: number, time: number, data: any)
    {
        this.id = id;
        this.time = time;
        this.data = data;
    }

    /**
     * Remove the record from the storage
     */
    public pop(): IPCRecord
    {
        IPC.remove(this);

        return this;
    }
}

export default class IPC
{
    /**
     * Read records from the "shared inter-process storage"
     */
    public static read(): Promise<IPCRecord[]>
    {
        return new Promise(async (resolve) => {
            Neutralino.filesystem.readFile(`${await constants.paths.launcherDir}/.ipc.json`)
                .then((data) => resolve(JSON.parse(data)))
                .catch(() => resolve([]));
        });
    }

    /**
     * Write some data to the "shared inter-process storage"
     */
    public static write(data: any): Promise<void>
    {
        return new Promise(async (resolve) => {
            const records = await this.read();

            records.push({
                id: Math.round(Math.random() * 100000),
                time: Date.now(),
                data: data
            } as IPCRecord);

            await Neutralino.filesystem.writeFile(`${await constants.paths.launcherDir}/.ipc.json`, JSON.stringify(records));

            resolve();
        });
    }

    /**
     * Remove record from the "shared inter-process storage"
     */
    public static remove(record: IPCRecord): Promise<void>
    {
        return new Promise(async (resolve) => {
            let records = await this.read();

            records = records.filter((item) => item.id !== record.id && item.time !== record.time);

            await Neutralino.filesystem.writeFile(`${await constants.paths.launcherDir}/.ipc.json`, JSON.stringify(records));

            resolve();
        });
    }
};

export { IPCRecord };
