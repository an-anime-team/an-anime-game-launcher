import * as dbus from 'dbus-next';
const bus = dbus.systemBus();

let switcherooProxy: dbus.ProxyObject,
    switcherooRunning: boolean;

bus.getProxyObject('net.hadess.SwitcherooControl', '/net/hadess/SwitcherooControl').then(proxy => {
    switcherooProxy = proxy;
    switcherooRunning = true;
}, () => switcherooRunning = false);

interface GPU {
    Default: dbus.Variant<boolean>,
    Environment: dbus.Variant<string[]>,
    Name: dbus.Variant<string>
}

export default class SwitcherooControl {
    public static waitReady() {
        const poll = (resolve: () => void, reject: () => void) => {
            if (switcherooRunning === false) reject();
            if (switcherooProxy) resolve();
            else setTimeout(poll, 100, resolve, reject);
        };

        return new Promise<void>(poll)
    }

    public static getGpus() {
        if (!switcherooRunning) return null;

        const properties = switcherooProxy.getInterface("org.freedesktop.DBus.Properties");
        const gpus: Promise<dbus.Variant<GPU[]>> = properties.Get('net.hadess.SwitcherooControl', 'GPUs');
        return gpus;
    }

    public static async getGpuByName(name: string) {
        const gpus = await SwitcherooControl.getGpus();
        if (!gpus) return null;

        return gpus.value.find(gpu => gpu.Name.value === name);
    }

    //switcheroo-control returns env vars as an array like this: ["NAME1", "VALUE1", "NAME2", "VALUE2"], but we want them as an object
    public static getEnvAsObject(gpu: GPU) {
        const env: any = {};

        for (let i = 0; i < gpu.Environment.value.length; i += 2) {
            env[gpu.Environment.value[i]] = gpu.Environment.value[i + 1];
        }

        return env;
    }
}