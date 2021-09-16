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

Directory `src` stores your [TypeScript](https://typescriptlang.org) and [SASS](https://sass-lang.com) code. When you run `npm run dev` command in console - they will compile to the js and css files inside `public` directory

In the `public` stored information about your application - images it uses, html pages and something you want to use

Default application page is `public/html/index.html`

To run your application - use `npm start` command. It will automatically run `npm run dev`

To build application for any systems - `npm run build:all`

* For Windows only: `npm run build:win`
* For Linux only: `npm run build:linux`
* For MacOS only: `npm run build:darwin`

All binaries will appear in the `dist` directory in a folder with name `[app name]-[platform]-[arch]`, for example `electron-blank-app-linux-x64`

To pack linux binary to the flatpak binary you can run `npm run pack:flatpak`. This operation requires pre-installed `flatpak` and `flatpak-build` packages

<br>

Author: [Nikita Podvirnyy](https://vk.com/technomindlp)