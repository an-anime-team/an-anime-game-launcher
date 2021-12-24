/**
 * Make a promise from a synchronous function and run it
 */
export default function promisify(callback: () => any): Promise<any>
{
    return new Promise((resolve) => {
        resolve(callback());
    });
};
