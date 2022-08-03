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
