import type {
    DXVKTable, 
    DXVK as TDXVK
} from './types/DXVK';

import constants from './Constants';
import AbstractInstaller from './AbstractInstaller';

declare const Neutralino;

class Stream extends AbstractInstaller
{
    public constructor(dxvk: TDXVK)
    {
        super(dxvk.uri, constants.paths.dxvksDir);
    }
}

export default class DXVK
{
    /**
     * Get DXVKs list
     */
    public static get(): Promise<DXVKTable[]>
    {
        return new Promise((resolve) => {
            constants.paths.dxvksDir.then(async (dxvksDir: string) => {
                let list: DXVKTable[] = JSON.parse(await Neutralino.filesystem.readFile(`${constants.paths.appDir}/public/dxvks.json`));

                const installed: { entry: string, type: string }[] = await Neutralino.filesystem.readDirectory(dxvksDir);

                let dxvks: DXVKTable[] = [];

                list.forEach((family) => {
                    let newFamily: DXVKTable = {
                        title: family.title,
                        versions: []
                    };

                    family.versions.forEach((dxvk) => {
                        let inst = false;

                        for (let dir of installed)
                            inst ||= dir.entry == `dxvk-${dxvk.version}`;

                        newFamily.versions.push({
                            ...dxvk,

                            installed: inst
                        });
                    });

                    dxvks.push(newFamily);
                });

                resolve(dxvks);
            });
        });
    }

    /**
     * Download DXVK to the [constants.paths.dxvks] directory
     * 
     * @param dxvk DXVK object or version
     * @returns null if the DXVK with specified version dosen't exist. Otherwise - installation stream
     */
    public static download(dxvk: TDXVK|TDXVK['version']): Promise<null|Stream>
    {
        return new Promise(async (resolve) => {
            // If we provided dxvk parameter with a version of DXVK
            // then we should find this DXVK version and call this method for it
            if (typeof dxvk == 'string')
            {
                let foundDXVK = null;

                (await this.get()).forEach((family) => {
                    family.versions.forEach((DXVK) => {
                        if (DXVK.version == dxvk)
                            foundDXVK = DXVK;
                    });
                });

                resolve(foundDXVK === null ? null : new Stream(foundDXVK));
            }

            // Otherwise we can use dxvk.uri and so on to download DXVK
            else resolve(new Stream(dxvk));
        });
    }
}

export type { TDXVK, DXVKTable };
