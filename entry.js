const {
    app,
    BrowserWindow,
    ipcMain,
    Notification,
    shell,
    nativeImage
} = require('electron');

const path = require('path');

let mainWindow;

ipcMain.handle('hide-window', () => mainWindow.hide());
ipcMain.handle('show-window', () => mainWindow.show());

ipcMain.on('notification', (event, args) => {
    if (args.icon !== undefined)
        args.icon = nativeImage.createFromPath(args.icon);

    new Notification(args).show();
});

ipcMain.handle('open-settings', () => {
    const settingsWindow = new BrowserWindow({
        width: 900,
        height: 600,
        webPreferences: {
            nodeIntegration: true,
            contextIsolation: false
        },
        icon: path.join(__dirname, 'public', 'images', 'icon64.png'),
        autoHideMenuBar: true,
        resizable: false,
        parent: mainWindow,
        modal: true,
        show: false
    });

    settingsWindow.loadFile(path.join(__dirname, 'public', 'html', 'settings.html'));
    settingsWindow.once('ready-to-show', () => settingsWindow.show());
});

function createWindow ()
{
    // https://www.electronjs.org/docs/latest/api/browser-window/#class-browserwindow
    mainWindow = new BrowserWindow ({
        width: 1280,
        height: 700,
        webPreferences: {
            nodeIntegration: true,
            contextIsolation: false
        },
        icon: path.join(__dirname, 'public', 'images', 'icon64.png'),
        autoHideMenuBar: true,
        resizable: false
    });

    mainWindow.loadFile(path.join(__dirname, 'public', 'html', 'index.html'));

    // open URLs in Browser instead of an pop-up in electron app.
    mainWindow.webContents.setWindowOpenHandler(({ url }) => {
        shell.openExternal(url);

        return { action: 'deny' };
    });

    // mainWindow.webContents.openDevTools();
}

// Set language on start
if(Genshinlib.getConfig().lang.launcher)
    app.commandLine.appendSwitch('lang', Genshinlib.getConfig().lang.launcher);
else
    app.commandLine.appendSwitch('lang', 'en-us');

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.

app.whenReady().then(() => {
    createWindow();

    app.on('activate', () => {
        // On macOS it's common to re-create a window in the app when the
        // dock icon is clicked and there are no other windows open.
        if (BrowserWindow.getAllWindows().length === 0)
            createWindow();
    });

    // This has to be here otherwise webContents is invalid.
    ipcMain.on('changelang', (event, args) => {
        app.commandLine.appendSwitch('lang', Genshinlib.getConfig().lang.launcher);
        mainWindow.webContents.send('changelang', { 'lang': args.lang });
    });

    ipcMain.on('updateVP', (event, args) => {
        mainWindow.webContents.send('updateVP', { 'oldvp': args.oldvp });
    });

    ipcMain.on('rpcstate', (event, args) => {
        mainWindow.webContents.send('rpcstate', {});
    });
});

// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.

app.on('window-all-closed', () => {
    if (process.platform !== 'darwin')
        app.quit();
});
