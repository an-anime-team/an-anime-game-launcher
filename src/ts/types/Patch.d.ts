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
};

export type {
    PatchState,
    PatchInfo
};
