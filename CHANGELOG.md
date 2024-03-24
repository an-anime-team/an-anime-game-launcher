# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Bundle `applications-system-symbolic` icon to the app
- Added "force grab cursor" option to the gamescope settings
- Added Thai
- Added Ukrainian

### Changed

- Update wish url
- Updated dependencies
- Improved app args parsing
- Updated locales

### Fixed

- Fixed GtkSwitch UI state representation

## [3.9.4] - 29.12.2023

### Changed

- Replaced FPS unlocker by a custom one

## [3.9.3] - 23.12.2023

### Added

- Added `UpdatingPermissions` installation step
- Downloaders now will skip finished files and truncate them if needed

### Changed

- Increased voiceovers version prediction error
- Updated FPS Unlocker version which fixes new game version integration issue
- Updated Turkish
- Updated German
- Updated Chinese
- Updated Polish

## [3.9.2] - 13.11.2023

### Added

- Added Korean
- Added Dutch
- Added 4.2.0 voiceovers sizes
- Made free space checks resolve symlinks

### Changed

- Updated development libraries versions
- Updated Japanese

## [3.9.1] - 28.09.2023

### Added

- Added Vietnamese
- Added support for segmented zip archives (fixed initial game downloading)

### Changed

- Updated Chinese

## [3.9.0] - 20.08.2023

### Added

- Added feature to map wine drives
- Added `%launch_args%` magic word for game launching command.
  Now you can use `%bash_command% <script> %launch_args%` to run custom script
- Added 4.0.0 voiceovers sizes
- Added `--session <name>` flag to switch active session
- Added Portuguese
- Added Polish

### Fixed

- Fixed logo size in the first run window

### Changed

- Updated Turkish
- Updated Italian
- Updated Japanese
- Updated Swedish
- Improved files migration code. In the best case scenarios, it will work immediately now

### Removed

- Removed patch integration

## [3.8.0] - 02.08.2023

### Added

- Added new gamescope version compatibility
- Added "launcher behavior" option
- Added "kill game process" button when chosen behavior keeps launcher window open
- Bundled some icons into the app for consistency across the systems
- Added better panics handler
- Added Swedish

### Fixed

- Fixed predownload button sensitivity

### Changed

- Improved pre-downloads state checking
- Replaced translation functions by `tr!` macro
- Reworked app resources structure
- Improved game running status check (wasn't working properly with Chinese client)

## [3.7.6] - 15.07.2023

### Fixed

- Fixed game launching on installs without `launcher.bat` file

## [3.7.5] - 14.07.2023

### Added

- Added support to the new wishes url cache location

### Fixed

- Fixed telemetry disabling

### Changed

- Updated Italian
- Updated Hungarian
- Updated Japanese
- Updated Indonesian
- Updated Spanish
- Updated Turkish
- Updated Chinese
- Disabled patch applying for new installations

## [3.7.4] (hotfix) - 19.06.2023

### Fixed

- Fixed main patch applying

## [3.7.3] - 18.06.2023

### Added

- Added telemetry disabling state support
- Added Discord RPC icons updating

### Changed

- Replaced xlua patch by "disable mhypbase" option
- Returned back old `background` file path

## [3.7.2] - 14.06.2023

### Fixed

- Fixed check button style for newly made sessions
- Fixed repairer's NaN progress
- Fixed game session selection when current one is removed

### Changed

- Updated Spanish
- Updated Hungarian (fixed #194)

## [3.7.1] - 11.06.2023

### Added

- Added Discord RPC icon selection
- Added Japanese
- Added Hungarian

### Fixed

- Fixed progress bar style after running game repairer
- Fixed repair button functionality (#186)
- Fixed default launcher language selection at the first start
- Fixed some installer updates reporting (including "checking free space")

### Changed

- Reworked game sessions selection
- Updated Indonesian
- Updated French
- Made initial tasks async which has decreased startup time
- Updated fps unlocker to 2.1.1

### Removed

- Removed 3.7.0 workaround
- Removed patch mirror migration

## [3.7.0] - 24.05.2023

### Added

- Added Italian
- Added Indonesian
- Added dynamic main button icon switching
- Set button label as "Resume" when the diff is part downloaded
- Added options to use wine / gstreamer shared libraries from selected wine build.
  These options will configure `LD_LIBRARY_PATH` and `GST_PLUGIN_PATH` environment variables
- Added setting of `LC_ALL` in wine lang setting
- Added `LAUNCHER_REQUESTS_TIMEOUT` environment variable
- Added option to disable main patch applying

### Fixed

- Fixed session applying on each launcher start
- Fixed predownload button ui
- Fixed proton builds integration with sandbox
- Fixed compatibility between sessions manager and sandbox
- Fixed sandboxing of inexisting folders

### Changed

- Apply selected session before launching the game.
  This will properly save your game session when you switch between wine prefixes
- Redesigned main button
- Used `whatadistro` to identify recommended package manager in the first run window
- Moved a lot of settings to separate page
- Set fsr quality mode in enhancements settings instead of strength
- Updated fps unlocker data
- Made temporary workaround to the game API changes
- Increased default requests timeout to 8 seconds
- Updated minreq to support `http_proxy`-like variables
- Disabled xlua patch applying by default

### Removed

- Removed Futex2 wine sync option

## [3.6.0] - 06.05.2023

### Added

- Added rules approving dialog to the first run window
- Added game settings section
- Added game sessions manager
- Added `LAUNCHER_FOLDER` variable support.
  Using this you can specify root path where the launcher stores `config.json` and other files
- Added patch repository mirror

### Changed

- Improved launcher logo rendering quality
- Reworked entry rows in the settings

### Fixed

- Fixed wine tools running using proton builds
- Fixed sandboxed game running (sounds are broken for now)

## [3.5.2] - 17.04.2023

### Added

- Added arguments and symlinks editor to sandbox settings

### Fixed

- Fixed game running issue if you have spaces in paths

## [3.5.1] (hotfix) - 16.04.2023

### Fixed

- Fixed telemetry checking

## [3.5.0] - 16.04.2023

### Added

- Added game sandboxing feature
- Added debugger to wine tools

### Changed

- Removed fractions displaying in components downloading progress bar
- Moved to upgraded launcher SDK

## [3.4.1] - 12.04.2023

### Fixed

- Fixed base game's hdiff patches applying errors caused by 3.6's voiceovers files migration
- Fixed xlua patch applying

### Changed

- Removed fractions displaying in repairer's progress bar

## [3.4.0] - 11.04.2023

### Added

- Added installation migration feature
- Added game environment switcher
- Added game edition switcher
- Added changelog to updated components toast
- Added wine tools to settings
- Added preferences search
- Added new progress bar statuses for applyign hdiff patches and removing outdated files
- Added automatic 3.5 -> 3.6 voiceover files migration related to changed files structure

### Fixed

- Added whitespaces removing from environment values

### Changed

- Improved game repairing feature
- Replaced `curl` dependency by native code
- Replaced static image by spinner in wine / dxvk version selection
- Made wine / dxvk versions always visible if they're downloaded

## [3.3.0] - 24.03.2023

### Added

- Added option to use additional xlua patch
- Added menu option to open wishes history url

### Fixed

- Fixed downloaded wine version selection on "download wine" button
- Fixed game downloading (it wasn't working since some version????)
- Fixed infinite retries to download some update or patch the game if it failed

### Removed

- Removed `launcher.speed_limit` config

## [3.2.1] - 18.03.2023

### Fixed

- Fixed DXVK applying on changed default path during initial setup
- Disabled long swipes and wheel scrolling for carousel in initial setup window

## [3.2.0] - 18.03.2023

### Added

- Added "components index updated" toast
- Added wine / dxvk downloading skipping in initial setup if you already downloaded them
- Added fps unlocker path chooser in initial setup

### Fixed

- Fixed main button sensitivity after prefix creation
- Fixed components related error on changing default path
- Fixed "wine not installed" button work

### Changed

- Changed `opt-level` to `s` (optimize for size). Decreased build size by 2 MB
- Changed default game and prefix paths
- Improved Proton-builds compatibility

## [3.1.5] - 07.03.2023

### Added

- Added support for `features` entry in wine/dxvk components index
- Added caching to `ComponentsLoader` methods
- Added Turkish

## [3.1.4] - 07.03.2023

### Changed

- Removed commit hash from core/SDK versions in about window

## [3.1.3] - 07.03.2023

### Added

- Added Spanish translations

### Fixed

- Fixed compatibility with GE-Proton

### Changed

- Improved components downloading in initial setup 

## [3.1.2] - 07.03.2023

### Added

- Added status page to the initial setup window

### Fixed

- Fixed initial setup window panic
- Fixed components and game downloading

## [3.1.1] - 06.03.2023

### Added

- Added components index path selection to initial setup

### Fixed

- Fixed game launching command

## [3.1.0] - 06.03.2023

### Added

- Added game repairing function
- Added french translations (#81)
- Used `open` library for xdg (#84)
- Added dynamic components loading
- Added commit hashes to core/sdk versions in about window
- Added translation for Simplified Chinese (#88)

### Fixed

- Forced `format_lang` to return regions for language codes
- Fixed titlebar behavior on some DEs
- Fixed wine `LANG` values
- Updated launcher URLs in about window

### Changed

- Changed default language from en to en-us
- Moved integer scaling, fsr and nis into separate group in gamescope settings

## [3.0.1] - 04.03.2023

### Fixed

- Fixed background picture updating on "update background" switch disabled

### Changed

- Updated core library & sdk, changed game running mechanism
- Temporary hidden repair game button

## [3.0.0] - 04.03.2023

🚀 Initial release

<br>

[unreleased]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.9.4...next
[3.9.4]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.9.3...3.9.4
[3.9.3]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.9.2...3.9.3
[3.9.2]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.9.1...3.9.2
[3.9.1]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.9.0...3.9.1
[3.9.0]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.8.0...3.9.0
[3.8.0]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.7.6...3.8.0
[3.7.6]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.7.5...3.7.6
[3.7.5]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.7.4...3.7.5
[3.7.4]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.7.3...3.7.4
[3.7.3]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.7.2...3.7.3
[3.7.2]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.7.1...3.7.2
[3.7.1]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.7.0...3.7.1
[3.7.0]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.6.0...3.7.0
[3.6.0]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.5.2...3.6.0
[3.5.2]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.5.1...3.5.2
[3.5.1]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.5.0...3.5.1
[3.5.0]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.4.1...3.5.0
[3.4.1]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.4.0...3.4.1
[3.4.0]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.3.0...3.4.0
[3.3.0]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.2.1...3.3.0
[3.2.1]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.2.0...3.2.1
[3.2.0]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.1.5...3.2.0
[3.1.5]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.1.4...3.1.5
[3.1.4]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.1.3...3.1.4
[3.1.3]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.1.2...3.1.3
[3.1.2]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.1.1...3.1.2
[3.1.1]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.1.0...3.1.1
[3.1.0]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.0.1...3.1.0
[3.0.1]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.0.0...3.0.1
[3.0.0]: https://github.com/an-anime-team/an-anime-game-launcher/releases/tag/3.0.0
