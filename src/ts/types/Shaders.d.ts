type Shader = {
    name: string;
    folder: string;
    author: string;
    uri?: string;
    images?: {
        file: string;
        captions?: string[];
    }[];
};

export type { Shader };
