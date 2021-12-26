import type Launcher from '../Launcher';

import type { LauncherState } from '../types/Launcher';

export default class State
{
    public launcher: Launcher;

    public launchButton: HTMLElement;

    protected _state: LauncherState = 'game-launch-available';

    protected events = {
        'game-launch-available': import('./states/Launch')
    };

    public constructor(launcher: Launcher)
    {
        this.launcher = launcher;

        this.launchButton = <HTMLElement>document.getElementById('launch');

        this.launchButton.onclick = () => {
            if (this.events[this._state])
                this.events[this._state].then((event) => event.default());
        };
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

        switch(state)
        {
            case 'game-launch-available':
                this.launcher.progressBar!.hide();

                this.launchButton.textContent = 'Launch';

                break;
        }
    }
};
