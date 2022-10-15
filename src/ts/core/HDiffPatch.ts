import { Debug } from '../../empathize';
import { path } from '../../empathize';


export default class HDiffPatch
{
    public static patch(file: string, patch: string, output: string): Promise<boolean>
    {
        return new Promise(async (resolve) => {
            let exeResult = await Neutralino.os.execCommand(`./public/hdiffpatch/hpatchz -f "${path.addSlashes(file)}" "${path.addSlashes(patch)}" "${path.addSlashes(output)}"`);

            const result = (exeResult.stdOut ?? exeResult.stdErr).includes('patch ok!');

            Debug.log({
                function: 'HDiffPatch.patch',
                message: { file, patch, output, result }
            });

            resolve(result);
        });
    }
};
