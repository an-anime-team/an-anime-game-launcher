# An Anime Game Launcher GTK

The launcher variant written on Rust, GTK4 and libadwaita, using [Anime Game Core](https://gitlab.com/an-anime-team/anime-game-core) library

<img src="https://gitlab.com/an-anime-team/an-anime-game-launcher/-/raw/main/repository/pics/logo.jpg">
<img src="repository/pictures/main.png">
<img src="repository/pictures/settings.png">

<br>

<p align="center">You could also try <a href="https://gitlab.com/an-anime-team/an-anime-game-launcher">the main branch</a></p>

<br>

## Documentation

I wrote small documentation [here](https://gitlab.com/an-anime-team/alternatives/an-anime-game-launcher-gtk/-/wikis/home). It may contain some useful information

## Chinese version support

This should be automatically enabled if you're using zh_cn (Chinese) as your system language. If you're not using it - you'll need to set `China` as your `launcher.edition` in the `config.json` file

## Development

| Folder | Description |
| - | - |
| ui | Blueprint UI files |
| ui/.dist | UI files compiled by the blueprint |
| src | Rust source code |
| target/release | Release build of the app |
| blueprint-compiler | Blueprint compiler |
| anime-game-core | Anime Game Core library |

### Clone repo

```sh
git clone --recursive https://gitlab.com/an-anime-team/an-anime-game-launcher-gtk
```

### Run app

```sh
cargo run
```

### Build app

```sh
cargo build --release
```

### Building AppImage

```
./scripts/build.sh
```
