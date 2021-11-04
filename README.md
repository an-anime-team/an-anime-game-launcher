<img src="repository-pics/logo.jpg">

<img src="repository-pics/launcher-main.png">

<img src="repository-pics/launcher-settings.png">

<br><br>

# Status

| Game version | Launcher version | Patch version |
| :---: | :---: | :---: |
| 2.2.0 | 1.3.0 | 2.2.0 stable ✅ |

Download from [Releases](https://notabug.org/nobody/an-anime-game-launcher/releases)

<br>

### ⚠️ Please, be careful with 2.3.0 game's release. It may suddenly break game's patch state

### ⚠️ Also patch's repository will be changed and you will HAVE to update launcher to the newest version

<br>

# Requirements

To work this launcher requires

* wine
* winetricks
* unzip
* tar
* git

## Install

### apt-get

```sh
sudo apt-get install wine winetricks unzip tar git
```

### pacman

```sh
sudo pacman -Syu wine winetricks unzip tar git
```

# Development

## Build from source

```sh
npm run build
```

## Run from source

```sh
npm start
```

# Roadmap

### ✓ <s>To 1.0.0 release</s>

* <s>Fix AppImage builds</s> *(0.3.0)*
* <s>Parse background banners from the game's API ([issue #1](https://notabug.org/nobody/an-anime-game-launcher/issues/1), [pull request #2](https://notabug.org/nobody/an-anime-game-launcher/pulls/2))</s> *(0.3.0)*
* <s>Update launcher logo</s> *(0.4.0)*
* <s>Cache launcher background picture ([pull request #6](https://notabug.org/nobody/an-anime-game-launcher/pulls/6))</s> *(0.4.0)*
* <s>Make Proton-GE default compatibility tool and fix game input issues</s> (added runners manager) *(0.5.0)*
* <s>Add preferences menu</s> *(0.5.0)*
* <s>Add additional telemetry checking</s> *(0.6.0)*
* <s>Add DXVK downloading in settings</s> *(0.6.0)*
* <s>Make participation in the anonymous analytics request</s> *(0.8.0)*
* <s>Add launcher updates notifications</s> *(1.0.0-rc1)*
* <s>Make automatic patch state parsing</s> *(1.0.0-rc1)*

### ➤ To 2.0.0 release

* <s>Add runners environmental variables manager</s> *(1.1.0)*
* <s>Add outdated files deletion when new game's update releases</s> *(1.1.0)*
* <s>Add installed packages deletion</s> *(1.2.0)*
* <s>Add voice packs support</s> (Thank @Maroxy for the developments in the previous versions) *(1.3.0)*
* Screenshots explorer
* Set default wine version to download so the wine install requirement is no longer needed.
* Add Patch category in settings menu with
  - Always participate in patches testing
  - Applying anti login crash patch
  - Remove patch
* Playing statistics

And don't forget to change the patch's URI when it will be changed

<br>

### Please, try to use "An Anime Game" phrase instead of the real game name to avoid search engines parsing