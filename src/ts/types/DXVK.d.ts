type DXVK = {
    title: string;
    version: string;
    uri: string;
    recommended: boolean;
    installed: boolean;
};

type DXVKTable = {
    title: string;
    versions: DXVK[];
};

export type { DXVKTable, DXVK };
