# An Anime Game Launcher GTK

The launcher variant written on Rust, GTK4 and libadwaita, using [Anime Game Core](https://gitlab.com/an-anime-team/anime-game-core) library

<img src="repository/pictures/main.png">
<img src="repository/pictures/settings.png">

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

Important to run it from the `scripts/appimage` folder

```
cd scripts/appimage
./build_appimage.sh
```
