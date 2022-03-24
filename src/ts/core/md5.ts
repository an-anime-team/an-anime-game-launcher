import { fs, path } from '../../empathize';

declare const Neutralino;

export default function md5(file: string): Promise<string|null>
{
    return new Promise(async (resolve) => {
        if (await fs.exists(file))
        {
            const process = await Neutralino.os.execCommand(`md5sum "${path.addSlashes(file)}"`);

            resolve((process.stdOut || process.stdErr).split(' ')[0]);
        }

        else resolve(null);
    });
};
