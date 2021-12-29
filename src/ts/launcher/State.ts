import type Launcher from '../Launcher';
import type { LauncherState } from '../types/Launcher';

import Window from '../neutralino/Window';

import Game from '../Game';
import Patch from '../Patch';
import Voice from '../Voice';

declare const Neutralino;

export default class State
{
    public launcher: Launcher;

    public launchButton: HTMLElement;
    public predownloadButton: HTMLElement;

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
        this.predownloadButton = <HTMLElement>document.getElementById('predownload');

        this.launchButton.onclick = () => {
            if (this.events[this._state])
            {
                this.launchButton.style['display'] = 'none';

                this.events[this._state].then((event) => {
                    event.default(this.launcher).then(() => {
                        this.update().then(() => {
                            this.launchButton.style['display'] = 'block';
                        });
                    });
                });
            }
        };

        this.predownloadButton.onclick = () => {
            this.launchButton.style['display'] = 'none';
            this.predownloadButton.style['display'] = 'none';

            const module = this._state === 'game-pre-installation-available' ?
                'Predownload' : 'PredownloadVoice';

            import(`./states/${module}`).then((module) => {
                module.default(this.launcher).then(() => {
                    this.update().then(() => {
                        this.launchButton.style['display'] = 'block';
                    });
                });
            });
        };

        this.update().then(() => {
            Neutralino.storage.setData('launcherLoaded', 'aboba');

            Window.current.show();
        });
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
        this.predownloadButton.style['display'] = 'none';

        switch(state)
        {
            case 'game-launch-available':
                this.launchButton.textContent = 'Launch';

                break;

            case 'game-pre-installation-available':
            case 'game-voice-pre-installation-available':
                this.launchButton.textContent = 'Launch';

                this.predownloadButton.style['display'] = 'block';

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
            const gameLatest = await Game.getLatestData();
            const patch = await Patch.latest;
            const voiceData = await Voice.current;

            if (gameCurrent === null)
                state = 'game-installation-available';
            
            else if (gameCurrent != gameLatest.game.latest.version)
                state = 'game-update-available';

            // TODO: update this thing if the user selected another voice language
            else if (voiceData.installed.length === 0)
                state = 'game-voice-update-required';

            else if (!patch.applied)
            {
                state = patch.state == 'preparation' ?
                    'patch-unavailable' : (patch.state == 'testing' ?
                    'test-patch-available' : 'patch-available');
            }

            else if (gameLatest.pre_download_game && !await Game.isUpdatePredownloaded())
                state = 'game-pre-installation-available';

            else if (gameLatest.pre_download_game && !await Voice.isUpdatePredownloaded(await Voice.selected))
                state = 'game-voice-pre-installation-available';

            else state = 'game-launch-available';

            this.set(state);

            resolve(state);
        });
    }
};
