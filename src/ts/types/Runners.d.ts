type Runner = {
    /**
     * e.g. proton-ge
     */
    family: string;

    /**
     * e.g. Proton-6.20-GE-1
     */
    name: string;

    /**
     * e.g. Proton 6.20 GE 1
     */
    title: string;

    uri: string;

    files: {
        /**
         * e.g. bin/wine64
         */
        wine: string;

        /**
         * e.g. bin/wineserver
         */
        wineserver: string;

        /**
         * e.g. lib64/wine/x86_64-windows/winecfg.exe
         */
        winecfg: string;
    };

    recommended: boolean;
    installed: boolean;
};

type RunnerFamily = {
    title: string;
    runners: Runner[];
};

export type {
    Runner,
    RunnerFamily
};
