type Runner = {
    /**
     * i.e. proton-ge
     */
     family: string;

    /**
     * i.e. Proton-6.20-GE-1
     */
    name: string;

    /**
     * i.e. Proton 6.20 GE 1
     */
    title: string;

    uri: string;

    files: {
        /**
         * i.e. bin/wine64
         */
        wine: string;

        /**
         * i.e. bin/wineserver
         */
        wineserver: string;

        /**
         * i.e. lib64/wine/x86_64-windows/winecfg.exe
         */
        winecfg: string;
    };

    recommended: boolean;
    installed: boolean;
};

type RunnersFamily = {
    title: string;
    runners: Runner[];
};

export type {
    Runner,
    RunnersFamily
};
