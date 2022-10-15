import { path } from '../../empathize';

type Tag = {
    tag: string,
    commit: string
};


export default class Git
{
    /**
     * Get list of git repository tags
     * 
     * @param repository URI of the repository
     */
    public static getTags(repository: string): Promise<Tag[]>
    {
        return new Promise(async (resolve) => {
            const output = await Neutralino.os.execCommand(`git ls-remote --tags "${path.addSlashes(repository)}"`);

            let tags: Tag[] = [];

            output.stdOut.split(/\r\n|\r|\n/).forEach((line: string) => {
                if (line != '')
                {
                    const matches = /^([0-9a-f]+)\trefs\/tags\/(.*)/.exec (line);

                    if (matches)
                    {
                        tags.push({
                            tag: matches[2],
                            commit: matches[1]
                        });
                    }
                }
            });

            resolve(tags);
        });
    }
};
