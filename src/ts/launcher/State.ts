import Game from '../Game';
import type Launcher from '../Launcher';
import Patch from '../Patch';

import type { LauncherState } from '../types/Launcher';

export default class State
{
    public launcher: Launcher;

    public launchButton: HTMLElement;

    protected _state: LauncherState = 'game-launch-available';

    protected events = {
        'game-launch-available': import('./states/Launch'),

        'game-installation-available': import('./states/Install'),
        'game-update-available': import('./states/Install'),

        'game-voice-update-required': import('./states/InstallVoice'),

        'test-patch-available': import('./states/ApplyPatch'),
        'patch-available': import('./states/ApplyPatch')
    };

    public constructor(launcher: Launcher)
    {
        this.launcher = launcher;

        this.launchButton = <HTMLElement>document.getElementById('launch');

        this.launchButton.onclick = () => {
            if (this.events[this._state])
                this.events[this._state].then((event) => event.default(this.launcher));
        };

        this.update();
    }

    /**
     * Get current launcher state
     */
    public get(): LauncherState
    {
        return this._state;
    }

    /**
     * Set launcher state
     */
    public set(state: LauncherState): void
    {
        this._state = state;

        this.launcher.progressBar!.hide();

        switch(state)
        {
            case 'game-launch-available':
                this.launchButton.textContent = 'Launch';

                break;

            case 'game-installation-available':
                this.launchButton.textContent = 'Install';

                break;

            case 'game-update-available':
            case 'game-voice-update-required':
                this.launchButton.textContent = 'Update';

                break;

            case 'patch-available':
                this.launchButton.textContent = 'Apply patch';

                break;

            case 'test-patch-available':
                // todo some warning message
                this.launchButton.textContent = 'Apply test patch';

                break;

            case 'patch-unavailable':
                // todo some warning message
                this.launchButton.textContent = 'Patch unavailable';

                break;
        }
    }

    /**
     * Update launcher state
     * 
     * @returns new launcher state
     * 
     * This state will be automatically applied to the launcher
     * so you don't need to do it manually
     */
    public update(): Promise<string>
    {
        return new Promise(async (resolve) => {
            let state: LauncherState;

            const gameCurrent = await Game.current;
            const gameLatest = (await Game.latest).version;
            const patch = await Patch.latest;

            console.log(patch);

            if (gameCurrent === null)
                state = 'game-installation-available';
            
            else if (gameCurrent != gameLatest)
                state = 'game-update-available';

            else if (!patch.applied)
            {
                state = patch.state == 'preparation' ?
                    'patch-unavailable' : (patch.state == 'testing' ?
                    'test-patch-available' : 'patch-available');
            }

            else state = 'game-launch-available';

            this.set(state);

            resolve(state);
        });
    }
};
