# Roadmap

This file is a roadmap archive. You can see here which journey we made to make the launcher looks like it looks

And that's not a final! Our future goals you can find in [readme](../../README.md)

## ✓ To 1.0.0 release

* <s>Fix AppImage builds</s> *(0.3.0)*
* <s>Parse background banners from the game's API ([notabug issue #1](https://notabug.org/nobody/an-anime-game-launcher/issues/1), [notabug pull request #2](https://notabug.org/nobody/an-anime-game-launcher/pulls/2))</s> *(0.3.0)*
* <s>Update launcher logo</s> *(0.4.0)*
* <s>Cache launcher background picture ([notabug pull request #6](https://notabug.org/nobody/an-anime-game-launcher/pulls/6))</s> *(0.4.0)*
* <s>Make Proton-GE default compatibility tool and fix game input issues</s> (added runners manager) *(0.5.0)*
* <s>Add preferences menu</s> *(0.5.0)*
* <s>Add additional telemetry checking</s> *(0.6.0)*
* <s>Add DXVK downloading in settings</s> *(0.6.0)*
* <s>Make participation in the anonymous analytics request</s> *(0.8.0)*
* <s>Add launcher updates notifications</s> *(1.0.0-rc1)*
* <s>Make automatic patch state parsing</s> *(1.0.0-rc1)*

## ✓ To 2.0.0 release

* <s>Add runners environmental variables manager</s> *(1.1.0)*
* <s>Add outdated files deletion when new game's update releases</s> *(1.1.0)*
* <s>Add installed packages deletion</s> *(1.2.0)*
* <s>Add voice packs support</s> (Thank @Maroxy for the developments in the previous versions) *(1.3.0)*
* <s>Color variants for progress bar's downloading text dependent on the background picture primary color</s> *(1.4.0, LAB-based in 1.5.3)*
* <s>Playing statistics</s> *(1.4.1)*
* <s>MangoHud support</s> (added HUD selector) *(1.5.0)*
* <s>Add vkBasalt support and "shaders library"</s> *(1.5.0)*
  - [yagocl's](https://notabug.org/Krock/GI-on-Linux/src/master/static/vkBasalt_yagocl.conf) basic sharpening preset *(without pictures)*
  - [notahuman's](https://notabug.org/Krock/GI-on-Linux/src/master/static/vkBasalt_notahuman.conf) prime preset v2
* <s>Make shaders manager hidden if vkBasalt is not installed</s> *(1.5.4)*
* <s>Make MangoHud option hidden if it is not installed</s> *(1.5.4)*
* <s>GameMode integration ([notabug issue 28](https://notabug.org/nobody/an-anime-game-launcher/issues/28), [notabug pull request 30](https://notabug.org/nobody/an-anime-game-launcher/pulls/30))</s> *(1.5.8)*
* <s>Hybrid GPU integration ([notabug issue 29](https://notabug.org/nobody/an-anime-game-launcher/issues/29), [notabug pull request 33](https://notabug.org/nobody/an-anime-game-launcher/pulls/33))</s> *(1.6.0)*
* <s>Add winetricks auto-downloading when new prefix creates so it is no longer required</s> *(1.6.0)*
* <s>DXVK logs auto-deletion option</s> *(1.6.0)*
* <s>Add default wine version to download</s> (Proton-6.20-GE-1) *(1.6.0)*
* <s>Add dark theme support for settings menu</s> *(1.7.0)*
* <s>Add winetricks and winecfg buttons to settings ([notabug issue 35](https://notabug.org/nobody/an-anime-game-launcher/issues/35))</s> *(1.8.0)*
* <s>Wine prefix folder selection ([notabug issue 37](https://notabug.org/nobody/an-anime-game-launcher/issues/37))</s> *(1.8.0)*
* <s>Use auto-downloaded winetricks in settings menu</s> *(1.9.0, !14)*
* <s>Use `winecfg.exe` from the installed runner in settings menu</s> *(1.9.0, !14)*
* <s>Fix voice data installation</s> *(1.9.0, !15)*
* <s>Add fps unlocker option</s> *(1.9.0, !15)*

## ✓ 2.0.0 release goals - moving launcher to Neutralino

#### Core functionality

* <s>Make `constants` class to store launcher's constants</s>
* <s>Make `Downloader` class to download files</s>
* <s>Make `Archive` class to work with archives</s>
* <s>Make `DXVK` class to manage DXVK installations</s>
* <s>Make `Runners` class to manage wines installations</s>
* <s>Make `Configs` class to manage launcher's configs</s>
* <s>Make `Game` class to control game-related features</s>
  * <s>Ability to parse current installed version</s>
  * <s>Ability to get latest available version</s>
  * <s>Ability to download and install updates</s>
* <s>Make `Voice` class to control voice packages-related features</s>
  * <s>Ability to parse current installed voice packs and get selected one</s>
  * <s>Ability to get latest available voice packs</s>
  * <s>Ability to download and install updates</s>
* <s>Make `Patch` class to control patch-related features</s>
  * <s>Ability to get current installed patch</s>
  * <s>Ability to get latest available patch</s>
  * <s>Ability to download and install it</s>
* <s>Add project binaries bundling</s>
  * <s>AppImage</s>

#### Launcher functions

* <s>Make `Launcher` class to manage launcher-related features</s>
  * <s>Downloading progress</s>
  * <s>Launcher state functionality</s>
    * <s>Game launch available</s>
    * <s>Game update (installation) required</s>
    * <s>Voice data update (installation) required</s>
    * <s>Patch unavailable</s>
    * <s>Test patch available</s>
* <s>Make Svelte components</s>
  * <s>Checkbox</s>
  * <s>Selectbox</s>
  * <s>SelectionList</s>
  * <s>PropertiesEditor</s>
* <s>Add `svelte-i18n`</s>
* <s>Telemetry checking</s>
* <s>Tooltips for some options</s>
* <s>Debugger</s>
* <s>Splash screen</s>
* <s>Theming system</s>
* <s>Game pre-installation</s>
* <s>Default runner and DXVK auto-installation</s>
* <s>Discord RPC settings</s>
* <s>Proper wine process monitoring</s>
* <s>Ability to hide some runners families</s>
* <s>Ability to change the temp directory where the launcher should download some files</s>
* <s>Shaders menu</s>
* <s>Launcher auto-updates</s> *(made updates notifications)*
