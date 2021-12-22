declare const Neutralino;

class Response
{
    /**
     * Requested url
     */
    public readonly url: string;

    /**
     * HTTP status code
     */
    public readonly status: number|null;

    /**
     * Content length
     */
    public readonly length: number|null;

    /**
     * Represents whether the response was successful (status in the range 200-299) or not
     */
    public readonly ok: boolean;

    public constructor(url: string, status: number|null, length: number|null)
    {
        this.url = url;
        this.status = status;
        this.length = length;

        // https://developer.mozilla.org/en-US/docs/Web/API/Response/ok
        this.ok = status >= 200 && status <= 299;
    }

    /**
     * Get request's body
     * 
     * @param delay maximal request delay in milliseconds
     */
    public body(delay: number|null = null): Promise<string>
    {
        return new Promise((resolve) => {
            Neutralino.os.execCommand(`curl -s -L ${delay !== null ? `-m ${(delay / 1000).toFixed(3)}` : ''} "${this.url}"`)
                .then((output) => resolve(output.stdOut));
        });
    }
}

/**
 * Fetch data from the URL
 * 
 * @param delay maximal request delay in milliseconds
 */
export default function fetch(url: string, delay: number|null = null): Promise<Response>
{
    return new Promise(async (resolve) => {
        let header = await Neutralino.os.execCommand(`curl -s -I -L ${delay !== null ? `-m ${(delay / 1000).toFixed(3)}` : ''} "${url}"`);

        if (header.stdOut == '')
            header = header.stdErr;

        else header = header.stdOut;

        header = header.split(/^HTTP\/[\d]+ /mi).pop();

        let status: any = /^([\d]+)[\s]+$/m.exec(header);
        let length: any = /^content-length: ([\d]+)/mi.exec(header);

        if (status !== null)
            status = parseInt(status[1]);

        if (length !== null)
            length = parseInt(length[1]);

        resolve(new Response(url, status, length));
    });
};

export { Response };
