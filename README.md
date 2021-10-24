<img src="repository-pics/logo.jpg">

<img src="repository-pics/launcher-main.png">

<img src="repository-pics/launcher-settings.png">

<br><br>

# Status

| Game version | Launcher version | Patch version |
| :---: | :---: | :---: |
| 2.2.0 | 0.7.0 ⚠️ | 2.2.0 stable ✅ |

Download from [Releases](https://notabug.org/nobody/an-anime-game-launcher/releases)

> ⚠️ Launcher is currently in development, but you already can use it

# Requirements

To work this launcher requires

* wine
* winecfg
* winetricks
* unzip
* tar

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

To 1.0.0 release

* <s>Fix AppImage builds</s> *(0.3.0)*
* <s>Parse background banners from the game's API ([issue #1](https://notabug.org/nobody/an-anime-game-launcher/issues/1), [pull request #2](https://notabug.org/nobody/an-anime-game-launcher/pulls/2))</s> *(0.3.0)*
* <s>Update launcher logo</s> *(0.4.0)*
* <s>Cache launcher background picture ([pull request #6](https://notabug.org/nobody/an-anime-game-launcher/pulls/6))</s> *(0.4.0)*
* <s>Make Proton-GE default compatibility tool and fix game input issues</s> (added runners manager) *(0.5.0)*
* <s>Add preferences menu</s> *(0.5.0)*
* <s>Add additional telemetry checking</s> *(0.6.0)*
* <s>Add DXVK downloading in settings</s> *(0.6.0)*
* Add launcher updates notifications
* Make automatical patch state parsing
* Make participation in the anonymous analytics request

To 2.0.0 release

* Add outdated files deletion when new game's update releases
* Add voice packs support
* Add Patch category in settings menu with
  - Always participate in patches testing
  - Applying anti login crash patch
  - Remove patch
* Playing statistics

And don't forget to change the patch's URI when it will be changed

<br>

### Please, try to use "An Anime Game" phrase instead of the real game name to avoid search engines parsing