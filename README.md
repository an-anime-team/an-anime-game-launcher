<img src="repository-pics/logo.jpg">

<img src="repository-pics/launcher-main.png">

<img src="repository-pics/launcher-settings.png">

<br><br>

### Please, try to use "An Anime Game" phrase instead of the real game name to avoid search engines parsing

<br><br>

# Status

| Game version | Launcher version | Patch version |
| :---: | :---: | :---: |
| 2.3.0 | 1.9.1 | 2.3.0 stable ✅ |

We have our own [An Anime Game](https://discord.gg/ck37X6UWBp) discord server where you can ask any questions

### AppImage

Download AppImage from [Releases](https://gitlab.com/KRypt0n_/an-anime-game-launcher/-/releases) page

### For arch users

This launcher is also available as the [an-anime-game-launcher](https://aur.archlinux.org/packages/an-anime-game-launcher) AUR repository

<br>

## Usage statistics

### 2.2.0 — 29 total

<img src="repository-pics/stats/2.2.0.png">

### 2.3.0 — 73 total

<img src="repository-pics/stats/2.3.0.png">

> You can suggest colors for your countries

<br>

# Requirements

To work this launcher requires

| Name | Description |
| --- | --- |
| unzip | To unpack zip archives (DXVKs / wines) |
| tar | To unpack tar archives (DXVKs / wines) |
| git | To check for new versions of the launcher |
| xdelta3 | To apply the patch to the game |
| cabextract | To install fonts to the wine prefix |

## Install

### apt-get

```sh
sudo apt-get install unzip tar git xdelta3 cabextract
```

### pacman

```sh
sudo pacman -Syu unzip tar git xdelta3 cabextract
```

# Additional requirements

| Name | Description |
| --- | --- |
| [MangoHud](https://github.com/flightlessmango/MangoHud) | To use MangoHud |
| [vkBasalt](https://github.com/DadSchoorse/vkBasalt) | To use shaders |
| [GameMode](https://github.com/FeralInteractive/gamemode) | To use GameMode (performance optimization) |
| [switcheroo-control](https://gitlab.freedesktop.org/hadess/switcheroo-control/) | To select the GPU launcher should use to run the game |

These requirements can't be easily installed so you should do it manually

They're required only for some specific functions

# Development

## Download

```sh
git clone https://gitlab.com/KRypt0n_/an-anime-game-launcher
cd an-anime-game-launcher
yarn
```

## Run

```sh
yarn dev
```

## Build

```sh
yarn build
```

# Roadmap

### ✓ <s>To 1.0.0 release</s>

* <s>Fix AppImage builds</s> *(0.3.0)*
* <s>Parse background banners from the game's API ([notabug issue #1](https://notabug.org/nobody/an-anime-game-launcher/issues/1), [notabug pull request #2](https://notabug.org/nobody/an-anime-game-launcher/pulls/2))</s> *(0.3.0)*
* <s>Update launcher logo</s> *(0.4.0)*
* <s>Cache launcher background picture ([notabug pull request #6](https://notabug.org/nobody/an-anime-game-launcher/pulls/6))</s> *(0.4.0)*
* <s>Make Proton-GE default compatibility tool and fix game input issues</s> (added runners manager) *(0.5.0)*
* <s>Add preferences menu</s> *(0.5.0)*
* <s>Add additional telemetry checking</s> *(0.6.0)*
* <s>Add DXVK downloading in settings</s> *(0.6.0)*
* <s>Make participation in the anonymous analytics request</s> *(0.8.0)*
* <s>Add launcher updates notifications</s> *(1.0.0-rc1)*
* <s>Make automatic patch state parsing</s> *(1.0.0-rc1)*

### ✓ <s>To 2.0.0 release</s>

* <s>Add runners environmental variables manager</s> *(1.1.0)*
* <s>Add outdated files deletion when new game's update releases</s> *(1.1.0)*
* <s>Add installed packages deletion</s> *(1.2.0)*
* <s>Add voice packs support</s> (Thank @Maroxy for the developments in the previous versions) *(1.3.0)*
* <s>Color variants for progress bar's downloading text dependent on the background picture primary color</s> *(1.4.0, LAB-based in 1.5.3)*
* <s>Playing statistics</s> *(1.4.1)*
* <s>MangoHud support</s> (added HUD selector) *(1.5.0)*
* <s>Add vkBasalt support and "shaders library"</s> *(1.5.0)*
  - [yagocl's](https://notabug.org/Krock/GI-on-Linux/src/master/static/vkBasalt_yagocl.conf) basic sharpening preset *(without pictures)*
  - [notahuman's](https://notabug.org/Krock/GI-on-Linux/src/master/static/vkBasalt_notahuman.conf) prime preset v2
* <s>Make shaders manager hidden if vkBasalt is not installed</s> *(1.5.4)*
* <s>Make MangoHud option hidden if it is not installed</s> *(1.5.4)*
* <s>GameMode integration ([notabug issue 28](https://notabug.org/nobody/an-anime-game-launcher/issues/28), [notabug pull request 30](https://notabug.org/nobody/an-anime-game-launcher/pulls/30))</s> *(1.5.8)*
* <s>Hybrid GPU integration ([notabug issue 29](https://notabug.org/nobody/an-anime-game-launcher/issues/29), [notabug pull request 33](https://notabug.org/nobody/an-anime-game-launcher/pulls/33))</s> *(1.6.0)*
* <s>Add winetricks auto-downloading when new prefix creates so it is no longer required</s> *(1.6.0)*
* <s>DXVK logs auto-deletion option</s> *(1.6.0)*
* <s>Add default wine version to download</s> (Proton-6.20-GE-1) *(1.6.0)*
* <s>Add dark theme support for settings menu</s> *(1.7.0)*
* <s>Add winetricks and winecfg buttons to settings ([notabug issue 35](https://notabug.org/nobody/an-anime-game-launcher/issues/35))</s> *(1.8.0)*
* <s>Wine prefix folder selection ([notabug issue 37](https://notabug.org/nobody/an-anime-game-launcher/issues/37))</s> *(1.8.0)*
* <s>Use auto-downloaded winetricks in settings menu</s> *(1.9.0, !14)*
* <s>Use `winecfg.exe` from the installed runner in settings menu</s> *(1.9.0, !14)*
* <s>Fix voice data installation</s> *(1.9.0, !15)*
* <s>Add fps unlocker option</s> *(1.9.0, !15)*

### ➤ To 3.0.0 release

* Move project to Neutralino
* Move project to Vue
* Add downloading pause button
* Use `LauncherLib.getGameVersion` function instead of the `config.json`'s `version` property
* Fix button flickering at start when the launcher's state updates
* Game's update pre-installation
* Screenshots explorer
* Add Patch category in settings menu with
  - Always participate in patches testing
  - Applying anti login crash patch
  - Remove patch
