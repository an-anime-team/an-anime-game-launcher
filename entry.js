const { app, BrowserWindow } = require('electron');
const path = require('path');

function createWindow ()
{
    // https://www.electronjs.org/docs/latest/api/browser-window/#class-browserwindow
    const mainWindow = new BrowserWindow ({
        width: 800,
        height: 600,
        webPreferences: {
            // Is not safety
            // Use it to have access to the node modules inside html files
            nodeIntegration: true,
            contextIsolation: false
        },
        icon: path.join(__dirname, 'public', 'images', 'icon64.png'),
        // autoHideMenuBar: true,
        // resizable: false
    });

    mainWindow.loadFile(path.join(__dirname, 'public', 'html', 'index.html'));

    // mainWindow.webContents.openDevTools();
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
});

// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.

app.on('window-all-closed', () => {
    if (process.platform !== 'darwin')
        app.quit();
});
