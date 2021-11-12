<img src="repository-pics/logo.jpg">

<img src="repository-pics/launcher-main.png">

<img src="repository-pics/launcher-settings.png">

<br><br>

### Please, try to use "An Anime Game" phrase instead of the real game name to avoid search engines parsing

<br><br>

# Status

| Game version | Launcher version | Patch version |
| :---: | :---: | :---: |
| 2.2.0 | 1.5.0| 2.2.0 stable ✅ |

> ⚠️ New patch's version will be delayed for a week because of some author's personal reasons

### AppImage

Download AppImage from [Releases](https://notabug.org/nobody/an-anime-game-launcher/releases) page

### For arch users

This launcher also available as the [an-anime-game-launcher](https://aur.archlinux.org/packages/an-anime-game-launcher) AUR repository

<br>

### ⚠️ Please, be careful with 2.3.0 game's release. It may suddenly break game's patch state

### ⚠️ Also patch's repository will be changed and you will HAVE to update launcher to the newest version

<br>

## Usage statistics

<img src="repository-pics/statistics.png">

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

# Additional requirements

| Name | Description |
| --- | --- |
| [MangoHud](https://github.com/flightlessmango/MangoHud) | To use MangoHud |
| [vkBasalt](https://github.com/DadSchoorse/vkBasalt) | To use shaders |

These requirements can't be easily installed so you should do it manually

They're required only for some specific functions

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
* <s>Color variants for progress bar's downloading text dependent on the background picture primary color</s> *(1.4.0)*
* <s>Playing statistics</s> *(1.4.1)*
* <s>MangoHud support</s> (added HUD selector) *(1.5.0)*
* <s>Add vkBasalt support and "shaders library"</s> *(1.5.0)*
  - [yagocl's](https://notabug.org/Krock/GI-on-Linux/src/master/static/vkBasalt_yagocl.conf) basic sharpening preset *(without pictures)*
  - [notahuman's](https://notabug.org/Krock/GI-on-Linux/src/master/static/vkBasalt_notahuman.conf) prime preset v2
* Make shaders manager hidden if vkBasalt is not installed
* Make MangoHud option hidden if it is not installed
* Screenshots explorer
* Make force launch button when the launcher's repository is unavailable *(waiting for the repository's unavailability lmao)*
* Game's update pre-installation *(waiting for the pre-installation date to find out how it actually works)*
* Set default wine version to download so the wine install requirement is no longer needed
* Add Patch category in settings menu with
  - Always participate in patches testing
  - Applying anti login crash patch
  - Remove patch
* DXVK logs auto-deletion option

And don't forget to change the patch's URI when it will be changed