type ArchiveType =
    | 'tar'
    | 'zip';

type Size = {
    compressed?: number;
    uncompressed?: number;
};

type File = {
    path: string;
    size: Size;
};

type ArchiveInfo = {
    size: Size;
    type: ArchiveType;
    files: File[];
};

export type {
    ArchiveType,
    Size,
    File,
    ArchiveInfo
};
