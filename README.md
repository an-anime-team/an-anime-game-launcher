<picture>
    <source media="(prefers-color-scheme: dark)" srcset="repository/logo-dark.png">
    <img src="repository/logo-light.png">
</picture>

| Modern style | Classic style |
| :-: | :-: |
| <picture><source media="(prefers-color-scheme: dark)" srcset="repository/main-modern-dark.png"><img src="repository/main-modern.png"></picture> | <picture><source media="(prefers-color-scheme: dark)" srcset="repository/main-classic-dark.png"><img src="repository/main-classic.png"></picture> |
| <picture><source media="(prefers-color-scheme: dark)" srcset="repository/settings-modern-dark.png"><img src="repository/settings-modern.png"></picture> | <picture><source media="(prefers-color-scheme: dark)" srcset="repository/settings-classic-dark.png"><img src="repository/settings-classic.png"></picture> |

<p align="center">
    <a href="https://discord.gg/ck37X6UWBp">Discord</a> ·
    <a href="https://matrix.to/#/#an-anime-game:envs.net">Matrix</a> ·
    <a href="https://github.com/an-anime-team/an-anime-game-launcher/wiki">Wiki</a>
</p>

<br>

# Please don't link to this repository

We understand you want to help people run An Anime Game on Linux. The issue is, the more people know
of the tools involved in it, the higher the chances The Anime Game Company will notice them, and then
fix the flaws that let us do the needful.

While sharing in your self-managed chat services and through private messaging is fine, we humbly ask
that you refrain from publically sharing links to this repository, or to any of the tools involved.

Or share them and ruin it for the dozens of Linux players. It's up to you.

## If you're going to share it anyway

Do what you must, but we would respectfully request that you try to label the game as "An Anime Game"
instead of the actual name of the game, to avoid search engine parsing.

# Documentation

Documentation for the launcher can be found in the [repository wiki](https://github.com/an-anime-team/an-anime-game-launcher/wiki).

# Download

Currently there are 5 options available:
1. Flatpak
2. AUR
3. RPM
4. Gentoo/ebuild
5. NixOS

To see the installation guides, [Please visit the wiki page here](https://github.com/an-anime-team/an-anime-game-launcher/wiki/Installation)

## Chinese version support

This should be automatically enabled if you're using zh_cn (Chinese) as your system language. If you're not using it - you'll need to set `China` as your `launcher.edition` in the `config.json` file

# Development

| Folder | Description |
| - | - |
| src | Rust source code |
| assets | App assets folder |
| assets/locales | App localizations |
| target/release | Release build of the app |

## Clone repo

```sh
git clone --recursive https://github.com/an-anime-team/an-anime-game-launcher
```

## Run app

```sh
cargo run
```

## Build app

```sh
cargo build --release
```

## Updates strategy

Starting from 3.2.1 ([fcab428](https://github.com/an-anime-team/an-anime-game-launcher/commit/fcab428cb40b1457f41e0856f9d1e1473acbe653)) we have 2 branches: stable ([main](https://github.com/an-anime-team/an-anime-game-launcher/tree/main)) and dev ([next](https://github.com/an-anime-team/an-anime-game-launcher/tree/next)). Code changes will be pushed into dev branch and merged into stable once they're ready for new version release

<img src="repository/branches.png" />
