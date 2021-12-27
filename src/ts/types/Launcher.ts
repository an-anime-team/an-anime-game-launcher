/**
 * With a first run the game will not be installed
 * and the launcher will have "game-installation-available" state
 * 
 * With it, launcher will create wine prefix if it is required,
 * download the game, voice data and unpack them
 * 
 * Then, with game's updates launcher will have "game-update-available" state
 * and with it it will download and unpack game and voice updates
 * 
 * When the game is installed and updated - then launcher will have either
 * "patch-unavailable", "test-patch-available", "patch-available", or "game-launch-available"
 * So it will either download and apply patch, launch the game or notify user that the patch is not available
 */

type LauncherState =
    | 'patch-unavailable'
    | 'test-patch-available'
    | 'patch-available'
    | 'game-installation-available'
    | 'game-update-available'
    | 'game-voice-update-required'
    | 'game-launch-available';

export type { LauncherState };