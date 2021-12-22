import type {
    PatchState,
    PatchInfo
} from './types/Patch';

import md5 from 'js-md5';

import constants from './Constants';

declare const Neutralino;

export default class Patch
{
    /*public static get latest(): Promise<PatchInfo>
    {
        return new Promise((resolve) => {
            
        });
    }

    public static getPatchInfo(version: string, source: 'origin' | 'additional' = 'origin'): Promise<PatchInfo|null>
    {
        return new Promise(async (resolve) => {
            const patchUri = constants.uri.patch[source];

            const patchSh = await fetch(`${patchUri}/raw/master/${version.replaceAll('.', '')}/patch.sh`);
        });
    }*/
}
