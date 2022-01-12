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

type WindowOpenResult = {
    status: boolean;
    data?: {
        pid: number;
        stdOut: string;
        stdErr: string;
        exitCode: number;
    };
};

declare const Neutralino;

class Window
{
    public static get current(): any
    {
        return {
            ...Neutralino.window,

            center(windowWidth: number, windowHeight: number)
            {
                Neutralino.window.move(Math.round((window.screen.width - windowWidth) / 2), Math.round((window.screen.height - windowHeight) / 2));
            }
        };
    }

    public static open(name: string, options: WindowOptions = {}): Promise<WindowOpenResult>
    {
        return new Promise(async (resolve) => {
            const status = await Neutralino.window.create(`/${name}.html`, {
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

            resolve({
                status: status !== undefined,
                data: status
            });
        });
    }
}

export type {
    WindowSize,
    WindowOptions,
    WindowOpenResult
};

export default Window;
