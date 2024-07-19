| Modern style | Classic style |
| :-: | :-: |
| <picture><source media="(prefers-color-scheme: dark)" srcset="repository/main-modern-dark.png"><img src="repository/main-modern.png"></picture> | <picture><source media="(prefers-color-scheme: dark)" srcset="repository/main-classic-dark.png"><img src="repository/main-classic.png"></picture> |
| <picture><source media="(prefers-color-scheme: dark)" srcset="repository/settings-modern-dark.png"><img src="repository/settings-modern.png"></picture> | <picture><source media="(prefers-color-scheme: dark)" srcset="repository/settings-classic-dark.png"><img src="repository/settings-classic.png"></picture> |

<p align="center">
    <a href="https://discord.gg/ck37X6UWBp">Discord</a> ·
    <a href="https://github.com/an-anime-team/an-anime-game-launcher/wiki">Wiki</a>
</p>

<br>

# 🚧 Project status

Due to lack of interest from my side the project stays in a legacy, maintaining-only state for a long period of time. This project will not receive huge updates unless really necessary. I still keep it up to date with latest changes in the game and work with community to solve the issues, but old-known unessential bugs will not be fixed, and new features will not be added. Instead, I'm working on other projects, and the future is in uniting all the launchers in one single [universal launcher](https://github.com/an-anime-team/anime-games-launcher). This project stays in "proof of concept" stage right now and requires major changes, which, again, require interest from my side. Keep your eye on our discord server for more details.

<br>

# ♥️ Useful links and thanks

* [macOS launcher](https://github.com/3Shain/yet-another-anime-game-launcher) which contains some additional compatibility components
* [Wiki](https://github.com/an-anime-team/an-anime-game-launcher/wiki) contains some basic FAQ, installation instructions and more
* [Releases page](https://github.com/an-anime-team/an-anime-game-launcher/releases) where you can find latest available version
* [Changelog](CHANGELOG.md) with chronology of the project

All the project's life happen in our discord server. If you have any questions or want to report an issue - please contact the dev directly there.

<br>

# ⬇️ Download

Launcher developer does not provide any packages for this programm. Instead, we almost fully rely on other people to maintain them.

To see the installation guides, please visit [this wiki page](https://github.com/an-anime-team/an-anime-game-launcher/wiki/Installation).

Instructions may be outdated due to lack of interest in maintaining them. You can help the project by keeping documentation up to date if you're interested in it.

## 😀 Official support

These packages are officially supported by the An Anime Team, and we try to ensure that they work for everyone.

| Format | Wiki | Source | Distributions | Maintainer |
| - | - | - | - | - |
| Flatpak | [wiki](https://github.com/an-anime-team/an-anime-game-launcher/wiki/Installation#-any-distribution-flatpak) | [flatpak-builds](https://github.com/an-anime-team/flatpak-builds) | Any (Fedora, Pop!_OS, etc.) | Luna (available in discord) |
| RPM | [wiki](https://github.com/an-anime-team/an-anime-game-launcher/wiki/Installation#-fedora-rpm) | [AAGL](https://build.opensuse.org/repositories/home:Maroxy:AAT-Apps/AAGL) * | Fedora, OpenSUSE | Maroxy (second discord admin) |

> [!NOTE]
> RPM packages are often really outdated. It's not recommended to use them.

## 🙂 Community support

These packages are supported by active members of our community. They're widely used and we keep some level of interactions with their maintainers.

| Format | Wiki | Source | Distributions | Maintainer |
| - | - | - | - | - |
| AUR | [wiki](https://github.com/an-anime-team/an-anime-game-launcher/wiki/Installation#-arch-linux-aur) | [an-anime-game-launcher-bin](https://aur.archlinux.org/packages/an-anime-game-launcher-bin) | Arch Linux, Manjaro, EndeavourOS | xstra * |
| NixOS module | [wiki](https://github.com/an-anime-team/an-anime-game-launcher/wiki/Installation#-nixos-nixpkg) | [aagl-gtk-on-nix](https://github.com/ezKEa/aagl-gtk-on-nix) | NixOS | Luxxy * |

> [!NOTE]
> Honorary members of our discord server. We have direct contact with them.

## 😑 Third party support

These packages are supported by third party distributors. They either did not contact us, or contact exceptionally rarely. We do not verify state of these packages, and we are not related to their state at all.

| Format | Source | Distributions |
| - | - | - |
| DEB | [an-anime-game-launcher](https://launchpad.net/~thundergemios10/+archive/ubuntu/an-anime-game-launcher) | Ubuntu, Linux Mint, Pop!_OS |
| Pacstall | [an-anime-game-launcher-bin](https://pacstall.dev/packages/an-anime-game-launcher-bin) | Ubuntu |
| Ebuild | [aagl-ebuilds](https://github.com/an-anime-team/gentoo-ebuilds) * | Gentoo |
| Lutris | `lutris.net/games/gen...-imp...` (stripping the link) | Any |

> [!NOTE]
> Although it's hosted in our official repo we didn't contact with its maintainer for some time already, and recent updates were made via merge requests by the community.

## Chinese version support

This should be automatically enabled if you're using `zh_cn` (Chinese) as your system language. If you're not using it - you can change the game edition in the launcher settings.

The main problem, though, is that github is blocked in China, and it's used in other parts of the launcher - not just in game edition. Notably, you can't use the same components index as other people do.

To fix this, you have to make your own copy of the [components](https://github.com/an-anime-team/components) repository and change all the links there from github releases to some mirror. Later you can update the components index repo link in your launcher's `config.json` file.

If you have any questions - feel free to contact the dev in our discord server (or if you have no way to use discord - try sending me an email, but it's unlikely to be received).
