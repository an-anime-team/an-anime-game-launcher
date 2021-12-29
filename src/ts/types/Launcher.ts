/**
 * When first run the game will not be installed
 * and the launcher will have the "game-installation-available" state
 * 
 * With that state the launcher will create the wine prefix if it is required then download the game, voice data and unpack them.
 * 
 * If the game has an update available the launcher will have the "game-update-available" state
 * which causes the launcher to download the update and apply it.
 * 
 * When the game is installed and updated then the launcher will have either
 * "patch-unavailable", "test-patch-available", "patch-available", or "game-launch-available"
 * So it will either download and apply the patch, launch the game or notify user that the patch is not available.
 */

type LauncherState =
    | 'runner-installation-required'
    | 'dxvk-installation-required'
    | 'patch-unavailable'
    | 'test-patch-available'
    | 'patch-available'
    | 'game-installation-available'
    | 'game-update-available'
    | 'game-voice-update-required'
    | 'game-pre-installation-available'
    | 'game-voice-pre-installation-available'
    | 'game-launch-available';

export type { LauncherState };