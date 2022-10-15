declare namespace Neutralino {
    type ErrorCode =
        | 'NE_FS_DIRCRER'
        | 'NE_FS_RMDIRER'
        | 'NE_FS_FILRDER'
        | 'NE_FS_FILWRER'
        | 'NE_FS_FILRMER'
        | 'NE_FS_NOPATHE'
        | 'NE_FS_COPYFER'
        | 'NE_FS_MOVEFER'
        | 'NE_OS_UNLTOUP'
        | 'NE_OS_INVMSGA'
        | 'NE_OS_INVKNPT'
        | 'NE_ST_INVSTKY'
        | 'NE_ST_STKEYWE'
        | 'NE_RT_INVTOKN'
        | 'NE_RT_NATPRME'
        | 'NE_RT_APIPRME'
        | 'NE_RT_NATRTER'
        | 'NE_RT_NATNTIM'
        | 'NE_CL_NSEROFF'
        | 'NE_EX_EXTNOTC'
        | 'NE_UP_CUPDMER'
        | 'NE_UP_CUPDERR'
        | 'NE_UP_UPDNOUF'
        | 'NE_UP_UPDINER';

    /**
     * If a particular native API call fails, Neutralinojs client library rejects the pending Promise with an error object.
     */
    interface Error {
        /**
         * Error code.
         */
        code: ErrorCode;
        /**
         * Error message.
         */
        message: string;
    }

    /**
     * `Neutralino.app` namespace contains methods related to the current application instance.
     */
    namespace app {
        interface RestartOptions {
            /**
             * Additional command-line arguments that need to be passed to the new application instance's process.
             */
            args: string;
        }

        /**
         * Terminates the running application.
         * @param exitCode Process's exit code. The default value is always `0` (success).
         * @example
         * await Neutralino.app.exit(130);
         *
         * await Neutralino.app.exit();
         */
        function exit(exitCode?: number): Promise<void>;

        /**
         * Kills the application process. If the application becomes unresponsive, you can use this to terminate the process instantly. It is recommended to use the `exit()` method to close your application properly.
         * @example
         * await Neutralino.app.killProcess();
         */
        function killProcess(): Promise<void>;

        /**
         * Restarts the current application instance.
         * @example
         * await Neutralino.app.restartProcess();
         *
         * await Neutralino.app.restartProcess({ args: '--restarted' });
         */
        function restartProcess(options?: RestartOptions): Promise<void>;

        /**
         * Returns the current application configuration as a JSON object.
         * @returns The current application configuration. Sometimes, this configuration object is not identical to your configuration file because the framework updates the configuration during several situations such as config overriding via CLI arguments and using `0` as the port.
         * @example
         * let config = await Neutralino.app.getConfig();
         * console.log('URL = ', config.url);
         */
        function getConfig(): Promise<any>;

        /**
         * Dispatches a new event to all app instances.
         * @param eventName Name of the event.
         * @param data Additional data for the event.
         * @example
         * await Neutralino.app.broadcast('myTestEvent', 'Hello');
         *
         * await Neutralino.app.broadcast('myTestEvent', {myData: 'Test data'});
         *
         * await Neutralino.app.broadcast('myTestEvent');
         */
        function broadcast<T>(eventName: string, data?: T): Promise<events.BroadcastResponse>;
    }

    /**
     * `Neutralino.clipboard` namespace offers functions to access system clipboard.
     */
    namespace clipboard {
        /**
         * Writes data into Neutralinojs shared storage.
         * @param text Text to store into the system clipboard.
         * @example
         * await Neutralino.clipboard.writeText('Test value');
         */
        function writeText(text: string): Promise<void>;

        /**
         * Reads and returns text from system clipboard.
         * @returns Stored text from the system clipboard.
         * @example
         * let clipboardText = await Neutralino.clipboard.readText();
         * console.log(`Text: ${clipboardText}`);
         */
        function readText(): Promise<string>;
    }

    /**
     * `Neutralino.computer` namespace contains methods related to the user's hardware.
     */
    namespace computer {
        interface MemoryInfo {
            /**
             * Total physical memory.
             */
            total: number;
            /**
             * Available physical memory.
             */
            available: number;
        }

        interface KernelInfo {
            /**
             * Kernel type: `Linux`, `Darwin`, `Windows NT`, or `Unknown`
             */
            variant: 'Linux' | 'Darwin' | 'Windows NT' | 'Unknown';
            /**
             * Version in the `<major>.<minor>.<patch>-<build_number>` format.
             */
            version: string;
        }

        interface OSInfo {
            /**
             * Operating system name.
             */
            name: string;
            /**
             * Operating system description.
             */
            description: string;
            /**
             * Version in the `<major>.<minor>.<patch>-<build_number>` format.
             */
            version: string;
        }

        interface CPUInfo {
            /**
             * Vendor name.
             */
            vendor: string;
            /**
             * Model name.
             */
            model: string;
            /**
             * The current CPU frequency in hertz (Hz).
             */
            frequency: number;
            /**
             * CPU architecture name. Returns the same value as the `getArch` function.
             */
            architecture: 'x64' | 'ia32' | 'arm' | 'itanium' | 'unknown';
            /**
             * Number of logical threads in the parallelism model.
             */
            logicalThreads: number;
            /**
             * Number of physical cores in the CPU.
             */
            physicalCores: number;
            /**
             * Number of physical CPU hardware units in the motherboard.
             */
            physicalUnits: number;
        }

        interface Display {
            /**
             * A virtual display identifier.
             */
            id: number;
            /**
             * Display resolution information
             */
            resolution: {
                /**
                 * Display width.
                 */
                width: number;
                /**
                 * Display height
                 */
                height: number;
            };
            /**
             * DPI (Dots Per Inch) value.
             */
            dpi: number;
            /**
             * BPP (Bits Per Pixel) value (also known as the color depth).
             */
            bpp: number;
            /**
             * Refresh rate in hertz (Hz).
             */
            refreshRate: number;
        }

        /**
         * Provides physical memory details (in megabytes).
         * @example
         * let memoryInfo = await Neutralino.computer.getMemoryInfo();
         * console.log(`Your ram size: ${Math.round(memoryInfo.total / 1000000)}GB`);
         */
        function getMemoryInfo(): Promise<MemoryInfo>;

        /**
         * Returns the CPU architecture identifier: `x64` (x86 64bit/arm64), `ia32` (x86 32bit), `arm`, `itanium`, or `unknown`.
         * @example
         * let arch = await Neutralino.computer.getArch();
         * console.log(arch);
         */
        function getArch(): Promise<'x64' | 'ia32' | 'arm' | 'itanium' | 'unknown'>;

        /**
         * Returns operating system kernel information.
         * @example
         * let kernelInfo = await Neutralino.computer.getKernelInfo();
         * console.log(`Kernel: ${kernelInfo.variant}`);
         */
        function getKernelInfo(): Promise<KernelInfo>;

        /**
         * Returns operating system information.
         * @example
         * let osInfo = await Neutralino.computer.getOSInfo();
         * console.log(`OS: ${kernelInfo.name}`);
         */
        function getOSInfo(): Promise<OSInfo>;

        /**
         * Returns the CPU information.
         * @example
         * let cpuInfo = await Neutralino.computer.getCPUInfo();
         * console.log(`CPU model: ${cpuInfo.model}`);
         */
        function getCPUInfo(): Promise<CPUInfo>;

        /**
         * Returns information about all connected displays.
         * @example
         * let displays = await Neutralino.computer.getDisplays();
         * for(let display of displays) {
         *  console.log(display);
         * }
         */
        function getDisplays(): Promise<Display[]>;
    }

    /**
     * `Neutralino.debug` namespace contains application debugging utilities.
     */
    namespace debug {
        /**
         * Writes messages to `neutralinojs.log` file or/and standard output streams.
         * @param message Content to be logged.
         * @param type Type of the message. Accepted values are `INFO`, `WARNING`, and `ERROR`. The default value is `INFO`.
         * @example
         * await Neutralino.debug.log('Hello Neutralinojs');
         *
         * await Neutralino.debug.log('An error occured', 'ERROR');
         *
         * await Neutralino.debug.log('A warning message', 'WARNING');
         */
        function log(message: string, type?: 'INFO' | 'WARNING' | 'ERROR'): Promise<void>;
    }

    /**
     * `Neutralino.events` namespace contains methods related to the native events handling. These events are often initiated by the Neutralinojs server based on native state changes.
     */
    namespace events {
        type Handler<T> = (event?: CustomEvent<T>) => void;
        interface Response {
            success: boolean;
            message: string;
        }

        type BroadcastResponse = Omit<Response, 'message'>;

        /**
         * Registers a new event handler.
         * @param eventName Name of the event.
         * @param handler A function that will be called when the given event occurs. Neutralinojs will call the handler with a [CustomEvent](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent) instance by attaching additional data to the `detail` key.
         * @example
         * function onTrayMenuItemClicked(event) {
         *   console.log(`Event data: ${event.detail}`);
         * }
         * await Neutralino.events.on('trayMenuItemClicked', onTrayMenuItemClicked);
         */
        function on(eventName: 'ready', handler: Handler<null>): Promise<Response>;
        function on(eventName: 'trayMenuItemClicked', handler: Handler<Neutralino.os.TrayMenuItem>): Promise<Response>;
        function on(eventName: 'windowClose', handler: Handler<null>): Promise<Response>;
        function on(eventName: 'serverOffline', handler: Handler<null>): Promise<Response>;
        function on(eventName: 'clientConnect', handler: Handler<number>): Promise<Response>;
        function on(eventName: 'clientDisconnect', handler: Handler<number>): Promise<Response>;
        function on(eventName: 'appClientConnect', handler: Handler<number>): Promise<Response>;
        function on(eventName: 'appClientDisconnect', handler: Handler<number>): Promise<Response>;
        function on(eventName: 'extClientConnect', handler: Handler<string>): Promise<Response>;
        function on(eventName: 'extClientDisconnect', handler: Handler<string>): Promise<Response>;
        function on(eventName: 'extensionReady', handler: Handler<string>): Promise<Response>;
        function on(eventName: 'spawnedProcess', handler: Handler<Neutralino.os.SpawnProcessResult>): Promise<Response>;
        function on<T>(eventName: string, handler: Handler<T>): Promise<Response>;

        /**
         * Unregisters an event handler.
         * @param eventName Name of the event.
         * @param handler A function reference.
         * @example
         * await Neutralino.events.off('trayMenuItemClicked', onTrayMenuItemClicked);
         */
        function off(eventName: 'ready', handler: Handler<null>): Promise<Response>;
        function off(eventName: 'trayMenuItemClicked', handler: Handler<Neutralino.os.TrayMenuItem>): Promise<Response>;
        function off(eventName: 'windowClose', handler: Handler<null>): Promise<Response>;
        function off(eventName: 'serverOffline', handler: Handler<null>): Promise<Response>;
        function off(eventName: 'clientConnect', handler: Handler<number>): Promise<Response>;
        function off(eventName: 'clientDisconnect', handler: Handler<number>): Promise<Response>;
        function off(eventName: 'appClientConnect', handler: Handler<number>): Promise<Response>;
        function off(eventName: 'appClientDisconnect', handler: Handler<number>): Promise<Response>;
        function off(eventName: 'extClientConnect', handler: Handler<string>): Promise<Response>;
        function off(eventName: 'extClientDisconnect', handler: Handler<string>): Promise<Response>;
        function off(eventName: 'extensionReady', handler: Handler<string>): Promise<Response>;
        function off(eventName: 'spawnedProcess', handler: Handler<Neutralino.os.SpawnProcessResult>): Promise<Response>;
        function off<T>(eventName: string, handler: Handler<T>): Promise<Response>;

        /**
         * Dispatches a new event to the current app instance. Neutralinojs client uses this JavaScript function call internally to dispatch native events.
         * @param event Name of the event.
         * @param data Additional data for the event.
         * @example
         * await Neutralino.events.dispatch('myTestEvent', {myData: 'Test data'});
         */
        function dispatch(event: 'ready', data: null): Promise<Response>;
        function dispatch(event: 'trayMenuItemClicked', data: Neutralino.os.TrayMenuItem): Promise<Response>;
        function dispatch(event: 'windowClose', data: null): Promise<Response>;
        function dispatch(event: 'serverOffline', data: null): Promise<Response>;
        function dispatch(event: 'clientConnect', data: number): Promise<Response>;
        function dispatch(event: 'clientDisconnect', data: number): Promise<Response>;
        function dispatch(event: 'appClientConnect', data: number): Promise<Response>;
        function dispatch(event: 'appClientDisconnect', data: number): Promise<Response>;
        function dispatch(event: 'extClientConnect', data: string): Promise<Response>;
        function dispatch(event: 'extClientDisconnect', data: string): Promise<Response>;
        function dispatch(event: 'extensionReady', data: string): Promise<Response>;
        function dispatch(event: 'spawnedProcess', data: Neutralino.os.SpawnProcessResult): Promise<Response>;
        function dispatch<T>(event: string, data?: T): Promise<Response>;

        /**
         * Dispatches a new event to all clients (both app and extension clients).
         * @param event Name of the event.
         * @param data Additional data for the event.
         * @example
         * await Neutralino.events.broadcast('myTestEvent', 'Hello');
         *
         * await Neutralino.events.broadcast('myTestEvent', {myData: 'Test data'});
         *
         * await Neutralino.events.broadcast('myTestEvent'); // without any data payload
         */
        function broadcast(event: 'ready', data: null): Promise<void>;
        function broadcast(event: 'trayMenuItemClicked', data: Neutralino.os.TrayMenuItem): Promise<void>;
        function broadcast(event: 'windowClose', data: null): Promise<void>;
        function broadcast(event: 'serverOffline', data: null): Promise<void>;
        function broadcast(event: 'clientConnect', data: number): Promise<void>;
        function broadcast(event: 'clientDisconnect', data: number): Promise<void>;
        function broadcast(event: 'appClientConnect', data: number): Promise<void>;
        function broadcast(event: 'appClientDisconnect', data: number): Promise<void>;
        function broadcast(event: 'extClientConnect', data: string): Promise<void>;
        function broadcast(event: 'extClientDisconnect', data: string): Promise<void>;
        function broadcast(event: 'extensionReady', data: string): Promise<void>;
        function broadcast(event: 'spawnedProcess', data: Neutralino.os.SpawnProcessResult): Promise<Response>;
        function broadcast<T>(event: string, data?: T): Promise<BroadcastResponse>;
    }

    /**
     * `Neutralino.extensions` namespace contains methods related to Neutralino extensions. Extensions let developers write custom backend APIs for Neutralinojs applications.
     *
     * Learn more about extensions with [this guide](https://neutralino.js.org/docs/how-to/extensions-overview).
     */
    namespace extensions {
        interface ExtensionStats {
            /**
             * An array of loaded extensions.
             */
            loaded: string[];
            /**
             * An array of connected extensions. These extensions have an active WebSocket-based IPC connection with the main process.
             */
            connected: string[];
        }

        /**
         * Dispatches a new event to an extension instance. If the targeted extension is not connected yet, Neutralino client library will queue the function call and send whenever the extension comes online.
         * @param extensionId Extension identifier.
         * @param eventName Name of the event.
         * @param data Additional data for the event.
         * @example
         * await Neutralino.extensions.dispatch('js.neutralino.sampleextension',
         *             'myTestEvent', {myData: 'Test data'});
         * await Neutralino.extensions.dispatch('js.neutralino.sampleextension',
         *             'myTestEvent');
         */
        function dispatch(extensionId: string, eventName: string, data?: any): Promise<void>;

        /**
         * Dispatches a new event to all connected extensions. If an extension is loaded but not connected yet, the particular extension won't get the new event. Use [extensions.dispatch](https://neutralino.js.org/docs/api/extensions#extensionsdispatchextensionid-eventname-data) to send messages even if the extension is not connected to the main process.
         * @param eventName Name of the event.
         * @param data Additional data for the event.
         * @example
         * await Neutralino.extensions.broadcast('myTestEvent', 'Hello');
         *
         * await Neutralino.extensions.broadcast('myTestEvent', {myData: 'Test data'});
         *
         * await Neutralino.extensions.broadcast('myTestEvent');
         */
        function broadcast(eventName: string, data?: any): Promise<void>;

        /**
         * Returns details about connected and loaded extensions.
         * @example
         * let stats = await Neutralino.extensions.getStats();
         * console.log('stats: ', stats);
         */
        function getStats(): Promise<ExtensionStats>;
    }
    /**
     * `Neutralino.filesystem` namespace contains methods for handling files.
     */
    namespace filesystem {
        interface DirectoryEntry {
            /**
             * file name.
             */
            entry: string;
            /**
             * The type of the entry (`FILE` or `DIRECTORY`).
             */
            type: 'FILE' | 'DIRECTORY';
        }

        interface Stats {
            /**
             * Size in bytes.
             */
            size: number;
            /**
             * `true` if the path represents a normal file.
             */
            isFile: boolean;
            /**
             * `true` if the path represents a directory.
             */
            isDirectory: boolean;
        }

        /**
         * Creates a new directory. Throws `NE_FS_DIRCRER` if directory creation is not possible.
         * @param path New directory path.
         * @example
         * await Neutralino.filesystem.createDirectory('./newDirectory');
         *
         * await Neutralino.filesystem.createDirectory(NL_PATH + '/myFolder');
         */
        function createDirectory(path: string): Promise<void>;

        /**
         * Removes a given directory. Throws `NE_FS_RMDIRER` if the removal is not possible.
         * @param path Directory path.
         * @example
         * await Neutralino.filesystem.removeDirectory('./tmpDirectory');
         */
        function removeDirectory(path: string): Promise<void>;

        /**
         * Writes a text file. Throws `NE_FS_FILWRER` for file write errors.
         * @param filename Filename.
         * @param data Content of the file.
         * @example
         * await Neutralino.filesystem.writeFile('./myFile.txt', 'Sample content');
         */
        function writeFile(filename: string, data: string): Promise<void>;

        /**
         * Appends text content to file. Throws `NE_FS_FILWRER` for file write errors. If the provided file doesn't exist, this function creates a new file with `data`.
         * @param filename Filename.
         * @param data Content to append.
         * @example
         * await Neutralino.filesystem.appendFile('./myFile.txt', 'Sample ');
         * await Neutralino.filesystem.appendFile('./myFile.txt', 'content');
         */
        function appendFile(filename: string, data: string): Promise<void>;

        /**
         * Writes a binary file. Throws `NE_FS_FILWRER` for file write errors.
         * @param filename Filename.
         * @param data Content of the binary file as an [ArrayBuffer](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer).
         * @example
         * let rawBin = new ArrayBuffer(1);
         * let view = new Uint8Array(rawBin);
         * view[0] = 64; // Saves ASCII '@' to the binary file
         *
         * await Neutralino.filesystem.writeBinaryFile('./myFile.bin', rawBin);
         */
        function writeBinaryFile(filename: string, data: ArrayBuffer): Promise<void>;

        /**
         * Appends binary data to a file. Throws `NE_FS_FILWRER` for file write errors. If the provided file doesn't exist, this function creates a new file with `data`.
         * @param filename Filename.
         * @param data Binary content to append as an [ArrayBuffer](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer).
         * @example
         * let rawBin = new ArrayBuffer(1);
         * let view = new Uint8Array(rawBin);
         * view[0] = 64; // Saves ASCII '@' to the binary file
         *
         * await Neutralino.filesystem.appendBinaryFile('./myFile.bin', rawBin);
         * await Neutralino.filesystem.appendBinaryFile('./myFile.bin', rawBin);
         */
        function appendBinaryFile(filename: string, data: ArrayBuffer): Promise<void>;

        /**
         * Reads a text file. Throws `NE_FS_FILRDER` for file read errors.
         * @param filename Filename.
         * @returns File content.
         * @example
         * let data = await Neutralino.filesystem.readFile('./myFile.txt');
         * console.log(`Content: ${data}`);
         */
        function readFile(filename: string): Promise<string>;

        /**
         * Reads binary files. Throws `NE_FS_FILRDER` for file read errors.
         * @param filename Filename.
         * @returns Content of the binary file as an ArrayBuffer.
         * @example
         * let data = await Neutralino.filesystem.readBinaryFile({
         *   fileName: './myFile.bin'
         * });
         * let view = new Uint8Array(data);
         *
         * console.log('Binary content: ', view);
         */
        function readBinaryFile(filename: string): Promise<ArrayBuffer>;

        /**
         * Removes given file. Throws `NE_FS_FILRMER` for file removal errors.
         * @param filename Filename.
         * @example
         * await Neutralino.filesystem.removeFile('./myFile.txt');
         */
        function removeFile(filename: string): Promise<void>;

        /**
         * Reads directory contents. Throws `NE_FS_NOPATHE` if the path doesn't exist.
         * @param path File/directory path.
         * @example
         * let entries = await Neutralino.filesystem.readDirectory(NL_PATH);
         * console.log('Content: ', entries);
         */
        function readDirectory(path: string): Promise<DirectoryEntry[]>;

        /**
         * Copies a file to a new destination. Throws `NE_FS_COPYFER` if the system cannot copy the file.
         * @param source Source path.
         * @param destination Destination path.
         * @example
         * await Neutralino.filesystem.copyFile('./source.txt', './destination.txt');
         */
        function copyFile(source: string, destination: string): Promise<void>;

        /**
         * Moves a file to a new destination. Throws `NE_FS_MOVEFER` if the system cannot move the file.
         * @param source Source path.
         * @param destination Destination path.
         * @example
         * await Neutralino.filesystem.moveFile('./source.txt', './destination.txt');
         */
        function moveFile(source: string, destination: string): Promise<void>;

        /**
         * Returns file statistics for the given path. If the given path doesn't exist or is inaccessible,`NE_FS_NOPATHE` is thrown. Therefore, you can use this method to check for the existance of a file or directory.
         * @param path File or directory path.
         * @example
         * let stats = await Neutralino.filesystem.getStats('./sampleVideo.mp4');
         * console.log('Stats:', stats);
         */
        function getStats(path: string): Promise<Stats>;
    }

    /**
     * `init` is not a namespace, it's a function that initializes a Neutralinojs application.
     *
     * The application developer needs to call this method explicitly via a JavaScript source file before using any native API function. The `init` function does the following tasks when it's called.
     * - Starts a WebSocket connection with the Neutralinojs server asynchronously.
     * - Registers auto-reload event handler if the `--neu-dev-auto-reload` flag (the `neu run` command sets this flag) is provided.
     * - Defines `NL_CVERSION` with the client libary version in the window scope.
     *
     * You can call native API calls right after the `init` function call, as shown below.
     * @example
     * Neutralino.init();
     *
     * Neutralino.os.showMessageBox('Welcome', 'Hello Neutralinojs');
     *
     * @description Also, you can wrap immediate native calls with the `ready` event callback if you like.
     *
     * @example
     * Neutralino.init();
     *
     * Neutralino.events.on('ready', () => {
     *     Neutralino.os.showMessageBox('Welcome', 'Hello Neutralinojs');
     * });
     */
    function init(): void;

    /**
     * `Neutralino.os` namespace contains methods related to the user's operating system.
     */
    namespace os {
        interface ExecCommandOptions {
            /**
             * Executes the command in background and resolve the Promise immediately if this is set to `true`.
             */
            background?: boolean;
            /**
             * Standard input as a string.
             */
            stdIn?: string;
        }

        interface ExecCommandResult {
            /**
             * Process identifier.
             */
            pid: number;
            /**
             * Standard output.
             */
            stdOut: string;
            /**
             * Standard error.
             */
            stdErr: string;
            /**
             * Exit code of the process.
             */
            exitCode: number;
        }

        interface SpawnProcessResult {
            /**
             * A Neutralino-scoped process identifier. This value is used for controlling the process via the native API.
             */
            id: number;
            /**
             * Process identifier from the operating system.
             */
            pid: number;
        }

        interface Filter {
            /**
             * Filter name.
             */
            name: string;
            /**
             * Array of file extensions. Eg: `['jpg', 'png']`
             */
            extensions: string[];
        }

        interface OpenDialogOptions {
            /**
             * An array of Filter objects to filter the files list.
             */
            filter?: Filter[];
            /**
             * Enables multi selections.
             */
            multiSelections?: boolean;
            /**
             * Initial path/filename displayed by the dialog.
             */
            defaultPath?: string;
        }

        interface SaveDialogOptions {
            /**
             * An array of Filter objects to filter the files list.
             */
            filter?: Filter[];
            /**
             * Skips file overwrite warning message.
             */
            forceOverwrite?: boolean;
            /**
             * Initial path/filename displayed by the dialog.
             */
            defaultPath?: string;
        }

        interface FolderDialogOptions {
            /**
             * Initial path displayed by the dialog.
             */
            defaultPath?: string;
        }

        type Icon = 'INFO' | 'WARNING' | 'ERROR' | 'QUESTION';

        interface TrayMenuItem {
            /**
             *  A unique identifier for each menu item.
             */
            id?: string;
            /**
             * Label of the menu item. This field is a mandatory field. Use `-` (hyphen) character for a menu separator.
             */
            text: string;
            /**
             *  A boolean flag to disable/enable a specific menu item.
             */
            isDisabled?: boolean;
            /**
             * A boolean flag to mark a specific menu item as selected.
             */
            isChecked?: boolean;
        }

        interface TrayOptions {
            /**
             * Tray icon path. Eg: `/resources/icons/trayIcon.png`. A 20x20-sized PNG image file works fine on all supported operating systems.
             */
            icon?: string;
            /**
             * An array of `TrayMenuItem` objects.
             */
            menuItems?: TrayMenuItem[];
        }

        /**
         * Executes a command and returns the output.
         * @param command The command that is to be executed.
         * @example
         * let info = await Neutralino.os.execCommand('python --version');
         * console.log(`Your Python version: ${info.stdOut}`);
         *
         * await Neutralino.os.execCommand('npm start', { background: true });
         */
        function execCommand(command: string, options?: ExecCommandOptions): Promise<ExecCommandResult>;

        /**
         * Spawns a process based on a command in background and let developers control it.
         * @param command The command that is to be executed in a new process.
         * @example
         * let pingProc = await Neutralino.os.spawnProcess('ping neutralino.js.org');
         * 
         * Neutralino.events.on('spawnedProcess', (evt) => {
         *      if(pingProc.id == evt.detail.id) {
         *          switch(evt.detail.action) {
         *              case 'stdOut':
         *                  console.log(evt.detail.data);
         *                  break;
         *              case 'stdErr':
         *                  console.error(evt.detail.data);
         *                  break;
         *              case 'exit':
         *                  console.log(`Ping process terminated with exit code: ${evt.detail.data}`);
         *                  break;
         *          }
         *      }
         * });
         */
        function spawnProcess(command: string): Promise<SpawnProcessResult>;

        /**
         * Updates a spawned process based on a provided action and data. Throws `NE_OS_UNLTOUP` if the process cannot be updated.
         * @param id Neutralino-scoped process identifier.
         * @param action An action to take. Accepts only the following values: stdIn, stdInEnd, and exit.
         * @param data Additional data for the `action`. Send stardard input string if the `action` is `stdIn`.
         * @example
         * let nodeProc = await Neutralino.os.spawnProcess('node');
         * 
         * Neutralino.events.on('spawnedProcess', (evt) => {
         *     if(nodeProc.id == evt.detail.id) {
         *         switch(evt.detail.action) {
         *             case 'stdOut':
         *                 console.log(evt.detail.data); // 10
         *                 break;
         *             case 'stdErr':
         *                 console.error(evt.detail.data);
         *                 break;
         *             case 'exit':
         *                 console.log(`Node.js process terminated with exit code: ${evt.detail.data}`);
         *                 break;
         *         }
         *     }
         * });
         * 
         * await Neutralino.os.updateSpawnedProcess(nodeProc.id, 'stdIn', 'console.log(5 + 5);');
         * await Neutralino.os.updateSpawnedProcess(nodeProc.id, 'stdInEnd')
         */
        function updateSpawnedProcess(id: number, action: 'stdIn' | 'stdInEnd' | 'exit', data?: object): Promise<void>;

        /**
         * Returns all spawned processes.
         * @example
         * await Neutralino.os.spawnProcess('ping neutralino.js.org');
         * await Neutralino.os.spawnProcess('ping codezri.org');
         * 
         * let processes = await Neutralino.getSpawnedProcesses();
         * console.log(processes);
         */
        function getSpawnedProcesses(): Promise<SpawnProcessResult[]>;

        /**
         * Provides the value of a given environment variable.
         * @param key The name of the environment variable.
         * @returns Value of the given environment variable. Returns an empty string if the environment variable is not defined.
         * @example
         * let value = await Neutralino.os.getEnv('USER');
         * console.log(`USER = ${value}`);
         */
        function getEnv(key: string): Promise<string>;

        /**
         * Shows the file open dialog.
         * @param title Title of the dialog.
         * @returns An array of selected entries.
         * @example
         * let entries = await Neutralino.os.showOpenDialog('Save your diagram', {
         *   defaultPath: '/home/my/directory/',
         *   filters: [
         *     {name: 'Images', extensions: ['jpg', 'png']},
         *     {name: 'All files', extensions: ['*']}
         *   ]
         * });
         * console.log('You have selected:', entries);
         */
        function showOpenDialog(title?: string, options?: OpenDialogOptions): Promise<string[]>;

        /**
         * Shows the file open dialog.
         * @param title Title of the dialog.
         * @returns Selected filename.
         * @example
         * let entry = await Neutralino.os.showSaveDialog('Open a file', {
         *   defaultPath: 'untitled.jpg',
         *   filters: [
         *     {name: 'Images', extensions: ['jpg', 'png']},
         *     {name: 'All files', extensions: ['*']}
         *   ]
         * });
         * console.log('You have selected:', entry);
         */
        function showSaveDialog(title?: string, options?: SaveDialogOptions): Promise<string>;

        /**
         * Shows the folder open dialog.
         * @param title Title of the dialog.
         * @returns Selected folder.
         * @example
         * let entry = await Neutralino.os.showFolderDialog('Select installation directory', {
         *   defaultPath: '/home/my/directory/'
         * });
         * console.log('You have selected:', entry);
         */
        function showFolderDialog(title?: string, options?: FolderDialogOptions): Promise<string>;

        /**
         * Displays a notification message.
         * @param title Notification title.
         * @param content Content of the notification.
         * @param icon Icon name. Accepted values are: `INFO`, `WARNING`, `ERROR`, and `QUESTION`. The default value is `INFO`.
         * @example
         * await Neutralino.os.showNotification('Hello world', 'It works!. Have a nice day');
         *
         * await Neutralino.os.showNotification('Oops :/', 'Something went wrong', 'ERROR');
         */
        function showNotification(title: string, content: string, icon?: Icon): Promise<void>;

        /**
         * Displays a message box.
         * @param title Title of the message box.
         * @param content Content of the message box.
         * @param choice Message box buttons. Accepted values are: `OK`, `OK_CANCEL`, `YES_NO`, `YES_NO_CANCEL`, `RETRY_CANCEL`, and `ABORT_RETRY_IGNORE`. The default value is `OK`.
         * @param icon Icon name. Accepted values are: `INFO`, `WARNING`, `ERROR`, and `QUESTION`. The default value is `INFO`.
         * @returns User's `choice`.
         * @example
         * await Neutralino.os.showMessageBox('Hello', 'Welcome');
         *
         * let button = await Neutralino.os
         *             .showMessageBox('Confirm',
         *                             'Are you sure you want to quit?',
         *                             'YES_NO', 'QUESTION');
         * if (button == 'YES') {
         *     Neutralino.app.exit();
         * }
         */
        function showMessageBox(
            title: string,
            content: string,
            choice?: 'OK' | 'OK_CANCEL' | 'YES_NO' | 'YES_NO_CANCEL' | 'RETRY_CANCEL' | 'ABORT_RETRY_IGNORE',
            icon?: Icon
        ): Promise<string>;

        /**
         * Creates/updates the tray icon and menu.
         * @example
         * let tray = {
         *   icon: '/resources/icons/trayIcon.png',
         *   menuItems: [
         *     {id: "about", text: "About"},
         *     {text: "-"},
         *     {id: "quit", text: "Quit"}
         *   ]
         * };
         *
         * await Neutralino.os.setTray(tray);
         */
        function setTray(options: TrayOptions): Promise<void>;

        /**
         * Returns known platform-specific folders such as Downloads, Music, Videos, etc.
         * @param name Name of the folder. Accepted values are: `config`, `data`, `cache`, `documents`, `pictures`, `music`, `video`, `downloads`, `savedGames1`, and `savedGames2`. Throws `NE_OS_INVKNPT` for invalid folder names.
         * @returns Path.
         * @example
         * let downloadsPath = await Neutralino.os.getPath('downloads');
         * console.log(`Downloads folder: ${downloadsPath}`);
         */
        function getPath(
            name:
                | 'config'
                | 'data'
                | 'cache'
                | 'documents'
                | 'pictures'
                | 'music'
                | 'video'
                | 'downloads'
                | 'savedGames1'
                | 'savedGames2'
        ): Promise<string>;

        /**
         * Opens a URL with the default web browser.
         *
         * If your application is running in the default web browser, this method will open a new tab.
         * @param url URL to be opened.
         * @example
         * Neutralino.os.open('https://neutralino.js.org');
         */
        function open(url: string): Promise<void>;
    }
    /**
     * Neutralinojs has a built-in shared key-value storage. It's like a global [LocalStorage](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage) for all Neutralinojs modes. `Neutralinos.storage` exposes methods for interacting with this storage feature.
     */
    namespace storage {
        /**
         * Writes data into Neutralinojs shared storage.
         * @param key A unique identifier.
         * @param data  Data as a string. If this value is `null` or `undefined`, the specific data record will be erased from the disk.
         * @example
         * await Neutralino.storage.setData('userDetails',
         *                         JSON.stringify({ username: 'TestValue'})
         * );
         */
        function setData(key: string, data: string): Promise<void>;

        /**
         * Reads and returns data for a given Neutralinojs shared storage key.
         * @param key Storage data record identifier.
         * @returns Data string of the storage record.
         * @example
         * let data = await Neutralino.storage.getData('userDetails');
         * console.log(`Data: ${data}`);
         */
        function getData(key: string): Promise<string>;
    }
    /**
     * `Neutralino.updater` namespace contains methods related to built-in automatic updater. Neutralinojs offers a built-in client-based updating mechanism. Therefore, you can update Neutralinojs apps without even calling third-party update services, operating system level services, or other binaries/scripts.
     *
     * Learn more about extensions with this [guide](https://neutralino.js.org/docs/how-to/auto-updater).
     */
    namespace updater {
        interface Manifest {
            applicationId: string;
            version: string;
            resourcesURL: string;
        }

        /**
         * Checks latest updates from the given URL. The URL should return a valid Neutralinojs update manifest with `Content-Type: application/json header`. Throws `NE_UP_CUPDMER` for invalid manifests and `NE_UP_CUPDERR` for network connectivity issues.
         * @param url URL to fetch update manifest.
         * @returns Update manifest.
         * @example
         * let url = 'https://example.com/updates/manifest.json';
         * let manifest = await Neutralino.updater.checkForUpdates(url);
         */
        function checkForUpdates(url: string): Promise<Manifest>;

        /**
         * Installs updates from the downloaded update manifest. Throws `NE_UP_UPDNOUF` if the manifest isn't loaded. If the update installation process fails, this function will throw `NE_UP_UPDINER`.
         * @example
         * let url = 'https://example.com/updates/manifest.json';
         * let manifest = await Neutralino.updater.checkForUpdates(url);
         *
         * if (manifest.version != NL_APPVERSION) {
         *     await Neutralino.updater.install();
         * }
         * else {
         *     console.log('You are using the latest version!');
         * }
         */
        function install(): Promise<void>;
    }
    /**
     * The `Neutralino.window` namespace contains methods related to the current native window instance. This namespace's methods will work only for the [window](https://neutralino.js.org/docs/configuration/modes#window) mode.
     */
    namespace window {
        interface WindowSizeOptions {
            /**
             * Window width in pixels.
             */
            width?: number;
            /**
             * Window height in pixels.
             */
            height?: number;
            /**
             * Minimum width of the window in pixels.
             */
            minWidth?: number;
            /**
             * Minimum height of the window in pixels.
             */
            minHeight?: number;
            /**
             * Maximum width of the window in pixels.
             */
            maxWidth?: number;
            /**
             * Maximum height of the window in pixels.
             */
            maxHeight?: number;
            /**
             * A boolean value to make the window resizable or fixed.
             */
            resizable?: boolean;
        }

        interface WindowPosOptions {
            /**
             * Horizontal coordinate of the left edge of the window.
             */
            x: number;
            /**
             * Vertical coordinate of the top edge of the window.
             */
            y: number;
        }

        interface WindowOptions extends WindowSizeOptions {
            /**
             * Window title.
             */
            title?: string;
            /**
             * Window icon path.
             */
            icon?: string;
            /**
             * Sets full screen mode.
             */
            fullScreen?: boolean;
            /**
             * Activates the top-most mode.
             */
            alwaysOnTop?: boolean;
            /**
             * Activates developer tools and opens the web inspector window.
             */
            enableInspector?: boolean;
            /**
             * Makes the window borderless.
             */
            borderless?: boolean;
            /**
             * Launches the window maximized.
             */
            maximize?: boolean;
            /**
             * Hides the window.
             */
            hidden?: boolean;
            /**
             * Makes the window maximizable or not.
             */
            maximizable?: boolean;
            /**
             *  Exits the application process when the user clicks the window's close button.
             */
            exitProcessOnClose?: boolean;
            /**
             * Additional command-line arguments for the new window process.
             */
            processArgs?: string;
        }

        interface ProcessInfo {
            /**
             * Process identifier.
             */
            pid: number;
            /**
             * Standard output. This value is always empty since the new window process starts asynchronously.
             */
            stdOut: number;
            /**
             * Standard error. This value is always empty since the new window process starts asynchronously.
             */
            stdErr: number;
            /**
             * Exit code of the process.
             */
            exitCode: boolean;
        }

        /**
         * Sets the title of the native window.
         * @param title Title of the window. Clears the title, if the function was called without the parameter.
         * @example
         * await Neutralino.window.setTitle('New title');
         */
        function setTitle(title: string): Promise<void>;

        /**
         * Returns the title of the native window.
         * @returns The current title of the native window instance.
         * @example
         * let title = await Neutralino.window.getTitle();
         * console.log(`title = ${title}`);
         */
        function getTitle(): Promise<string>;

        /**
         * Minimizes the native window.
         * @example
         * await Neutralino.window.minimize();
         */
        function minimize(): Promise<void>;

        /**
         * Maximizes the native window.
         * @example
         * await Neutralino.window.maximize();
         */
        function maximize(): Promise<void>;

        /**
         * Restores the native window.
         * @example
         * await Neutralino.window.unmaximize();
         */
        function unmaximize(): Promise<void>;

        /**
         * Returns `true` if the native window is maximized.
         * @returns `true` or `false` based on current maximized status.
         * @example
         * let status = await Neutralino.window.isMaximized();
         */
        function isMaximized(): Promise<boolean>;

        /**
         * Enables the full screen mode.
         * @example
         * await Neutralino.window.setFullScreen();
         */
        function setFullScreen(): Promise<void>;

        /**
         * Exits from the full screen mode.
         * @example
         * await Neutralino.window.exitFullScreen();
         */
        function exitFullScreen(): Promise<void>;

        /**
         * Returns `true` if the native window is in the full screen mode.
         * @returns `true` or `false` based on current full screen status.
         * @example
         * let status = await Neutralino.window.isFullScreen();
         */
        function isFullScreen(): Promise<boolean>;

        /**
         * Shows the native window.
         * @example
         * await Neutralino.window.show();
         */
        function show(): Promise<void>;

        /**
         * Hides the native window.
         * @example
         * await Neutralino.window.hide();
         */
        function hide(): Promise<void>;

        /**
         * Returns `true` if the native window is visible.
         * @returns `true` or `false` based on current visibility status.
         * @example
         * let status = await Neutralino.window.isVisible();
         */
        function isVisible(): Promise<boolean>;

        /**
         * Focuses the native window.
         * @example
         * await Neutralino.window.focus();
         */
        function focus(): Promise<void>;

        /**
         * Activates or deactivates the always on top mode.
         * @param onTop Says whether the on top mode should be activated or not. The default value is `true`.
         * @example
         * await Neutralino.window.setAlwaysOnTop(true); // or setAlwaysOnTop();
         * await Neutralino.window.setAlwaysOnTop(false);
         */
        function setAlwaysOnTop(onTop: boolean): Promise<void>;

        /**
         * Moves the native window into given coordinates. Neutralinojs's cross-platform coordinate system starts from top-left corner of the screen. In other words, `x=0,y=0` point refers to the top-left corner of the device's main screen.
         * @param x A integer value for the horizontal position.
         * @param y A integer value for the vertical position.
         * @example
         * await Neutralino.window.move(200, 400);
         */
        function move(x: number, y: number): Promise<void>;

        /**
         * Sets an icon for the native window or Dock.
         * @param icon A `200x200` PNG image file works fine on all supported operating systems.
         * @example
         * const icon = '/resources/icons/appIcon.png';
         * await Neutralino.window.setIcon(icon);
         */
        function setIcon(icon: string): Promise<void>;

        /**
         * Converts a given DOM element to a draggable region. The user will be able to drag the native window by dragging the given DOM element. This feature is suitable to make custom window bars along with the [borderless mode](https://neutralino.js.org/docs/configuration/neutralino.config.json#modeswindowborderless-boolean).
         * @param domId A DOM element identifier.
         * @example
         * await Neutralino.window.setDraggableRegion('myCustomTitleBar');
         */
        function setDraggableRegion(domId: string | HTMLElement): Promise<void>;

        /**
         * Converts a draggable region to a normal DOM elements by removing drag event handlers.
         * @param domId A DOM element identifier.
         * @example
         * await Neutralino.window.unsetDraggableRegion('myCustomTitleBar');
         */
        function unsetDraggableRegion(domId: string | HTMLElement): Promise<void>;

        /**
         * This method sets the size of the window.
         *
         * This method always expects width and height couples. For example, if you are setting `minWidth`, you should set `minHeight` too.
         * @example
         * await Neutralino.window.setSize({
         *     width: 500,
         *     height: 200,
         *     maxWidth: 600,
         *     maxHeight: 400
         * });
         *
         * await Neutralino.window.setSize({
         *     resizable: false
         * });
         */
        function setSize(options: WindowSizeOptions): Promise<void>;

        /**
         * Returns window size information.
         * @example
         * let sizeInfo = await Neutralino.window.getSize();
         *
         * console.log(sizeInfo);
         */
        function getSize(): Promise<WindowOptions>;

        /**
         * Returns window position coordinates.
         * @example
         * let position = await Neutralino.window.getPosition();
         *
         * console.log(position);
         */
        function getPosition(): Promise<WindowPosOptions>;

        /**
         * Creates a native window. You can use this method to create new window for your multi-window Neutralinojs app. Neutralinojs spawns a new process for each native window. Therefore, the new window works as an isolated app once the window is created.
         *
         * However, you can build communication streams between windows with the [storage API](https://neutralino.js.org/docs/api/storage).
         * @param url URL to be loaded. Eg: `/resources/aboutWindow.html`.
         * @param options an instance of `WindowOption` type.
         * @example
         * await Neutralino.window.create('/resources/aboutWindow.html', {
         *     icon: '/resources/icons/aboutIcon.png',
         *     enableInspector: false,
         *     width: 500,
         *     height: 300,
         *     maximizable: false,
         *     exitProcessOnClose: true,
         *     processArgs: '--window-id=W_ABOUT'
         * });
         */
        function create(url: string, options?: WindowOptions): Promise<ProcessInfo>;
    }
}

// https://neutralino.js.org/docs/api/global-variables
/**
 * Operating system name: `Linux`, `Windows`, or `Darwin`
 */
declare var NL_OS: 'Linux' | 'Windows' | 'Darwin';

/**
 * Application identifier
 */
declare var NL_APPID: string;

/**
 * Application version
 */
declare var NL_APPVERSION: string;

/**
 * 	Application port
 */
declare var NL_PORT: number;

/**
 * Mode of the application: `window`, `browser, `cloud`, or `chrome`
 */
declare var NL_MODE: 'window' | 'browser' | 'cloud' | 'chrome';

/**
 * Neutralinojs server version
 */
declare var NL_VERSION: string;

/**
 * Neutralinojs client version
 */
declare var NL_CVERSION: string;

/**
 * Current working directory
 */
declare var NL_CWD: string;

/**
 * Application path
 */
declare var NL_PATH: string;

/**
 * Command-line arguments
 */
declare var NL_ARGS: string[];

/**
 * Identifier of the current process
 */
declare var NL_PID: string;

/**
 * Source of application resources: `bundle` or `directory`
 */
declare var NL_RESMODE: 'bundle' | 'directory';

/**
 * Returns `true` if extensions are enabled
 */
declare var NL_EXTENABLED: boolean;

/**
 * Framework binary's release commit hash
 */
declare var NL_COMMIT: string;

/**
 * 	Client library's release commit hash
 */
declare var NL_CCOMMIT: string;

/**
 * Augment this interface to expose global variables from your app
 */
interface DefaultCustomGlobals {}

type NeutralinoCustomGlobals<T = DefaultCustomGlobals> = {
    readonly [K in keyof T as `NL_${K & string}`]: T[K];
};

interface Window extends NeutralinoCustomGlobals {}

// interface globalThis extends NeutralinoCustomGlobals {}
