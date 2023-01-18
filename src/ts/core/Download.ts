import type {
    Latest,
    Diff,
    VoicePack,
} from '../types/GameData';

import { DebugThread } from '@empathize/framework/dist/meta/Debug';
import md5 from './md5';

declare const Neutralino;

/**
 * Return the latest file or the diff to be downloaded based on the current version.
 *
 * @param data the data of the file to be downloaded. Must include the data for
 *     the latest version and the diffs.
 * @param version the current version of the game. null if the game is not installed yet.
 *
 * @returns the latest version or the diff for the data to be downloaded.
 */
export function resolveDownloadTarget(data: {latest: Latest, diffs: Diff[]}, version: string | null): Latest | Diff
{
    if (version !== null)
    {
        for (const diff of data.diffs)
        {
            if (diff.version == version) {
                return diff;
            }
        }
    }
    return data.latest;
}

/**
 * Check if the target is downloaded as filePath.
 *
 * If the file exists but is corrupted (too big or MD5 mismatch), the file will be removed.
 *
 * @param target the latest file or the diff file to be downloaded.
 * @param filePath the path to download the target to.
 *
 * @returns true if the target is fully downloaded and verified. false if the
 * target needs to be downloaded or resumed.
 */
export async function isDownloaded(target: Latest | Diff | VoicePack, filePath: string): Promise<boolean>
{
    const debugThread = new DebugThread('Game.isDownloaded', 'Checking if the file is fully downloaded');
    try
    {
        const stats = await Neutralino.filesystem.getStats(filePath);
        if (stats.size == target.package_size)
        {
            const fileMd5 = await md5(filePath);
            if (fileMd5?.toLowerCase() == target.md5.toLowerCase())
            {
                debugThread.log(`The file is fully downloaded and the MD5 checksum matches.`);
                return true;
            }
            debugThread.log(`MD5 mismatchs for file ${filePath}. Expected ${target.md5}, got ${fileMd5}.`);
        }
        else if (stats.size < target.package_size)
        {
            debugThread.log(`File ${filePath} is incomplete. Expected size is ${target.package_size} bytes, got ${stats.size} bytes.`);
            return false;
        }
        else
        {
            debugThread.log(`File ${filePath} is too big. Expected size is ${target.package_size} bytes, got ${stats.size} bytes.`);
        }
        // If we reach here, the downloaded file is either bigger than the target, or the MD5 mismatches.
        // In either case, we should remove the existing file and re-download it.
        await Neutralino.filesystem.removeFile(filePath);
        return false;
    }
    catch (error)
    {
        debugThread.log(`Got error ${JSON.stringify(error)}. The file ${filePath} may not exist.`);
        return false;
    }
}
