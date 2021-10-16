<img src="repository-pics/logo.png">

<img src="repository-pics/launcher.png">

<br><br>

# Status

### Launcher is in the beta state; patch is in the testing phase

| Game version | Launcher version | Patch version |
| :---: | :---: | :---: |
| 2.2.0 | 2.2.0-beta2 | 2.2.0-testing |

Download from [Releases](https://notabug.org/nobody/an-anime-game-launcher/releases)

# Requirements

To work this launcher requires

* wine
* winecfg
* winetricks
* unzip

# Development

## Build from source

```sh
npm run build:linux
```

## Run from source

```sh
npm start
```

# Roadmap

To 2.2.0-release1

* <s>Fix AppImage builds</s>
* Parse background banners from the game's API ([#1](https://notabug.org/nobody/an-anime-game-launcher/issues/1))
* Make Proton-GE default compatibility tool and fix game input issues
* Add additional telemetry checking
* Add preferences menu
* Add launcher updates notifications

I've found a reason why do AppImages don't work, and I don't want to say it, but I have to rewrite half a project lmao

<br>

### Please, try to use "An Anime Game" phrase instead of the real game name to avoid search engines parsing