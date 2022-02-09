<img src="repository/pics/logo.jpg">

<img src="repository/pics/launcher-main.png">

<img src="repository/pics/launcher-settings.png">

<br><br>

### Please, try to use "An Anime Game" phrase instead of the real game name to avoid search engines parsing

<br><br>

# Download

| Source | Link |
| --- | --- |
| AppImage | [Releases](https://gitlab.com/KRypt0n_/an-anime-game-launcher/-/releases) page |
| AUR | [an-anime-game-launcher-bin](https://aur.archlinux.org/packages/an-anime-game-launcher-bin) |
| Pacstall | an-anime-game-launcher-bin [(?)](https://pacstall.dev) |

## Flatpak support

To install the launcher via flatpak you will first have to make sure that you have flathub's remote installed

```zsh
flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
```

After installing flathub's remote you install launcher.moe's remote

```zsh
flatpak remote-add --if-not-exists launcher.moe https://gol.launcher.moe/gol.launcher.moe.flatpakrepo
```

Now the only thing remaining is to install the launcher

```zsh
flatpak install launcher.moe com.gitlab.KRypt0n_.an-anime-game-launcher
```

Some additional configuration is possible when using the flatpak. For info about this, see [this page](https://gitlab.com/lane__/an-anime-game-launcher-flatpak/-/blob/master/README.md)

# Status

| Game version | Launcher version | Patch version |
| :---: | :---: | :---: |
| 2.4.0 | 2.1.3 | 2.4.0 stable ✅ |

We have our own [An Anime Game](https://discord.gg/ck37X6UWBp) discord server where you can ask any questions

## Usage statistics

This is our current usage statistics. You can find older ones [here](repository/pages/STATISTICS.md)

### Our discord server — 250 members

<img src="repository/pics/stats/discord.png">

<br>

# Requirements

To work this launcher requires

| Name | Description |
| --- | --- |
| webkit2gtk | To run Neutralino apps |
| libappindicator-gtk3 | To run Neutralino apps |
| unzip | To unpack zip archives (DXVKs / wines) |
| tar | To unpack tar archives (DXVKs / wines) |
| git | To check for new versions of the launcher |
| curl | To download archives with game, voice data, runners and so on |
| xdelta3 | To apply the patch to the game |
| cabextract | To install fonts to the wine prefix |
| libnotify | To send system notifications |

## Install

### apt-get

```sh
sudo apt-get install unzip tar git curl xdelta3 cabextract libnotify
```

### pacman

```sh
sudo pacman -Syu unzip tar git curl xdelta3 cabextract libnotify
```

### dnf

```sh
sudo dnf install unzip tar git curl xdelta cabextract libnotify
```

# Additional requirements

| Name | Description |
| --- | --- |
| [MangoHud](https://github.com/flightlessmango/MangoHud) | To use MangoHud |
| [vkBasalt](https://github.com/DadSchoorse/vkBasalt) | To use shaders |
| [GameMode](https://github.com/FeralInteractive/gamemode) | To use GameMode (performance optimization) |

These requirements can't be easily installed so you should do it manually

They're required only for some specific functions

# Development

## Download source

```sh
git clone https://gitlab.com/KRypt0n_/an-anime-game-launcher
cd an-anime-game-launcher
yarn
yarn neu update
```

## Run

```sh
yarn dev
```

## Build

```sh
yarn build
```

## Bundle to AppImage

```sh
yarn bundle
```

# Roadmap

This is our current roadmap goals. You can find older ones [here](repository/pages/ROADMAP.md)

## ➤ To 3.0.0 release

### Features

* <s>Use `LauncherLib.getGameVersion` function instead of the `config.json`'s `version` property</s> *(deprecated due to the new core functions)*
* <s>Fix button flickering at start when the launcher's state updates</s> *(fixed due to the new framework usage and proper state updating)*
* <s>Add downloading pause button</s> *(2.0.0 release)*
* <s>Move launcher on [Empathize](https://github.com/empathizejs/framework)</s> *(2.1.0)*
* <s>Make notifications translations</s> *(2.1.0)*
* <s>ToS Violation Window</s> *(2.1.0)*
* <s>Add `latest.log` file generation</s> *(2.1.2)*
* <s>Add an option to show terminal with the wine's log of the game</s> *(7375c743, released in 2.1.3)*
* Add Chinese game's version support (due to changes in the Krock's patch)
* Implement manual config flushing functionality from the Empathize's API
* Add analytics window
* Dark progress bar design
* Statistics window
* Changelog window
* Screenshots explorer
* Add Patch category in settings menu with
  - Always participate in patches testing
  - Applying anti login crash patch
  - Remove patch
