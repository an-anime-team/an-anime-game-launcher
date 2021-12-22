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
     * If the main UnityPlayer patch applied
     */
    player: boolean;

    /**
     * If the anti-login crash patch applied
     */
    xlua: boolean;
};

export type {
    PatchState,
    PatchInfo
};
