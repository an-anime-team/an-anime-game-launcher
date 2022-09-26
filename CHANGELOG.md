# 1.1.1

- updated core library and components library
- added (likely working) updates pre-downloading functionality
- moved to `anyhow::Result` in lots of places
- added partial support of wincompatlib
- moved to libadwaita 1.2

# 1.1.0

- added support for FPS unlocker
- added support for system wine during initial setup
- added usage of xdg-portals for folders chooser during initial setup
- fixed DXVK applying using system wine

# 1.0.4

- added creation of wine/dxvk folders at start if needed
- fixed ability to use system wine to run the game
- updated components (wine/dxvk) system
- reworked DXVKs UI components to support different builds
- fixed thread issues when calling `MainApp::update_state`
- updated core library; now launcher will continue downloading
  of partially downloaded files
- added downloading speed limiter (`config.json` -> `launcher.speed_limit`)
- added `Config::try_get_selected_dxvk_info` method;
  now launcher loads currently applied dxvk version from the wine prefix files
- added initial updates pre-downloading support (from 1.0.3 core)
- removed patch-related buttons
- changed FSR description

# 1.0.3

- fixed work with `patch` folder
- reworked components downloading during initial setup
  now lots of stuff happens in separate threads and launcher shouldn't freeze
  also fixed sometimes weird progress bar behavior

# 1.0.2

- updated core library with lots of fixes
- added new wine versions
- fixed Polish `LANG` value
- made FSync default wine sync option

# 1.0.1

- updated core library with fixed Chinese game's data folder name
- added icon loading from "icon" file, added `--run-game` argument
- fixed gamescope switcher's state loading from config
- fixed FSR switch in gamescope settings

# 1.0.0

- added new wine versions
- changed preferences icon, updated main window's menu
- removed open launcher folder button from settings
- added fix for dxvk applying with spaces to the runners folder path
- updated default patch repo's mirror
- updated core library; added support for Chinese version
- added default game edition prediction based on system locale
- added ability to run the game when all patch servers are down
- used `std::process::Output` on DXVK applying instead of String
  this fixes errors related to UTF-8 decoding since different systems may have
  different default encodings
- added "borderless" and "virtual desktop" options to settings;
  added separate FSR option to gamescope settings
- dxvk now uses wine64 to update prefix before applying patches
- added setting of `-window-mode exclusive` args to the wine when using wine FSR
- `WinePrefix::update` and so now use wine64 binary instead of wineboot
- renamed `files.wine` to `files.wine64` in `wine.json`
- `Config::try_get_wine_executable` function now return wine64 binary
- added errors toasting for initial setup window

# 1.0.0-rc2

- updated core library;
  this fixes getting available space on systems with lots of disks
- added patch folder selection during initial setup
- fixed error panicking when you're closing folder selection dialogue
  during initial setup
- disabled mangohud if gamescope is enabled

Reworked work with config file
- now missing fields will be automatically filled;
  excess fields - removed.
  thanks to new code structure I can easily create
  new fields or rename old ones
- improved `WineLang` enum; now launcher loads languages list dynamically
  from this enum so I can easily add support for new languages

Initial setup changes
- renamed "page_*" to some actual pages names
- added voice packages selection page
- made "Advanced" button working. It shows default paths selection page

# 1.0.0-rc1

*(nothing changed)*

# Beta 0.7.1

- updated core library; new version caches patch fetching results
- added Nvidia Image Scaling option to gamescope
- added `dxvk-async-1.10.3`

# Beta 0.7.0

- added gamescope support
- now gamemode option will not be clickabke if gamemode is not installed
- reworked enhancements page's events
- changed winesync env variables
- added `lib::is_available` function to check packages availability

# Beta 0.6.3

- updated core library;
  added `lib::consts::TELEMETRY_CHECK_TIMEOUT` and `PATCH_FETCHING_TIMEOUT`
  to specify new core-required timeouts
- fixed error message toasting when failed to run the game
- added telemetry servers acessibility checking before running the game
- fixed setting game command default value in environment settings
- use `patch.root = false` by default in flatpak

# Beta 0.6.2

- updated core library;
  now launcher will check available free space
  before downloading anything
- added dxvk 1.10.3
- added `WINEARCH=win64` to some places
- added `devel` style to `FirstRunApp` if it's debug mode
- changed app's id
- added xdelta3 requirement during initial setup
- added wine & dxvk version selection during initial setup
- added "Open launcher folder" button to settings
- removed `glib::Downgrade` trait from all the `App`s' `Value`s
- removed `tasks` mod; removed `tokio` dependency;
  rewritten `OpenPreferencesPage` to work with threads instead of futures
- added `opt-level = 3` to release profile
- upscaled logo to 512x512; added source link
- bundled all the UI files into gtk resources

# Beta 0.6.1

- added ability to edit game running command

# Beta 0.6.0

- made working environment variables manager
- renamed `ToastError` trait to `Toast`;
  renamed its `toast_error` method to `toast`;
  now `toast` method will not display button if toast's message is empty
- updated `game::run` function, now it prints running command
  and supports `gamemoderun`
- added automatic `DXVK_ASYNC=1` setting for dxvk-async

# Beta 0.5.3

- added "Repair game" button
- added `repairer` field to settings file
- updated core library
- made preparations for environment settings
- now launcher hides when you launch the game
- now `Config::try_get_wine_executable` can return `Some("wine")`
- removed old wine and dxvk versions;
  added new Wine-GE-Proton and GE-Proton builds

# Beta 0.5.2

- updated core library
- now general settings page displays patch version with orange color
  if patch is not applied
- added working patch applying mechanism
- added `patch.root` config, and you can apply patch
  without using root privilegies

# Beta 0.5.1

- updated core library;
  now you can delete voice packages from settings

# Beta 0.5.0

- removed "Settings" main menu item because I said so
- made `VoiceoverRow` component;
  added dynamic voiceovers loading
  Now you can download new voiceovers, but can't delete them (WIP)
- probably fixed startup gtk errors

# Beta 0.4.0

- added "Settings" option for main window menu
- added working default paths selection in first run window
- added subfolders support for blueprint compiler;
  moved first_run and preferences pages to subfolders
- added 2 first run pages:
  + when you don't have some required components
  + to select default folders paths

# Beta 0.3.1

- added automatic downloading if you already clicked "download" button
  so e.g. you don't need to press "download" button two times to download the game
  and then download its voiceover
- reduced amount of action calls
- added "WIP" tooltips for progress pause buttons

# Beta 0.3.0

- added usage of config's temp path to all the installers
- added processing of `WineNotInstalled` and `PrefixNotExists` actions
- added (forgotten) launcher states updating
- fixed app title
- removed excess use statements to hide warning messages

# Beta 0.2.1

- removed excess code
- tested and fixed game downloading
- small first run UI changes, marked some wine version as not recommended

# Beta 0.2.0

- updated core library to 0.1.3
- added soda wine version
- added default patch servers for config file
- with core modifications now first run window finally works
- added new wine versions
- added automatic default folder creation
- added `latest` methods for DXVK/Wine versions
- added `wine_prefix` mod with `WinePrefix` struct to manage what do you think what
- spent lots of time trying to make the launcher
  download default wine version,
  create prefix and apply DXVK
  but it just pauses actions flow after
  ~400 KB of downloaded wine version progress

# Beta 0.1.1

- added more system data in about dialog
- updated core library

*(0.1.0 considered as alpha and not listed here)*
