# Electron Blank Application

This is the blank application for your electron project

## Requirements

To use Electron you must have installed node js. For windows it is available on its [official website](https://nodejs.org/en/), for linux you can download the `node` packet from your packet manager

## Installation

```sh
git clone https://github.com/krypt0nn/electron-blank-app ./my-app
cd my-app
npm i
```

`git clone` will download a blank bundle for your project and save it in the `./my-app` directory *(`my-app` folder in the current opened in the console directory)*

`cd my-app` will move you to this downloaded bundle

`npm i` will install requirements

## Set up

In the `package.json` you should change these parameters:

* name - your project name
* version
* description
* keywords - your project keywords
* author

## Development

All your development processes will be inside the `src` directory. The default page is `src/html/index.html`

To run your application - use `npm start` command

To build it for any systems - `npm run build:all`

* For Windows only: `npm run build:win`
* For Linux only: `npm run build:linux`
* For MacOS only: `npm run build:darwin`

All the binaries will appear in the `dist` directory in a folder with name `[app name]-[platform]-[arch]`, for example `electron-blank-app-linux-x64`

To pack linux binary to the flatpak binary you can run `npm run pack:flatpak`. This operation requires pre-installed `flatpak` and `flatpak-build` packages

<br>

Author: [Nikita Podvirnyy](https://vk.com/technomindlp)