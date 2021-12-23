type ArchiveType =
    | 'tar'
    | 'zip'
    | null;

type Size = {
    compressed?: number | null;
    uncompressed?: number | null;
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
