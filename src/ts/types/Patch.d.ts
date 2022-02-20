type PatchState =
    | 'preparation'
    | 'testing'
    | 'stable';

type PatchInfo = {
    /**
     * Applied patch version
     */
    version: string;
    state: PatchState;

    /**
     * If the patch was applied
     */
    applied: boolean;

    /**
     * Source where this info was got from
     */
    source?: 'origin' | 'additional';

    /**
     * Used game server
     */
    server: 'global' | 'cn';
};

export type {
    PatchState,
    PatchInfo
};
