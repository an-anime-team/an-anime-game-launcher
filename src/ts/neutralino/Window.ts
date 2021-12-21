type WindowSize = {
    width?: number;
    height?: number;
    minWidth?: number;
    minHeight?: number;
    maxWidth?: number;
    maxHeight?: number;
    resizable?: boolean;
};

type WindowOptions = WindowSize & {
    title?: string;
    icon?: string;
    fullScreen?: boolean;
    alwaysOnTop?: boolean;
    enableInspector?: boolean;
    borderless?: boolean;
    maximize?: boolean;
    hidden?: boolean;
    maximizable?: boolean;
    exitProcessOnClose?: boolean;
    processArgs?: string;
};

declare const Neutralino;

class Window
{
    public static get current(): any
    {
        return Neutralino.window;
    }

    public static async open(name: string, options: WindowOptions = {}): Promise<boolean>
    {
        return new Promise(async (resolve) => {
            const status = Neutralino.window.create(`/${name}.html`, {
                width: 600,
                height: 400,
                enableInspector: false,
                exitProcessOnClose: true,

                ...options,

                // So basically you should display the window manually
                // with Window.current.show() when your app will load
                // all its content there
                hidden: true
            });

            resolve(status !== undefined);
        });
    }
}

export type {
    WindowSize,
    WindowOptions
};

export default Window;
