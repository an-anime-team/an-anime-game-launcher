type callback = () => any;

type PromiseOptions = {
    callbacks: callback[];

    /**
     * If true, then all the callbacks will be called
     * at the same time and promisify will be resolved
     * when all of them will be finished
     * 
     * Otherwise, callbacks will be called one after the other
     * and promisify will be resolved with the last one
     */
    callAtOnce?: boolean;

    /**
     * [callAtOnce: true] updates interval in ms
     * 
     * @default 100
     */
    interval?: number;
};

/**
 * Make a promise from a function(s) and run it
 */
export default function promisify(callback: callback|PromiseOptions): Promise<any>
{
    return new Promise(async (resolve) => {
        if (typeof callback === 'function')
            resolve(await Promise.resolve(callback()));

        else
        {
            let outputs = {};

            if (callback.callAtOnce)
            {
                let remained = callback.callbacks.length;

                for (let i = 0; i < callback.callbacks.length; ++i)
                    promisify(callback.callbacks[i]).then((output) => {
                        outputs[i] = output;

                        --remained;
                    });

                const updater = () => {
                    if (remained > 0)
                        setTimeout(updater, callback.interval ?? 100);

                    else resolve(outputs);
                };

                setTimeout(updater, callback.interval ?? 100);
            }

            else for (let i = 0; i < callback.callbacks.length; ++i)
                outputs[i] = await promisify(callback.callbacks[i]());

            resolve(outputs);
        }
    });
};

export type {
    PromiseOptions,
    callback
};
