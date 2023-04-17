# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[unreleased]: https://github.com/an-anime-team/an-anime-game-launcher/compare/3.5.1...next
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
