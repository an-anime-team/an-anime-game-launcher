const {
    app,
    BrowserWindow,
    ipcMain,
    Notification,
    shell,
    nativeImage,
    nativeTheme,
    dialog
} = require('electron');

const path = require('path');

require('electron-store').initRenderer();

let mainWindow, analyticsWindow, settingsWindow, splashWindow;

ipcMain.handle('hide-window', () => mainWindow.hide());
ipcMain.handle('show-window', () => mainWindow.show());

ipcMain.on('notification', (event, args) => {
    args.icon = nativeImage.createFromPath(args.icon ?? path.join(__dirname, 'public', 'images', 'baal64-transparent.png'));

    new Notification(args).show();
});

ipcMain.on('is-window-dark', (e) => e.returnValue = nativeTheme.shouldUseDarkColors);

ipcMain.handle('open-settings', () => {
    settingsWindow = new BrowserWindow ({
        width: 900,
        height: 600,
        webPreferences: {
            nodeIntegration: true,
            contextIsolation: false
        },
        icon: path.join(__dirname, 'public', 'images', 'icons', '64x64.png'),
        autoHideMenuBar: true,
        resizable: false,
        parent: mainWindow,
        modal: true,
        show: false
    });

    settingsWindow.loadFile(path.join(__dirname, 'public', 'html', 'settings.html'));
    settingsWindow.once('ready-to-show', settingsWindow.show);
});

ipcMain.handle('open-analytics-participation', () => {
    analyticsWindow = new BrowserWindow ({
        width: 700,
        height: 500,
        webPreferences: {
            nodeIntegration: true,
            contextIsolation: false
        },
        icon: path.join(__dirname, 'public', 'images', 'icons', '64x64.png'),
        autoHideMenuBar: true,
        resizable: false,
        parent: mainWindow,
        modal: true,
        show: false
    });

    analyticsWindow.loadFile(path.join(__dirname, 'public', 'html', 'analytics.html'));
    analyticsWindow.once('ready-to-show', analyticsWindow.show);
});

ipcMain.handle('hide-analytics-participation', () => analyticsWindow.close());

// https://www.electronjs.org/docs/latest/api/browser-window/#class-browserwindow
function createWindow ()
{
    // Launcher
    mainWindow = new BrowserWindow ({
        width: 1280,
        height: 700,
        webPreferences: {
            nodeIntegration: true,
            contextIsolation: false
        },
        icon: path.join(__dirname, 'public', 'images', 'icons', '64x64.png'),
        autoHideMenuBar: true,
        resizable: false,
        show: false
    });

    mainWindow.loadFile(path.join(__dirname, 'public', 'html', 'index.html'));

    // open URLs in Browser instead of an pop-up in electron app.
    mainWindow.webContents.setWindowOpenHandler(({ url }) => {
        shell.openExternal(url);

        return { action: 'deny' };
    });
    
    // mainWindow.webContents.openDevTools();

    // Splash
    splashWindow = new BrowserWindow({
        width: 250, 
        height: 320, 
        transparent: true, 
        frame: false,
        icon: path.join(__dirname, 'public', 'images', 'icons', '64x64.png'),
        autoHideMenuBar: true
    });

    splashWindow.loadFile(path.join(__dirname, 'public', 'splash', 'index.html'));
    splashWindow.center();
}


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

    ipcMain.handle('loaded', () => {
        setTimeout(() => {
            splashWindow.close();
            mainWindow.show();
        }, 1000);
    });

    // This has to be here otherwise webContents is invalid
    ipcMain.on('change-lang', (event, args) => {
        mainWindow.webContents.send('change-lang', { 'lang': args.lang });
    });

    ipcMain.on('change-voicepack', () => {
        mainWindow.webContents.send('change-voicepack');
    });

    ipcMain.on('prefix-select', async () => {
        const result = await dialog.showOpenDialog({
            properties: ['openDirectory']
        });

        if (result.filePaths.length > 0)
        {
            mainWindow.webContents.send('change-prefix', {
                'type': 'change',
                'dir': result.filePaths[0]
            });
        }
    });

    ipcMain.on('prefix-reset', async () => {
        mainWindow.webContents.send('change-prefix', {
            'type': 'reset'
        });
    });

    ipcMain.on('prefix-changed', async () => {
        settingsWindow.webContents.send('prefix-changed');
    });

    ipcMain.on('rpc-toggle', () => mainWindow.webContents.send('rpc-toggle'));
});

// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.

app.on('window-all-closed', () => {
    if (process.platform !== 'darwin')
        app.quit();
});
