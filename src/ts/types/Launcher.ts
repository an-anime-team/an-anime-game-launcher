type LauncherState =
    | 'patch-unavailable'
    | 'test-patch-available'
    | 'patch-applying'
    | 'game-update-available'
    | 'game-installation-available'
    | 'game-voice-update-required'
    | 'resume-download-available'
    | 'game-launch-available';

export type { LauncherState };