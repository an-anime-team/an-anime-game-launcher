type Params = {
    id: string;
    details?: string;
    state?: string;

    icon?: {
        large?: string;
        small?: string;
    };

    time?: {
        start?: number;
        end?: number;
    };
};

export type { Params };
